use std::ffi::OsString;

use regex::Regex;


use crate::{datastruct::{tspoint::TsPoint, collection::Collection}, indexing::{indexresult::IndexResult, aggregator::Aggregator}, BATCH_SIZE, sysutils::helpers::Helpers};


use super::System;

impl System {
    pub fn use_index_result(&self, ir : &IndexResult) -> (u64, Vec<TsPoint>){

        fn get_data (ir : &IndexResult) -> Vec<TsPoint> {
            ir.colec.map.get_data(&ir.start_batchid, &(ir.end_batchid-ir.start_batchid))
            .into_iter()
            .enumerate()
            .filter(|(i,_x)| i>=&(ir.start_ind as usize) && i<=&(ir.end_ind as usize)).map(|(_x,y)| y)
            .collect::<Vec<TsPoint>>()
        }

        match &ir.aggregator {
            Some(agreg) => {
                (
                    agreg.width*ir.colec.map.step, 
                    get_data(ir).chunks(agreg.width as usize)
                    .map(|x|agreg.aggregate(x))
                    .collect::<Vec<TsPoint>>()
                )
            }, 
            None => {
                (ir.colec.map.step, get_data(ir))
            }
        }
      
    }

    pub fn parse_query_string(&self, query_string : &str) -> IndexResult {
        let re = Regex::new(r"^(ts|batch)((?:\s-[0-9a-zA-Z]+)?) ([:0-9a-zA-Z]+)$").unwrap();
        let caps = re.captures_iter(query_string).nth(0).expect("Invalid query string. Need to start by ts or batch");
        match (&caps[1], &caps[2]) {
            ("ts", "") => {
                self.ts_query_string_to_indexresult(&caps[3], None)
            },
            ("batch", "") => {
                self.batch_query_string_to_indexresult(&caps[3])
            },
            ("ts", arg) => {
                self.ts_query_string_to_indexresult(&caps[3], Some(Aggregator::ohlc(&Aggregator::get_aggregator_width(arg))))
            }
            _ => {panic!("Unrecognized start of query string")}
        }   
    }

    fn ts_query_string_to_indexresult(&self, query_string : &str, agg : Option<Aggregator>) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(query_string).nth(0).expect("Invalid query string by timestamp");
        let source = self.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let mut start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
        let mut end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");
        match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
            (true, true, true) => {

                start_ts = Helpers::u64closest_down_divider(&start_ts, &(agg.as_ref().map_or(1, |x|x.width)*colec.map.step));
                let x1 = Helpers::u64overflowed_substract(&start_ts,&colec.map.mints,&0)/colec.map.step;
                let batch_inferior = x1/BATCH_SIZE;

                end_ts = Helpers::u64closest_up_divider(&end_ts, &(agg.as_ref().map_or(1, |x|x.width)*colec.map.step));
                let x2 = Helpers::u64overflowed_rangefit(&0,&(colec.map.maxts-colec.map.mints), &(end_ts-colec.map.mints))/colec.map.step;
                let batch_superior = x2/BATCH_SIZE +1;

                IndexResult::create(
                    &(batch_inferior+1),&(x1 - batch_inferior*BATCH_SIZE),&(x2 - batch_inferior*BATCH_SIZE),&(batch_superior+1), colec, agg.into()
                )            
            },
            (false, _, _) => {panic!("Start ts is not superior to end ts")},
            (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
            (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
        }
    
    }


    fn batch_query_string_to_indexresult(&self, query_string : &str) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(query_string).nth(0).expect("Invalid query string by timestamp");
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
}