use std::{ffi::OsString};

use timeseries_database::channels::inoutputs::InOutputs;




fn main() {
    use std::time::Instant;
    let now = Instant::now();

    
    // let mut sys = System::instanciate();
    // let _c : &mut Collection =  sys.sources.get_mut(&OsString::from("TEST")).unwrap().colecs.get_mut(&OsString::from("TEST")).unwrap();
   
    // System::main();

    println!("parsed result : {:?}" ,InOutputs::json_to_points(r#"[{"t":15, "o":15, "h":158000, "l":512, "c":15 }, {"t":11, "o":15, "h":158000, "l":512, "c":15 }]"#) );
    println!("readed result : {}" , InOutputs::points_to_json(&InOutputs::json_to_points(r#"[{"t":15, "o":15, "h":158000, "l":512, "c":15 }, {"t":11, "o":15, "h":158000, "l":512, "c":15 }]"#)));





    let elapsed = now.elapsed();
    println!("_______________________________");
    println!("Elapsed: {:.2?}", elapsed);

}
