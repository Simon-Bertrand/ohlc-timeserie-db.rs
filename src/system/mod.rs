pub mod system;
pub use system::System;

    //     let mut inserted_count = 0;
    // b.append_points(&(0..50000).into_iter().map(|x| TsPoint { 
    //     t: x*60,
    //     data : TsPointData { 
    //     c:Some(Decimal::from_u64(x).unwrap()),
    //     h:Some(Decimal::from_u64(x*60).unwrap()),
    //     l:Some(Decimal::from_u64(x / 256 ).unwrap()),
    //     o:Some(Decimal::from_u64(x / MAX_LINE_BLOC).unwrap())
    // }} ).collect::<Vec<TsPoint>>(), &mut false, &mut inserted_count);


        // println!("Inserted rows :  {}", inserted_count);

        // let mut bytes = Channel::file_to_bytes(&Path::new("./db/Binance/BTCUSDT/1"));
        // let mut decoded_bytes : Vec<u8> = Vec::new();
        // bytes.iter().for_each(|x|EncodeMap::uncompress(&mut decoded_bytes,x));
        
        //println!("{:?}", EncodeMap::Serialize(&decoded_bytes).iter().map(|x| x.parse()).collect::<Vec<TsPointData>>());



        //println!("{:?}", decoded_bytes.iter().map(|x| EncodeMap::bytes_to_char(x).unwrap()).collect::<String >());
        


        // println!("{:?}", b.map.get_data(5, 3));
                //Channel::FileToString(Path::new("data2.csv"));
        // FileManager::scan_db_repo();
        //let colec = Collection::create(&OsString::from("Binance"), &OsString::from("BTCUSDT"));
        // let mut sys = System::instanciate();
        // let b : &Collection = &sys["Source"]["BTCUSDT"];
        // println!("{}", b);
        // Channel::compressFile(&sys["Binance"]["ETHUSDT"]["1"].path);
        // Channel::uncompressFile(&sys["Binance"]["ETHUSDT"]["1"].path);
        //Channel::Stringcompress(&SchemaURL::get_bloc_path(&OsString::from("Binance"), &OsString::from("ETHUSDT"), &OsString::from("1")), "");


  