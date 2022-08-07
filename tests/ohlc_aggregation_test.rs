
    use std::{ffi::OsString, fs, cmp::{min, max}};
    use rand::{prelude::*};


    use timeseries_database::{collection::Collection, system::System, tspoint::{TsPointData, TsPoint}, MAX_LINE_BLOC, schemaurl::SchemaURL, BATCH_SIZE, helpers::Helpers, DEFAULT_STEP, source::Source};
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    #[test]
    fn ohlc_aggregation_test() {

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

    let t1 = rng.gen_range(colec.map.mints..colec.map.maxts);
    let t2 = rng.gen_range(colec.map.mints..colec.map.maxts);
    let bot_ts =  min(t1,t2);
    let top_ts = max(t1,t2);

    let res_data_ref = &sys.query_data(&format!("ts -d TEST:TEST::{}:{}", bot_ts, top_ts));
    println!("data queried :{} -> {:?}", &format!("ts -d TEST:TEST::{}:{}", bot_ts, top_ts), res_data_ref);
    println!("minTs :{} maxTs {:?}", colec.map.mints, colec.map.maxts);
    
    assert_eq!(res_data_ref.len()!=0, true, "Testing if queried data is not empty");

    let aggregator_width = (res_data_ref[1].t-res_data_ref[0].t)/DEFAULT_STEP;

    let mut prec_point : &TsPoint;
    prec_point = &res_data_ref[0];
    for point in res_data_ref.iter().skip(1) {

        match (
            point.t == prec_point.t + aggregator_width*DEFAULT_STEP,
            prec_point.data.c.unwrap_or(Decimal::from_u64(0).unwrap()) + Decimal::from_u64(DEFAULT_STEP).unwrap() == Decimal::from(point.t),
            point.data.o.unwrap_or(Decimal::from(0)) == Decimal::from(point.t),
            point.data.h == point.data.c,
            point.data.h >= point.data.l )  {
                (true, true, true, true, true) => {assert_eq!(true, true)},
                (false,_,_,_,_) => {assert_eq!(false, true, "Testing equality point by point. Failed at point.t == prec_point.t + agg.width*default_step")},
                (_,false,_,_,_) => {assert_eq!(false, true, "Testing equality point by point. Failed at point.t == prec_point.c + default_step")},
                (_,_,false,_,_) => {assert_eq!(false, true, "Testing equality point by point. Failed at point.t == point.o")},
                (_,_,_,false,_) => {assert_eq!(false, true, "Testing equality point by point. Failed at point.h == point.c")},
                (_,_,_,_,false) => {assert_eq!(false, true, "Testing equality point by point. Failed at point.h > point.l")}

        }

        prec_point=point;
    }





    //Deleting the test collection
    assert_eq!(colec.delete().unwrap(), (), "Testing deletion of test collection");

    //Delete the test source
    assert_eq!(source.delete(), (), "Testing deleting of source")

    }
