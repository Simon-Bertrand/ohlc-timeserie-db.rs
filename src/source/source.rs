use core::fmt;
use std::{ffi::OsString, collections::HashMap, fs, ops::Index};

use crate::{collection::Collection, schemaurl::SchemaURL};



impl Source {
    pub fn create(source : &OsString) -> Option<Source> {
        let (source_exists,_, _) = SchemaURL::get_existing_paths(source, &OsString::new(), &OsString::new());
        if !source_exists {
            fs::create_dir(SchemaURL::get_source_path(source))
            .expect("Could not create Source directory");
            Some(Source { name : source.to_owned() , colecs : HashMap::new()})
        }
    
        else {return None}


    }
    pub fn delete(&self) {
        fs::remove_dir_all(SchemaURL::get_source_path(&self.name)).expect("Could not delete Source directory");
    }
}


#[derive(Debug)]
pub struct Source {
    pub name : OsString,
    pub colecs : HashMap<OsString, Collection>, 
}
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Source name {:?} - Collections<{}>", self.name, self.colecs.len())
    }
}


impl Index<&str> for Source {
    type Output = Collection;
    fn index<'a>(&'a self, collection: &str) -> &'a Collection {
        self.colecs.get(&OsString::from(collection)).unwrap()
        
    }
}


impl Source {
    pub fn get(&self, collection : &str) -> Option<&Collection> {
        self.colecs.get(&OsString::from(collection))
    }
    pub fn get_mut(&mut self, collection : &str) -> Option<&mut Collection> {
        self.colecs.get_mut(&OsString::from(collection))
    }



}
