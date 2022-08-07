
use std::{fs::{File, OpenOptions}, path::Path, io::{Read, Write, BufReader,Seek, SeekFrom}};

use crate::{datastruct::indexmap::IndexMap, compression::encodemap::encodemap::EncodeMap};

#[derive(Debug)]
pub struct BinaryChannel{}

impl BinaryChannel {

    pub fn indexmap_to_bytes(raw_data : &IndexMap)-> Vec<u8> {
        bincode::serialize(&raw_data).unwrap()
    }
    pub fn bytes_to_indexmap(bytes : &[u8]) -> IndexMap {
        bincode::deserialize(&bytes[..]).unwrap()
    }

    pub fn file_to_bytes(path : &Path) -> Vec<u8> {
        BufReader::new(File::open(path).unwrap()).bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()
    }

    pub fn append_linecompressed_to_dynfile(f: &mut File, bytes : &[u8]) -> u64 {
        let pos = f.seek(SeekFrom::End(0)).unwrap(); 
        f.write_all(&bytes.chunks(2).map(|x| 
            EncodeMap::compress(
                x.get(0).unwrap_or(&EncodeMap::char_to_bytes(&'-').unwrap()),
                x.get(1).unwrap_or(&EncodeMap::char_to_bytes(&'-').unwrap())
            )).collect::<Vec<u8>>()).expect("write failed");
        pos
    }

    pub fn write_to_file(path : &Path, bytes : &[u8]){
        // let mut buf: ByteBuf = bincode::deserialize(&BinaryChannel::file_to_bytes(path)).unwrap();
        // buf.append(&mut bytes.to_owned());
        let mut f = OpenOptions::new().write(true).open(path).expect("cannot open file");
        f.write_all(bytes).expect("write failed");
    }


}

