use std::io::Error;

use crate::tspoint::{TsPointData, TsPointDataRaw};

use super::EncodeMap;



impl EncodeMap {
    pub fn string_parse(data : &str) -> Result<Vec<u8>, Error> {
        data.chars().map(|x| EncodeMap::char_to_bytes(&x)).collect()
    }
    
    pub fn deserialize(tspoints : &Vec<&TsPointData>) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(120);
        for tspoint in tspoints.iter() {
            vec.append(&mut EncodeMap::string_parse(&tspoint.o.unwrap().to_string()).unwrap());
            vec.push(EncodeMap::char_to_bytes(&';').unwrap());
            vec.append(&mut EncodeMap::string_parse(&tspoint.h.unwrap().to_string()).unwrap());
            vec.push(EncodeMap::char_to_bytes(&';').unwrap());
            vec.append(&mut EncodeMap::string_parse(&tspoint.c.unwrap().to_string()).unwrap());
            vec.push(EncodeMap::char_to_bytes(&';').unwrap());
            vec.append(&mut EncodeMap::string_parse(&tspoint.l.unwrap().to_string()).unwrap());
            vec.push(EncodeMap::char_to_bytes(&'*').unwrap());

        }
        vec
    }


    pub fn serialize(bytes : &[u8]) -> Vec<TsPointDataRaw> {
        let mut res : Vec<TsPointDataRaw> = Vec::new();
        let mut byteslines_iter = bytes.split(|x| x == &EncodeMap::char_to_bytes(&'*').expect("deserializer - End character * not found"))
        .filter(|x| x.len() != 0 );
        while let Some(bytesline) = byteslines_iter.next() {
                let mut bytesvalue_iter = bytesline.split(|x| x == &EncodeMap::char_to_bytes(&';').expect("deserializer - Separator character ; not found"));
                res.push(TsPointDataRaw{
                    o: bytesvalue_iter.next().expect("deserializer - Could not load open price"),
                    h: bytesvalue_iter.next().expect("deserializer - Could not load high price"),
                    c: bytesvalue_iter.next().expect("deserializer - Could not load close price"),
                    l: bytesvalue_iter.next().expect("deserializer - Could not load low price"),
                });
        }
        res
    }
}