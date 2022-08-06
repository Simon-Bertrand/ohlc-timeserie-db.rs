use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::encodemap::EncodeMap;



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TsPoint {
    pub t: u64,
    pub data : TsPointData
}







impl TsPoint {

    fn _create_empty(t:u64) -> Self {
        TsPoint {
            t : t,
            data : TsPointData {
                o : None,
                c:  None,
                l : None,
                h : None
            }
        }
    }

}


#[derive(Debug)]
pub struct TsPointDataRaw<'a>{
    pub o:&'a [u8],
    pub h:&'a [u8],
    pub c:&'a [u8],
    pub l:&'a [u8]
} 



#[derive(Debug,Serialize, Deserialize, PartialEq)]
pub struct TsPointData{
    pub o:Option<Decimal>,
    pub h:Option<Decimal>,
    pub c:Option<Decimal>,
    pub l:Option<Decimal>
} 

impl<'a> TsPointDataRaw<'a> {
    pub fn parse(&self) -> TsPointData {
        TsPointData {
            o: Some(Decimal::from_str(&self.o.iter().map(|x| EncodeMap::bytes_to_char(&x).unwrap()).collect::<String>().replace("-", ""))
                    .expect(&format!("Parsing open {:?} is invalid", &self.o))),
            c: Some(Decimal::from_str(&self.c.iter().map(|x| EncodeMap::bytes_to_char(&x).unwrap()).collect::<String>().replace("-", ""))
                    .expect(&format!("Parsing close {:?} is invalid", &self.c))),
            l: Some(Decimal::from_str(&self.l.iter().map(|x| EncodeMap::bytes_to_char(&x).unwrap()).collect::<String>().replace("-", ""))
                    .expect(&format!("Parsing low {:?} is invalid", &self.l))),
            h: Some(Decimal::from_str(&self.h.iter().map(|x| EncodeMap::bytes_to_char(&x).unwrap()).collect::<String>().replace("-", ""))
                    .expect(&format!("Parsing high {:?} is invalid", &self.h)))
        }
    }
}