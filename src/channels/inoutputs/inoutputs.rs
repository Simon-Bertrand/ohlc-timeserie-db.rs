
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::datastruct::tspoint::{TsPoint, TsPointData};



#[derive(Deserialize, Serialize, Debug)]
struct SerializeTsPoint {
    t: u64,
    pub o:Option<Decimal>,
    pub h:Option<Decimal>,
    pub c:Option<Decimal>,
    pub l:Option<Decimal>
}



pub struct InOutputs;


impl InOutputs {
    pub fn json_to_points(dataToParse : &str) -> Vec<TsPoint> {
        let mut deserialized : Vec<SerializeTsPoint> = serde_json::from_str(dataToParse).unwrap();
        deserialized.sort_by(|a, b| a.t.cmp(&b.t));
        deserialized.iter().map(|data| {
            return TsPoint { t : data.t, data : TsPointData { o :data.o, h:data.h, c:data.c, l:data.l }}
        }).collect::<Vec<TsPoint>>()
    }

    pub fn points_to_json(points: &[TsPoint]) -> String {
        serde_json::to_string({
            &points.iter().map(|point| {
                return SerializeTsPoint { t : point.t,  o :point.data.o, h:point.data.h, c:point.data.c, l:point.data.l }
            }).collect::<Vec<SerializeTsPoint>>()
        }).unwrap()
    }
}