use core::fmt;
use std::fs;
use std::ops::Index;
use std::{ffi::OsString, path::PathBuf, collections::HashMap};
use crate::indexmap::{IndexMap};
use crate::bloc::Bloc;
use crate::schemaurl::SchemaURL;
use crate::source::Source;

#[derive(Debug)]
pub struct Collection {
    pub name : OsString,
    pub map : IndexMap,
    pub path : PathBuf,
    pub blocs : HashMap<OsString, Bloc>,

}


impl Collection {
    pub fn create(source : &OsString, collection : &OsString) -> Option<Collection> {
        let (source_exists,collection_exists, bloc_exists) = SchemaURL::get_existing_paths(source, collection, &OsString::from("1"));
        if !source_exists && !collection_exists && !bloc_exists{Source::create(source);}
        if !source_exists && !collection_exists {
            fs::create_dir(SchemaURL::get_collection_path(source,collection))
            .expect("Could not create Collection directory");
            let mut blocs : HashMap<OsString, Bloc> = HashMap::new();
            let bloc = Bloc::create(source, collection, &OsString::from("1")).expect("Could not create Bloc");
            blocs.insert(OsString::from("1"),bloc);
            Some(Collection {
                name: OsString::from(source),
                map: IndexMap::create(source, collection).expect("IndexMap cannot be created"),
                path: SchemaURL::get_collection_path(source,collection),
                blocs: blocs})
    
        } else {return None}
    }
    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.path).expect("Could not delete the collection");
        let mut im=self.path.clone();
        im.set_extension("im");
        fs::remove_file(im)
    }
}


impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Collection name {:?} - Blocs<{}>", self.name, self.blocs.len())
    }
}

impl Index<&str> for Collection {
    type Output = Bloc;
    fn index<'a>(&'a self, bloc: &str) -> &'a Bloc {
        self.blocs.get(&OsString::from(bloc)).unwrap()
    }
}


impl Collection {
    pub fn get(&self, bloc : &str) -> Option<&Bloc> {self.blocs.get(&OsString::from(bloc))}
    pub fn get_mut(&mut self, bloc : &str) -> Option<&mut Bloc> {self.blocs.get_mut(&OsString::from(bloc))}

    pub fn get_max_bloc_number(&self) -> String {
        self.blocs.iter()
        .map(|(_k,v)| v.name.to_str().unwrap_or("1").parse::<u64>().unwrap_or(1))
        .collect::<Vec<u64>>()
        .iter()
        .max()
        .unwrap_or(&1)
        .to_string()
    }
 

}






    // pub fn appendPoint(&mut self, point : TsPoint, creating_bloc : &mut bool) -> Result<(), std::io::Error> {
    //     let bloc_number = self.get_max_bloc_number() ;
    //     match self.map.append_index(&point.t, creating_bloc) {
    //         Ok(()) => {
    //             self.map.insertIndex(&point, &self.path.join(bloc_number));
    //             Ok(())
    //         },
    //         Err(e) if e.kind() == ErrorKind::OutOfMemory => {
    //             let new_bloc_number = OsString::from(((bloc_number.parse::<i64>().expect("Invalid Bloc File number name parse") + 1)).to_string());     

    //             let source = self.path.parent().unwrap().file_name().unwrap();
    //             let parent = self.path.file_name().unwrap();

  

    //             self.blocs.insert(new_bloc_number.to_owned(),
    //             Bloc::create(&source.to_owned(), &parent.to_owned() ,&new_bloc_number.to_owned()).unwrap());
    //             self.appendPoint(point, &mut true)
    //         },
    //         Err(e) => {return Err(e)},
    //     }
    // }