
    use std::{ffi::OsString, fs, cmp::{min, max}};
    use rand::{prelude::*};


    use timeseries_database::{collection::Collection, system::System, tspoint::{TsPointData, TsPoint}, MAX_LINE_BLOC, schemaurl::SchemaURL, BATCH_SIZE, helpers::Helpers, DEFAULT_STEP, source::Source};
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
    let mut source : &mut Source = sys.sources.get_mut(&OsString::from("TEST")).unwrap();
    colec = source.colecs.get_mut(&OsString::from("TEST")).unwrap();
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
    let r1 = rng.gen_range(1..((n_blocs*MAX_LINE_BLOC +remainder)/BATCH_SIZE));
    let r2 = rng.gen_range(1..((n_blocs*MAX_LINE_BLOC +remainder)/BATCH_SIZE));
    let bot_ts =  BATCH_SIZE*DEFAULT_STEP*min(r1,r2);
    let top_ts = BATCH_SIZE*DEFAULT_STEP*max(r1,r2);

  
    let res_data_ref = &sys.query_data(&format!("ts TEST:TEST::{}:{}", bot_ts, top_ts));


    let res_data =res_data_ref.to_owned();
    let awaited_data =gen_data
    .iter()
    .filter(|x| x.t>=Helpers::u64closest_down_divider(&bot_ts, &colec.map.step) && x.t<=Helpers::u64closest_up_divider(&top_ts, &colec.map.step))
    .map(|x| x).collect::<Vec<&TsPoint>>();
    assert_eq!(awaited_data.iter().zip(res_data).position(|(x,y)| *x!=y), None, "Testing equality between queried by ts data and stored data");
    assert_eq!(awaited_data.len() != 0, true, "Testing if data is not empty");
    assert_eq!(res_data.len() != 0, true, "Testing if data is not empty");
    let random_batch =  rng.gen_range(0..(colec.map.maxts-colec.map.mints)/colec.map.mints/BATCH_SIZE)+1;

    let res_data = &sys.query_data(&format!("batch TEST:TEST::{}", random_batch));
    let awaited_data =&colec.map.get_data(&random_batch, &1);
    assert_eq!(awaited_data.iter().zip(res_data).position(|(x,y)| x!=y), None, "Testing equality between queried by batch data and stored data");

    assert_eq!(awaited_data.len() != 0, true, "Testing if data is not empty");
    assert_eq!(res_data.len() != 0, true, "Testing if data is not empty");



    //Deleting the test collection
    assert_eq!(colec.delete().unwrap(), (), "Testing deletion of test collection");

    //Delete the test source
    assert_eq!(source.delete(), (), "Testing deleting of source")
    }
