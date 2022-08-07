
use std::{ffi::OsString, fs::File, io::{BufReader, Seek, SeekFrom, Read}, path::{PathBuf}};


use crate::{DEFAULT_STEP,BATCH_SIZE, MAX_LINE_BLOC, datastruct::tspoint::TsPoint, compression::encodemap::encodemap::EncodeMap, sysutils::schemaurl::SchemaURL};

use super::indexmap::IndexMap;



impl IndexMap {
    pub fn get_positions(&self, batch_id : &u64, take : &u64) -> Vec<(u64,PathBuf)> {
        (0..*take).filter(|x| (*x as u64)   <= self.get_virtual_n_samples() / BATCH_SIZE +1)
        .into_iter()
            .map(|shift| {
                let mut colec_path = self.path.clone();
                let future_id = batch_id+shift as u64;
                colec_path.set_extension("");
                (
                    *self.pos.get(&(future_id)).unwrap(),
                    SchemaURL::get_bloc_path(
                        &self.path.parent().unwrap().file_name().unwrap().to_os_string(), 
                        colec_path.file_name().unwrap(), 
                        &OsString::from(((0..(self.get_virtual_n_samples() / MAX_LINE_BLOC +1))
                        .into_iter()
                        .find(|x| x*MAX_LINE_BLOC <= (future_id-1)*BATCH_SIZE && (future_id-1)*BATCH_SIZE< (x+1)*MAX_LINE_BLOC)
                        .expect(&format!("Bloc name not found for the given batch {}",&future_id)) +1).to_string())
                    )
                )
            }
        )
        .collect::<Vec<(u64,PathBuf)>>()
    }



    pub fn get_data(&self, batch_id : &u64, take : &u64) -> Vec<TsPoint> {
        let positions = self.get_positions(batch_id,take);
            let mut uncompress_data : Vec<u8> = Vec::new();
            positions.iter().for_each(|(position_file, path_to_blocfile)| {
                let readbuffer = &mut BufReader::new(File::open(path_to_blocfile).unwrap());
                readbuffer.seek(SeekFrom::Start(*position_file)).unwrap();
                let mut count = 0;
                for byte in readbuffer.bytes() {

                    match EncodeMap::uncompress_simple(&byte.expect("Could not load byte")) {
                        (left,_) if left == EncodeMap::char_to_bytes(&'*').unwrap() => {
                            uncompress_data.push(left);
                            count+=1;
                            if count>=BATCH_SIZE { break;}
                        }
                        (left,right) if right == EncodeMap::char_to_bytes(&'*').unwrap() => {
                            uncompress_data.push(left);
                            uncompress_data.push(right);
                            count+=1;
                            if count>=BATCH_SIZE { break;}
                        }
                        (left,right) => {
                            uncompress_data.push(left);
                            uncompress_data.push(right);
                        }
                    }
                }
            });

            EncodeMap::serialize(&uncompress_data).iter().enumerate().map(|(i,x)| {
                TsPoint {
                    t: (i as u64 +1)*DEFAULT_STEP + (batch_id-1)*BATCH_SIZE*DEFAULT_STEP,
                    data :x.parse()
                }
            }).collect::<Vec<TsPoint>>()


    }
       
}
   

// pub fn readAllPoints(&self, bloc : &str) -> Vec<TsPointData> {
//     EncodeMap::decode(&BinaryChannel::file_to_bytes(&self.path.join(bloc))).iter().map(|x| x.parse()).collect()
//   }



    // pub fn insertIndex(&mut self, point : &TsPoint, path : &Path) -> bool {
    //     if (self.maxTs != 0 && self.minTs !=0) && (self.maxTs!=self.minTs){
    //         self.pos.insert(point.t,BinaryChannel::AppendLinecompressedToFile(path, &EncodeMap::deserialize(&vec![&point.data])));
    //         return true
    //     } false
    // }


    // pub fn getPosition(&self, batch_id : u64) -> (PathBuf,&u64) {
    //     let nVirtualSamp = self.get_virtual_n_samples();
    //     let nBatchs = nVirtualSamp/BATCH_SIZE +1;
    //     let mut askedBatchId : u64 = 0;
    //     if batch_id> nBatchs { askedBatchId = nBatchs;} else if batch_id<1 { askedBatchId = 1;} else { askedBatchId = batch_id;}
        
    //     let bloc_name = (1..(nVirtualSamp / MAX_LINE_BLOC +1))
    //                         .into_iter()
    //                         .find(|x| (x-1)*MAX_LINE_BLOC <= askedBatchId*BATCH_SIZE && askedBatchId*BATCH_SIZE< x*MAX_LINE_BLOC)
    //                         .expect(&format!("Bloc name not found for the given batch {}",&askedBatchId));
    //     let mut collection = &mut self.path.clone();
    //     collection.set_extension("");

    //     (
    //         SchemaURL::get_bloc_path(
    //             &self.path.parent().unwrap().file_name().unwrap().to_os_string(),
    //             collection.file_name().unwrap(), &OsString::from(bloc_name.to_string())),
        
    //         self.pos.get(&askedBatchId).expect("Batch ID is not referenced in the BTreeMap")
    //     )



        
    // }


    // pub fn get_positions(&self, batch_id_start : u64, take : u64) -> Vec<(PathBuf,&u64)> {
    //     let nVirtualSamp = self.get_virtual_n_samples();
    //     let nBatchs = nVirtualSamp/BATCH_SIZE +1;
    //     let mut askedBatchIds : Vec<u64> = Vec::new();
    //     let mut res : Vec<(PathBuf,&u64)> = Vec::new();

    //     (batch_id_start..(batch_id_start+take)).for_each(|batch_id|
    //         if batch_id> nBatchs { askedBatchIds.push(nBatchs)} else if batch_id<1 { askedBatchIds.push(1)} else { askedBatchIds.push(batch_id)}
    //     );
        
    //     for askedBatchId in askedBatchIds.iter() {
    //         let bloc_name = (1..(nVirtualSamp / MAX_LINE_BLOC +1))
    //                         .into_iter()
    //                         .find(|x| {
    //                             (x-1)*MAX_LINE_BLOC <= (askedBatchId-1)*BATCH_SIZE && (askedBatchId-1)*BATCH_SIZE< x*MAX_LINE_BLOC
    //                         })
    //                         .expect(&format!("Bloc name not found for the given batch {}",&askedBatchId));
    //         let mut collection = &mut self.path.clone();
    //         collection.set_extension("");

    //         res.push((
    //             SchemaURL::get_bloc_path(
    //                 &self.path.parent().unwrap().file_name().unwrap().to_os_string(),
    //                 collection.file_name().unwrap(), &OsString::from(bloc_name.to_string())),
            
    //             self.pos.get(&askedBatchId).expect("Batch ID is not referenced in the BTreeMap")
    //         ));
    //     }
        
    //     res



        
    // }


    // pub fn get_positionsGold(&self, batch_id_start : u64, take : u64) -> Vec<(PathBuf,&u64)> {
    //     let nVirtualSamp = self.get_virtual_n_samples();
    //     let nBatchs = nVirtualSamp/BATCH_SIZE +1;
    //     let mut askedBatchIds : Vec<u64> = Vec::new();
    //     let mut res : Vec<(PathBuf,&u64)> = Vec::new();
    //     (batch_id_start..(batch_id_start+take)).for_each(|batch_id|
    //         if batch_id> nBatchs { askedBatchIds.push(nBatchs)} else if batch_id<1 { askedBatchIds.push(1)} else { askedBatchIds.push(batch_id)}
    //     );
        
    //     for askedBatchId in askedBatchIds.iter() {
    //     256*askedBatchId

    //     }












    //     for askedBatchId in askedBatchIds.iter() {
    //         let bloc_name = (1..(nVirtualSamp / MAX_LINE_BLOC +1))
    //                         .into_iter()
    //                         .find(|x| {
    //                             (x-1)*MAX_LINE_BLOC <= (askedBatchId-1)*BATCH_SIZE && (askedBatchId-1)*BATCH_SIZE< x*MAX_LINE_BLOC
    //                         })
    //                         .expect(&format!("Bloc name not found for the given batch {}",&askedBatchId));
    //         let mut collection = &mut self.path.clone();
    //         collection.set_extension("");

    //         res.push((
    //             SchemaURL::get_bloc_path(
    //                 &self.path.parent().unwrap().file_name().unwrap().to_os_string(),
    //                 collection.file_name().unwrap(), &OsString::from(bloc_name.to_string())),
            
    //             self.pos.get(&askedBatchId).expect("Batch ID is not referenced in the BTreeMap")
    //         ));
    //     }
        
    //     res



        
    // }

    // pub fn get_dataGold(&self, batch_id : u64, take : u64) -> Vec<TsPoint> {

        // Doit obtenir les positions correspondant au batch id et au take
        // ces positions sont : 

        // let positions : Vec<(PathBuf, &u64)> = self.get_positions(batch_id, take);
        // let mut rawdata_tspoint : Vec<u8> = Vec::new();
        // let mut res : Vec<TsPoint> = Vec::new();
        // let mut path_old = PathBuf::new();
        // let readbuffer = &mut BufReader::new(File::open(&positions[0].0.to_path_buf()).unwrap());
        // for pos in positions.iter() {
        //     *readbuffer = BufReader::new(File::open(&pos.0).unwrap());
            
        //     //path_old=pos.0.clone();

        //     readbuffer.seek(SeekFrom::Start(*pos.1)).unwrap();
        //     let mut isSeparatorFollowed : bool = false;
        //     for (i,byte) in readbuffer.bytes().enumerate() {
        //         match (EncodeMap::SeparatorcompressedDetection(&byte, &mut isSeparatorFollowed)){
        //           true => {
        //             if rawdata_tspoint.len() > 0 {
        //                 if isSeparatorFollowed {
        //                     rawdata_tspoint.push(EncodeMap::uncompress_simple(&byte.unwrap()).0);       
        //                     isSeparatorFollowed = false;                                             
        //                 }
        //                 else {
        //                     EncodeMap::uncompress(&mut rawdata_tspoint, &byte.unwrap());
        //                 }  

        
        //                 res.push(TsPoint {
        //                     t: (batch_id*BATCH_SIZE*DEFAULT_STEP + (res.len() as u64))*DEFAULT_STEP,
        //                     data : EncodeMap::Serialize(&rawdata_tspoint)[0].parse()
        //               });
        //             }
        //             rawdata_tspoint = vec![];
        //             if res.len()>=(take*BATCH_SIZE).try_into().unwrap() {break;}

        //             },
        //           false => {
        //             EncodeMap::uncompress(&mut rawdata_tspoint, &byte.unwrap());
        //         },
        //         }


            
        //     }


        // }


       

       
    // res

    // }
