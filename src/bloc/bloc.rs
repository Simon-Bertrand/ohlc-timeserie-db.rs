use std::{ffi::OsString, path::PathBuf};
use std::fmt;
use std::fs::{File};
use crate::source::Source;
use crate::collection::Collection;
use crate::schemaurl::SchemaURL;

#[derive(Debug)]
pub struct Bloc {
    pub name : OsString,
    pub path : PathBuf,
    pub size : i32,
}

impl Bloc {
    pub fn create(source : &OsString, collection : &OsString, bloc : &OsString) -> Option<Bloc> {
        let (source_exists,collection_exists, bloc_exists) = SchemaURL::get_existing_paths(source, collection, bloc);
        if !source_exists && !collection_exists && !bloc_exists{ Source::create(&source).unwrap();}
        if !collection_exists && !bloc_exists { Collection::create(source, collection) ;}
        if !bloc_exists {
            File::create(SchemaURL::get_bloc_path(source,collection, bloc)).expect("Could not create bloc file");
            Some(Bloc {
                name: bloc.to_owned(),
                path : SchemaURL::get_bloc_path(source,collection, &bloc),
                size:0})
        } else {return None}
    }
}

impl fmt::Display for Bloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bloc name {:?} - Bloc<{}>", self.name, self.size)
    }
}

