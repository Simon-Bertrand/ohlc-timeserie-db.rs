use std::iter::{TakeWhile, Enumerate, Map};
use std::option::Iter;
use std::{ops::Index, ffi::OsString, collections::HashMap};
use regex::Regex;

use crate::BATCH_SIZE;
use crate::filemanager::FileManager;
use crate::helpers::Helpers;

use crate::indexresult::{IndexResult, Aggregator};
use crate::inoutputs::InOutputs;
use crate::tspoint::{TsPoint, TsPointData, TsPointAggregated};
use crate::{ source::Source, channel::Channel};


use crate::{schemaurl::SchemaURL, bloc::Bloc, collection::Collection,indexmap::IndexMap};


#[derive(Debug)]
pub struct System {
    pub files : FileManager,
    pub sources : HashMap<OsString, Source>
}


impl System {
    pub fn instanciate() -> Self {
        let mut sys = System {
            files : FileManager { schema : FileManager::scan_db_repo() },
            sources : HashMap::new()
        };
        sys.get_all_sources();
        sys
    }
}

impl System {
    pub fn main() {
     let mut sys = System::instanciate();
     let mut _i =0;
     let _b : &mut Collection =  sys.sources.get_mut(&OsString::from("Binance")).unwrap().colecs.get_mut(&OsString::from("BTCUSDT")).unwrap();
    }

}


impl System {
    fn inject_data_core<T>(&mut self, source : &str, collection : &str, data: T, parser : fn(T) -> Vec<TsPoint>) {
        let mut inserted_counts = 0;
        let c = self.sources.get_mut(&OsString::from(source)).expect("Source does not exist").get_mut(collection).expect("Collection does not exist");
        c.append_points(&parser(data), &mut false, &mut inserted_counts).expect("Adding points failed");
        println!("Inserted points : {}", inserted_counts);
    }

    pub fn inject_data_from_string(&mut self, source : &str, collection : &str, data: &str) {
        self.inject_data_core(source,collection,data, InOutputs::JsonToTsPoints)
    }
    pub fn inject_data_from_points(&mut self, source : &str, collection : &str, data: Vec<TsPoint>) {
        self.inject_data_core(source,collection,data, |x| x)
    }
} 




impl System {
    pub fn query_data(&self, query_string : &str) -> String {
        let index_result = self.parseQueryString( query_string);
        InOutputs::PointsToJson({
            &index_result
            .colec
            .map
            .get_data(&index_result.start_batchid, &(&index_result.end_batchid - &index_result.start_batchid))[(index_result.start_ind as usize)..(index_result.end_ind as usize)]
        })
    }
}  



impl System {

    fn useIndexResult(&self, ir : &'static IndexResult) -> Vec<TsPointAggregated>{
        match &ir.aggregator {
            Some(agreg) => {
                ir.colec.map.get_data(&ir.start_batchid, &(ir.end_batchid-ir.start_batchid))
                .into_iter()
                .enumerate()
                .filter(|(i,x)| i>=&(ir.start_ind as usize) && i<&(ir.end_ind as usize)).map(|(x,y)| y)
                .collect::<Vec<TsPoint>>()
                .chunks(agreg.width as usize)
                .map(|x|agreg.aggregate(x))
                .collect::<Vec<TsPointAggregated>>()
            }, 
            None => {
                ir.colec.map.get_data(&ir.start_batchid, &(ir.end_batchid-ir.start_batchid))
                .into_iter()
                .enumerate()
                .filter(|(i,x)| i>=&(ir.start_ind as usize) && i<&(ir.end_ind as usize)).map(|(x,y)| y)
                .map(|x|TsPointAggregated {
                    step : ir.colec.map.step,
                    point : x
                })
                .collect::<Vec<TsPointAggregated>>()
            }
        }
      
    }

    pub fn parseQueryString(&self, queryString : &str) -> IndexResult {
        let re = Regex::new(r"^(ts|batch)((?:\s-[0-9a-zA-Z]+)?) ([:0-9a-zA-Z]+)$").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string. Need to start by ts or batch");
        match (&caps[1], &caps[2]) {
            ("ts", "") => {
                
                self.ts_query_string_to_indexresult(&caps[3])
            },
            ("batch", "") => {
                self.batch_query_string_to_indexresult(&caps[3])
            },
            ("ts", arg) => {
                self.agg_ts_query_string_to_indexresult(&caps[3], Aggregator::ohlc(&16))
            }
            _ => {panic!("Unrecognized start of query string")}
        }   
    }



    fn agg_ts_query_string_to_indexresult(&self,queryString : &str, agg : Aggregator) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = self.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let mut start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
        let mut end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");


        match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
            (true, true, true) => {

    
                start_ts = Helpers::u64closest_down_divider(&start_ts, &(agg.width*colec.map.step));
                let x1 = Helpers::u64overflowed_substract(&start_ts,&colec.map.mints,&0)/colec.map.step;
                let batch_inferior = x1/BATCH_SIZE;

                end_ts = Helpers::u64closest_up_divider(&end_ts, &(agg.width*colec.map.step));
                let x2 = Helpers::u64overflowed_rangefit(&0,&(colec.map.maxts-colec.map.mints), &(end_ts-colec.map.mints))/colec.map.step;
                let batch_superior = x2/BATCH_SIZE +1;

                IndexResult::create(&(batch_inferior+1),&(x1 - batch_inferior),&x2,&(batch_superior+1), colec, Some(agg))
            
            },
            (false, _, _) => {panic!("Start ts is not superior to end ts")},
            (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
            (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
        }
    
    }

    fn batch_query_string_to_indexresult(&self, queryString : &str) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = self.sources.get_key_value(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.1.colecs.get_key_value(&OsString::from(&caps[2])).expect("Collection is not existing");
        let batchid= caps[3].parse::<u64>().expect("Could not parse correctly the batchId value");
        match (batchid>=1, batchid<=colec.1.map.get_virtual_n_samples()/BATCH_SIZE+1 ){
            (true, true,) => {
                IndexResult::create(&batchid,&0,&BATCH_SIZE,& (batchid+1), colec.1, None)
            },
            (false, _,) => {panic!("Batch id is not superior to 1")},
            (_, false) => {panic!("Batch id is superior to the max batchid")}
        }
    }


    fn ts_query_string_to_indexresult(&self, queryString : &str) -> IndexResult {

        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = self.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let mut start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
        let mut end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");

        match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
            (true, true, true) => {

                start_ts = Helpers::u64closest_down_divider(&start_ts, &colec.map.step);
                let x1 = Helpers::u64overflowed_substract(&start_ts,&colec.map.mints,&0)/colec.map.step;
                let batch_inferior = x1/BATCH_SIZE;

                end_ts = Helpers::u64closest_up_divider(&end_ts, &colec.map.step);
                let x2 = Helpers::u64overflowed_rangefit(&0,&(colec.map.maxts-colec.map.mints), &(end_ts-colec.map.mints))/colec.map.step;
                let batch_superior = x2/BATCH_SIZE +1;

                IndexResult::create(&(batch_inferior+1),&(x1 - batch_inferior),&x2,&(batch_superior+1), colec, None)
            
            },
            (false, _, _) => {panic!("Start ts is not superior to end ts")},
            (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
            (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
        }
    }
}













impl Index<&str> for System {
    type Output = Source;
    fn index<'a>(&'a self, source: &str) -> &'a Source {
        self.sources.get(&OsString::from(source)).unwrap()
    }
}




impl System {
    pub fn get_all_sources(&mut self) {
        for (source, _hm) in self.files.schema.iter() {
            self.sources.insert(source.to_owned(), Source {
                name :source.to_owned(),
                colecs : self.get_all_collections(source)
            });
        }
    }

    pub fn get_indexmap(&self, source : &OsString, collection : &OsString)-> IndexMap {
        let bytes = Channel::file_to_bytes(&SchemaURL::get_indexmap_path(source, collection));
        Channel::bytes_to_indexmap(&bytes)
    }

    pub fn get_all_collections(&self, source : &OsString) -> HashMap<OsString, Collection> {
     let mut data : HashMap<OsString, Collection> = HashMap::new();
        for (collection, _hm) in self.files.schema.get(source).unwrap().iter() {
            data.insert(collection.to_owned(),Collection {
                    name: collection.to_owned(),
                    map: self.get_indexmap(source, collection),
                    path: SchemaURL::get_collection_path(source, collection),
                    blocs : self.get_all_blocs(source, collection),
            });
        } 
        data
    }

    pub fn get_all_blocs(&self, source : &OsString, collection : &OsString) -> HashMap<OsString, Bloc>{
        let mut data : HashMap<OsString,Bloc> = HashMap::new();
        for bloc in self.files.schema.get(source).unwrap().get(collection).unwrap().iter() {
            data.insert(bloc.to_owned(),Bloc {
                name : bloc.to_owned(),
                path : SchemaURL::get_bloc_path(source, collection, bloc),
                size : 0,
            });
        }
        data
    }

}



    
