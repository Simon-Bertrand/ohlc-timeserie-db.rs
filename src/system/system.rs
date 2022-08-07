
use std::{ops::Index, ffi::OsString, collections::HashMap};

use crate::{sysutils::filemanager::FileManager, datastruct::{source::Source, collection::Collection}};


#[derive(Debug)]
pub struct System {
    pub files : FileManager,
    pub sources : HashMap<OsString, Source>
}

impl System {
    pub fn main() {
     let mut sys = System::instanciate();
     let mut _i =0;
     let _b : &mut Collection =  sys.sources.get_mut(&OsString::from("Binance")).unwrap().colecs.get_mut(&OsString::from("BTCUSDT")).unwrap();
    }

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




impl Index<&str> for System {
    type Output = Source;
    fn index<'a>(&'a self, source: &str) -> &'a Source {
        self.sources.get(&OsString::from(source)).unwrap()
    }
}



    


    // fn ts_query_string_to_indexresult(&self, queryString : &str) -> IndexResult {

    //     let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
    //     let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
    //     let source = self.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
    //     let colec = source.get(&caps[2]).expect("Collection is not existing");
    //     let mut start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
    //     let mut end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");

    //     match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
    //         (true, true, true) => {

    //             start_ts = Helpers::u64closest_down_divider(&start_ts, &colec.map.step);
    //             let x1 = Helpers::u64overflowed_substract(&start_ts,&colec.map.mints,&0)/colec.map.step;
    //             let batch_inferior = x1/BATCH_SIZE;

    //             end_ts = Helpers::u64closest_up_divider(&end_ts, &colec.map.step);
    //             let x2 = Helpers::u64overflowed_rangefit(&0,&(colec.map.maxts-colec.map.mints), &(end_ts-colec.map.mints))/colec.map.step;
    //             let batch_superior = x2/BATCH_SIZE +1;

    //             IndexResult::create(
    //                 &(batch_inferior+1),&(x1 - batch_inferior*BATCH_SIZE),&(x2 - batch_inferior*BATCH_SIZE),&(batch_superior+1), colec, None
    //             )
                
    //         },
    //         (false, _, _) => {panic!("Start ts is not superior to end ts")},
    //         (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
    //         (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
    //     }
    // }

        // fn agg_ts_query_string_to_indexresult(&self,queryString : &str, agg : Aggregator) -> IndexResult {
    //     //self.ts_query_string_to_indexresult_core(queryString, Some(agg))=)
    //     let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z]+)::([1-9][0-9]*):([1-9][0-9]*)").unwrap();
    //     let caps = re.captures_iter(queryString).nth(0).expect("Invalid query string by timestamp");
    //     let source = self.sources.get(&OsString::from(&caps[1])).expect("Source is not existing");
    //     let colec = source.get(&caps[2]).expect("Collection is not existing");
    //     let mut start_ts= caps[3].parse::<u64>().expect("Could not parse correctly the startTs value");
    //     let mut end_ts = caps[4].parse::<u64>().expect("Could not parse correctly the endTs value");
    //     match (start_ts<end_ts , colec.map.mints<=end_ts , colec.map.maxts>=start_ts ){
    //         (true, true, true) => {

    
    //             start_ts = Helpers::u64closest_down_divider(&start_ts, &(agg.width*colec.map.step));
    //             let x1 = Helpers::u64overflowed_substract(&start_ts,&colec.map.mints,&0)/colec.map.step;
    //             let batch_inferior = x1/BATCH_SIZE;

    //             end_ts = Helpers::u64closest_up_divider(&end_ts, &(agg.width*colec.map.step));
    //             let x2 = Helpers::u64overflowed_rangefit(&0,&(colec.map.maxts-colec.map.mints), &(end_ts-colec.map.mints))/colec.map.step;
    //             let batch_superior = x2/BATCH_SIZE +1;


    //             IndexResult::create(
    //                 &(batch_inferior+1),&(x1 - batch_inferior*BATCH_SIZE),&(x2 - batch_inferior*BATCH_SIZE),&(batch_superior+1), colec, Some(agg)
    //             )            
    //         },
    //         (false, _, _) => {panic!("Start ts is not superior to end ts")},
    //         (_, false, _) => {panic!("End ts is inferior to the min timestamp of collection")},
    //         (_, _, false) => {panic!("Start ts is superior to the max timestamp of collection")}
    //     }
    
    // }
