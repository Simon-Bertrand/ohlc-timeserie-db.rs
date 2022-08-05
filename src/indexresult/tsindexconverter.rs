use std::ffi::{OsStr, OsString};
use regex::Regex;

use crate::{system::System, BATCH_SIZE, collection::Collection, MAX_LINE_BLOC, DEFAULT_STEP};

use super::IndexResult;






// pub struct TsIndexConverter {
//     source: &'static OsStr,
//     collection : &'static OsStr, 
//     startTs : u64,
//     endTs : u64
// }



// impl TsIndexConverter {


//     // pub fn treatIndexResult(index_result : IndexResult) {


//     //     let batch_shift = (index_result.bloc_rows_shift + index_result.rows_take)/BATCH_SIZE +1;
//     //     index_result.colec.map.get_data(index_result.start_batchid, batch_shift);

//     //     // (0..(bloc_shift)).into_iter().for_each(|bloc_shit_u| {

          


//     //     // });



//     // }


    


// }

// TsIndexConverter {
//     source : &OsString::from(&caps[1]),
//     collection :&OsString::from(&caps[2]),
//     startTs : 
//     endTs: 