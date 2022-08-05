
    use std::{ffi::OsString, fs};
    use rand::{prelude::*};


    use timeseries_database::{collection::Collection, system::System, tspoint::{TsPointData, TsPoint}, MAX_LINE_BLOC, schemaurl::SchemaURL, BATCH_SIZE, helpers::Helpers};
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    #[test]
    fn query_string_test() {

    // Clean the collection TEST if exists and create a new one
    match Collection::create(&OsString::from("TEST"), &OsString::from("TEST")) {
        Some(_)=> {}
        None=> {
            fs::remove_dir_all(SchemaURL::get_source_path(&OsString::from("TEST"))).unwrap();
            Collection::create(&OsString::from("TEST"), &OsString::from("TEST"));
        }
    };

    // Instanciate the system and get the Collection
    let mut sys = System::instanciate();
    let colec : &mut Collection;
    colec = sys.sources.get_mut(&OsString::from("TEST")).unwrap().colecs.get_mut(&OsString::from("TEST")).unwrap();
    let mut inserted_count = 0;
    let n_blocs = 5;
    let remainder = 15;



    // Generate random data following the different insert rules
    let gen_data = (1..(MAX_LINE_BLOC*n_blocs + remainder + 1)).into_iter().map(|x| TsPoint { 
    t: x*60,
    data : TsPointData { 
    c:Some(Decimal::from_u64(x*60).unwrap()),
    h:Some(Decimal::from_u64(x*60).unwrap()),
    l:Some(Decimal::from_u64(x*60).unwrap()),
    o:Some(Decimal::from_u64(x*60).unwrap())
    }} ).collect::<Vec<TsPoint>>();



    // Append generated point to database
    colec.append_points(&gen_data, &mut false, &mut inserted_count).unwrap();
    // Refresh the FileManager from the system
    sys.files.refresh_db_repo();

    assert_eq!((gen_data.len() as u32)==inserted_count.to_owned(),true, "Length test for generated and written data");

    let sys : &System = &System::instanciate();
    //Query string data

    let mut rng = rand::thread_rng();
    rng.gen_range(1..(MAX_LINE_BLOC*n_blocs + remainder + 1)/2);


    let bot_ts =  rng.gen_range(1..(MAX_LINE_BLOC*n_blocs + remainder + 1)/2);
    let top_ts = bot_ts + rng.gen_range(1..(12*BATCH_SIZE));
    let ir = sys.parseQueryString(&format!("ts TEST:TEST::{}:{}", bot_ts, top_ts));
    let res_data_ref = &colec.map.get_data(&ir.start_batchid, &(ir.end_batchid-ir.start_batchid))[(ir.start_ind as usize)..(ir.end_ind as usize)];

    let res_data =res_data_ref.to_owned();
    
    let awaited_data =gen_data
    .iter()
    .filter(|x| x.t>=Helpers::u64closest_down_divider(&bot_ts, &colec.map.step) && x.t<=Helpers::u64closest_up_divider(&top_ts, &colec.map.step))
    .map(|x| x).collect::<Vec<&TsPoint>>();
    assert_eq!(awaited_data.iter().zip(res_data).position(|(x,y)| *x!=y), None, "Testing equality between queried by ts data and stored data");

    let random_batch =  rng.gen_range(0..(colec.map.maxts-colec.map.mints)/colec.map.mints/BATCH_SIZE)+1;


    let ir = sys.parseQueryString(&format!("batch TEST:TEST::{}", random_batch));
    let res_data = &colec.map.get_data(&ir.start_batchid, &(ir.end_batchid-ir.start_batchid))[(ir.start_ind as usize)..(ir.end_ind as usize)];

    let awaited_data =&colec.map.get_data(&ir.start_batchid, &1);
    assert_eq!(awaited_data.iter().zip(res_data).position(|(x,y)| x!=y), None, "Testing equality between queried by batch data and stored data");






    }
