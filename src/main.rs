use std::{ffi::OsString};

use timeseries_database::{system::System, collection::Collection};


fn main() {
    use std::time::Instant;
    let now = Instant::now();

    
    let mut sys = System::instanciate();
    let _c : &mut Collection =  sys.sources.get_mut(&OsString::from("TEST")).unwrap().colecs.get_mut(&OsString::from("TEST")).unwrap();
   
    System::main();
    let elapsed = now.elapsed();
    println!("_______________________________");
    println!("Elapsed: {:.2?}", elapsed);

}
