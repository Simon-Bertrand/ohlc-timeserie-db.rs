
    use std::{ffi::OsString, fs};

    use timeseries_database::{collection::Collection, system::System, tspoint::{TsPointData, TsPoint}, MAX_LINE_BLOC, schemaurl::SchemaURL, BATCH_SIZE};
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    #[test]
    fn write_read_test() {

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
    let colec : &mut Collection =  sys.sources.get_mut(&OsString::from("TEST")).unwrap().colecs.get_mut(&OsString::from("TEST")).unwrap();
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


    //Read the data 
    let readed_data_all_batchs = colec.map.get_data(&1, &((n_blocs*MAX_LINE_BLOC)/BATCH_SIZE +1));


    //Testing if written and generated data are different
    assert_eq!((readed_data_all_batchs.iter().zip(gen_data.iter()).find(|(x, y)| x!=y ), readed_data_all_batchs.iter().zip(gen_data.iter()).position(|(x, y)| x!=y )), (None,None), "Test for equality between readed all batchs and written data" );




    let mut readed_data_per_batch : Vec<TsPoint> = Vec::new();
    colec.map.pos.keys().into_iter().for_each(|batch_id| {
        readed_data_per_batch.append(&mut colec.map.get_data(batch_id, &1));
    });

     //Testing if written and generated data are different
     assert_eq!((readed_data_per_batch.iter().zip(gen_data.iter()).find(|(x, y)| x!=y ), readed_data_per_batch.iter().zip(gen_data.iter()).position(|(x, y)| x!=y )), (None,None), "Test for equality between readed batch per batch and written data " );



    //Deleting the test collection
    assert_eq!(colec.delete().unwrap(), (), "Testing deletion of test collection");

    //Delete the test source
    assert_eq!(sys.sources.get_mut(&OsString::from("TEST")).unwrap().delete(), (), "Testing deleting of source")

    }
