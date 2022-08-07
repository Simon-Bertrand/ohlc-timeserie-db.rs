use std::{ffi::OsString, collections::HashMap};


use crate::{datastruct::{source::Source, collection::Collection, bloc::Bloc, indexmap::IndexMap}, sysutils::schemaurl::SchemaURL, channels::binarychannel::BinaryChannel};

use super::System;



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
        let bytes = BinaryChannel::file_to_bytes(&SchemaURL::get_indexmap_path(source, collection));
        BinaryChannel::bytes_to_indexmap(&bytes)
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


