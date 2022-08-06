
    use std::{ffi::OsString, fs, cmp::{min, max}};
    use rand::{prelude::*};


    use timeseries_database::{collection::Collection, system::System, tspoint::{TsPointData, TsPoint}, MAX_LINE_BLOC, schemaurl::SchemaURL, BATCH_SIZE, helpers::Helpers};
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    #[test]
    fn aggregation_test() {

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

    let r1 = rng.gen_range(colec.map.mints..colec.map.maxts);
    let r2 = rng.gen_range(colec.map.mints..colec.map.maxts);
    let bot_ts =  min(r1,r2);
    let top_ts = max(r1,r2);

    let res_data_ref = &sys.query_data(&format!("ts -d TEST:TEST::{}:{}", bot_ts, top_ts));
    println!("data queried :{} -> {:?}", &format!("ts -d TEST:TEST::{}:{}", bot_ts, top_ts), res_data_ref);
    println!("minTs :{} maxTs {:?}", colec.map.mints, colec.map.maxts);




    }
