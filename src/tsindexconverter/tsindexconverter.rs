use std::ffi::{OsStr, OsString};
use regex::Regex;

use crate::{system::System, BATCH_SIZE, collection::Collection, MAX_LINE_BLOC};



pub struct IndexResult {
    start_batchid : u64,
    bloc_rows_shift : u64,
    rows_take : u64,
    colec : &'static Collection
}




pub struct TsIndexConverter {
    source: &'static OsStr,
    collection : &'static OsStr, 
    startTs : u64,
    endTs : u64
}



impl TsIndexConverter {

    pub fn treatIndexResult(index_result : IndexResult) {


        let batch_shift = (index_result.bloc_rows_shift + index_result.rows_take)/BATCH_SIZE;
        index_result.colec.map.get_data(index_result.start_batchid, batch_shift);

        // (0..(bloc_shift)).into_iter().for_each(|bloc_shit_u| {

          


        // });



    }





    pub fn parseByTimestampQueryString(sys : &System, queryString : &str) -> IndexResult {
        let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
        let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
        let source = sys.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
        let colec = source.get(&caps[2]).expect("Collection is not existing");
        let start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
        let end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");
        match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
            (true, true, true) => {
                let start_pos = (start_ts-colec.map.mints)/colec.map.step;
                IndexResult{
                    start_batchid : start_pos/BATCH_SIZE + 1,
                    bloc_rows_shift : start_pos%BATCH_SIZE,
                    rows_take : (end_ts-start_ts)/colec.map.step,
                    colec: colec
                }
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