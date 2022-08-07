use std::ffi::OsString;


use crate::{channels::inoutputs::InOutputs, datastruct::tspoint::TsPoint};

use super::System;


impl System {
    fn inject_data_core<T>(&mut self, source : &str, collection : &str, data: T, parser : fn(T) -> Vec<TsPoint>) {
        let mut inserted_counts = 0;
        let c = self.sources.get_mut(&OsString::from(source)).expect("Source does not exist").get_mut(collection).expect("Collection does not exist");
        c.append_points(&parser(data), &mut false, &mut inserted_counts).expect("Adding points failed");
        println!("Inserted points : {}", inserted_counts);
    }

    pub fn inject_data_from_string(&mut self, source : &str, collection : &str, data: &str) {
        self.inject_data_core(source,collection,data, InOutputs::json_to_points)
    }
    pub fn inject_data_from_points(&mut self, source : &str, collection : &str, data: Vec<TsPoint>) {
        self.inject_data_core(source,collection,data, |x| x)
    }
} 
