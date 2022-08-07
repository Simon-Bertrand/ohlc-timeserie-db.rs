
use crate::datastruct::tspoint::TsPoint;

use super::System;


impl System {

    pub fn query_data(&self, query_string : &str) -> Vec<TsPoint> {
        self.use_index_result(&self.parse_query_string(query_string)).1
    }

}
