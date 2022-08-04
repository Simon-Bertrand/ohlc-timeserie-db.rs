use std::{ops::Index, ffi::OsString, collections::HashMap};
use crate::BATCH_SIZE;
use crate::filemanager::FileManager;
use crate::indexresult::TsIndexConverter;
use crate::inoutputs::InOutputs;
use crate::tspoint::TsPoint;
use crate::{ source::Source, channel::Channel};


use crate::{schemaurl::SchemaURL, bloc::Bloc, collection::Collection,indexmap::IndexMap};


#[derive(Debug)]
pub struct System {
    pub files : FileManager,
    pub sources : HashMap<OsString, Source>
}


impl System {
    pub fn instanciate() -> Self {
        let mut sys = System {
            files : FileManager { schema : FileManager::scan_db_repo() },
            sources : HashMap::new()
        };
        sys.get_all_sources();
        sys
    }
}

impl System {
    pub fn main() {
     let mut sys = System::instanciate();
     let mut _i =0;
     let _b : &mut Collection =  sys.sources.get_mut(&OsString::from("Binance")).unwrap().colecs.get_mut(&OsString::from("BTCUSDT")).unwrap();
    }

}


impl System {
    fn inject_data_core<T>(&mut self, source : &str, collection : &str, data: T, parser : fn(T) -> Vec<TsPoint>) {
        let mut inserted_counts = 0;
        let c = self.sources.get_mut(&OsString::from(source)).expect("Source does not exist").get_mut(collection).expect("Collection does not exist");
        c.append_points(&parser(data), &mut false, &mut inserted_counts).expect("Adding points failed");
        println!("Inserted points : {}", inserted_counts);
    }

    pub fn inject_data_from_string(&mut self, source : &str, collection : &str, data: &str) {
        self.inject_data_core(source,collection,data, InOutputs::JsonToTsPoints)
    }
    pub fn inject_data_from_points(&mut self, source : &str, collection : &str, data: Vec<TsPoint>) {
        self.inject_data_core(source,collection,data, |x| x)
    }
} 




impl System {
    pub fn query_data(&'static self, query_string : &str) -> String {
        let index_result = TsIndexConverter::parseQueryString(&self, query_string);
        InOutputs::PointsToJson({
            &index_result
            .colec
            .map
            .get_data(&index_result.start_batchid, &(&index_result.end_batchid - &index_result.start_batchid))[(index_result.start_shift as usize)..(index_result.end_indice as usize)]
        })
    }
}  




impl Index<&str> for System {
    type Output = Source;
    fn index<'a>(&'a self, source: &str) -> &'a Source {
        self.sources.get(&OsString::from(source)).unwrap()
    }
}




impl System {
    pub fn get_all_sources(&mut self) {
        for (source, _hm) in self.files.schema.iter() {
            self.sources.insert(source.to_owned(), Source {
                name :source.to_owned(),
                colecs : self.get_all_collections(source)
            });
        }
    }

    pub fn get_indexmap(&self, source : &OsString, collection : &OsString)-> IndexMap {
        let bytes = Channel::file_to_bytes(&SchemaURL::get_indexmap_path(source, collection));
        Channel::bytes_to_indexmap(&bytes)
    }

    pub fn get_all_collections(&self, source : &OsString) -> HashMap<OsString, Collection> {
     let mut data : HashMap<OsString, Collection> = HashMap::new();
        for (collection, _hm) in self.files.schema.get(source).unwrap().iter() {
            data.insert(collection.to_owned(),Collection {
                    name: collection.to_owned(),
                    map: self.get_indexmap(source, collection),
                    path: SchemaURL::get_collection_path(source, collection),
                    blocs : self.get_all_blocs(source, collection),
            });
        } 
        data
    }

    pub fn get_all_blocs(&self, source : &OsString, collection : &OsString) -> HashMap<OsString, Bloc>{
        let mut data : HashMap<OsString,Bloc> = HashMap::new();
        for bloc in self.files.schema.get(source).unwrap().get(collection).unwrap().iter() {
            data.insert(bloc.to_owned(),Bloc {
                name : bloc.to_owned(),
                path : SchemaURL::get_bloc_path(source, collection, bloc),
                size : 0,
            });
        }
        data
    }

}



    
