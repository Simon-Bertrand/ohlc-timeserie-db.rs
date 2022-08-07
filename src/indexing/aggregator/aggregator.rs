use regex::Regex;

use crate::datastruct::tspoint::{TsPoint, TsPointData};




pub struct Aggregator{
    pub func : fn(u64, &[TsPoint]) -> TsPoint,
    pub width : u64
}


fn OHLC_core(width : u64, x : &[TsPoint]) -> TsPoint {
    return TsPoint { 
        t: x[0].t.to_owned(), 
        data : TsPointData { 
            o : x[0].data.o.to_owned(),
            h : {x.iter().map(|y| y.data.h).max().expect("No data given to compute the max in agg")},
            c : x[x.len()-1].data.c.to_owned(),
            l : {x.iter().map(|y| y.data.l).min().expect("No data given to compute the min in agg")}
        }
    }
}

impl Aggregator {
    pub fn aggregate(&self, data : &[TsPoint]) -> TsPoint {
        (self.func)(self.width, data)
    }


    pub fn ohlc(width : &u64) -> Self {
        Aggregator { 
            func : OHLC_core,
            width: *width
        }
    }




    pub fn get_aggregator_width(command : &str) -> u64 {
        let re = Regex::new(r"^-([0-9]+)([dwm])$").unwrap();
        let caps = re.captures_iter(command.trim()).nth(0).expect("Invalid aggregation argument");
        let mut num= caps[1].parse::<u64>().expect("Could not parse correctly the first numeric value of aggregator argument");
        let freq = &caps[2];
        if num<1 {num=1};

        num * match freq {
            "d"=> 24*60,
            "w"=> 7*24*60,
            "m"=> 1,
            _=> panic!("Invalid freq for aggregator")
        }

    }


}