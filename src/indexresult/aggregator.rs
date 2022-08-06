use crate::tspoint::{TsPoint, TsPointData, TsPointAggregated};


pub struct Aggregator {
    pub func : (fn(&'static u64, &[TsPoint]) -> TsPointAggregated),
    pub width : u64
}




fn OHLC_core(width : &'static u64, x : &[TsPoint]) -> TsPointAggregated {
    TsPointAggregated {
        step : *width,
        point : TsPoint { 
            t:x[0].t.to_owned(), 
            data : TsPointData { o : x[0].data.o.to_owned(),h : x[0].data.h.to_owned(),c : x[0].data.c.to_owned(),l : x[0].data.l.to_owned()}
        }
    }

}

impl Aggregator {
    pub fn aggregate(&'static self, data : &[TsPoint]) -> TsPointAggregated {
        (self.func)(&self.width, data)
    }


    pub fn ohlc(width : &u64) -> Self {
        Aggregator { 
            func : OHLC_core,
            width: *width
        }
    }
}