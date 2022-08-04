use std::ffi::{OsStr, OsString};
use regex::Regex;

use crate::{system::System, BATCH_SIZE, collection::Collection, MAX_LINE_BLOC, DEFAULT_STEP};

use super::IndexResult;






pub struct TsIndexConverter {
    source: &'static OsStr,
    collection : &'static OsStr, 
    startTs : u64,
    endTs : u64
}



impl TsIndexConverter {


    // pub fn treatIndexResult(index_result : IndexResult) {


    //     let batch_shift = (index_result.bloc_rows_shift + index_result.rows_take)/BATCH_SIZE +1;
    //     index_result.colec.map.get_data(index_result.start_batchid, batch_shift);

    //     // (0..(bloc_shift)).into_iter().for_each(|bloc_shit_u| {

          


    //     // });



    // }


    pub fn parseQueryString(sys : &'static System, queryString : &str) -> IndexResult {
        let re = Regex::new(r"^(ts|batch) ([:0-9a-zA-Z]+)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string. Need to start by ts or batch");
        match &caps[1] {
            "ts" => {
                Self::parseByTimestampQueryString(sys, queryString)
            },
            "batch" => {
                Self::parseByBatchIdQueryString(sys, queryString)
            },
            _ => {panic!("Unrecognized start of query string")}
        }
        
    }




    fn parseByBatchIdQueryString(sys : &'static System, queryString : &str) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = sys.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let batchid= caps[3].parse::<u64>().expect("Could not parse correctly the batchId value");
        match (batchid>=1, batchid<=colec.map.get_virtual_n_samples()/BATCH_SIZE+1 ){
            (true, true,) => {
                IndexResult::create(&batchid,&0,&((batchid+1)*BATCH_SIZE),& (batchid+1), colec)
            },
            (false, _,) => {panic!("Batch id is not superior to 1")},
            (_, false) => {panic!("Batch id is superior to the max batchid")}
        }
    }


    fn parseByTimestampQueryString(sys : &'static System, queryString : &str) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = sys.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
        let end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");
        match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
            (true, true, true) => {
                let start_pos = (start_ts-colec.map.mints)/colec.map.step;
                let start_batch_id = start_pos/BATCH_SIZE +1;
                let start_shift = start_pos%BATCH_SIZE;
                
                let end_pos = (end_ts-colec.map.mints)/colec.map.step +1;
                let end_batch_id = end_pos/BATCH_SIZE +2;
                let end_ind =  end_batch_id * BATCH_SIZE - end_pos;

                
                
                IndexResult::create(&start_batch_id,&start_shift,&end_ind,&end_batch_id, colec)
            },
            (false, _, _) => {panic!("Start ts is not superior to end ts")},
            (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
            (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
        }
    }


}

// TsIndexConverter {
//     source : &OsString::from(&caps[1]),
//     collection :&OsString::from(&caps[2]),
//     startTs : 
//     endTs: 