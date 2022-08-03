
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::tspoint::{TsPoint, TsPointData};

#[derive(Deserialize, Serialize, Debug)]
struct DeserializerTsPoint {
    t: u64,
    pub o:Decimal,
    pub h:Decimal,
    pub c:Decimal,
    pub l:Decimal
}
#[derive(Deserialize, Serialize, Debug)]
struct SerializerTsPoint {
    t: u64,
    pub o:Option<Decimal>,
    pub h:Option<Decimal>,
    pub c:Option<Decimal>,
    pub l:Option<Decimal>
}



pub struct InOutputs {

}


impl InOutputs {
    pub fn JsonToTsPoints(dataToParse : &str) -> Vec<TsPoint> {
        let mut deserialized : Vec<DeserializerTsPoint> = serde_json::from_str(dataToParse).unwrap();
        deserialized.sort_by(|a, b| a.t.cmp(&b.t));
        deserialized.iter().map(|data| {
            return TsPoint { t : data.t, data : TsPointData { o :Some(data.o), h:Some(data.h), c:Some(data.c), l:Some(data.l) }}
        }).collect::<Vec<TsPoint>>()
    }

    pub fn PointsToJson(points: &Vec<TsPoint>) -> String {
        serde_json::to_string({
            &points.iter().map(|point| {
                return SerializerTsPoint { t : point.t,  o :point.data.o, h:point.data.h, c:point.data.c, l:point.data.l }
            }).collect::<Vec<SerializerTsPoint>>()
        }).unwrap()
    }
}