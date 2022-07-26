use std::{path::{Path, PathBuf}, ffi::{OsStr,}};

use crate::BASE_URL;


pub struct SchemaURL {
}

impl SchemaURL {
    pub fn get_base_path()-> PathBuf {
        Path::new(BASE_URL).to_path_buf()
    }
    pub fn get_source_path(source: &OsStr)-> PathBuf {
        SchemaURL::get_base_path().join(source)
  
    }
    pub fn get_collection_path(source: &OsStr, pair : &OsStr)-> PathBuf {
        SchemaURL::get_source_path(source).join(pair)
        
    }
    pub fn get_bloc_path(source: &OsStr, pair : &OsStr, bloc : &OsStr)-> PathBuf {
        SchemaURL::get_collection_path(source, pair).join(bloc)
     
    }

    pub fn get_indexmap_path(source: &OsStr, pair : &OsStr)-> PathBuf {
        let mut schema = SchemaURL::get_collection_path(source, pair);
        schema.set_extension("im");
        schema
    }

    pub fn get_existing_paths(source: &OsStr, pair : &OsStr, bloc : &OsStr) -> (bool, bool, bool) {
        return (
            SchemaURL::get_source_path(source).exists(),
            SchemaURL::get_collection_path(source, pair).exists(),
            SchemaURL::get_bloc_path(source, pair, bloc).exists()
        )
    }
}
