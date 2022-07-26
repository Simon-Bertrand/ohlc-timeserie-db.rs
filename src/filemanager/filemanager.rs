use std::{collections::HashMap, ffi::OsString, fs};

use crate::schemaurl::SchemaURL;

#[derive(Debug)]

pub struct FileManager {
    pub schema : HashMap<OsString,HashMap<OsString,Vec<OsString>>>

}
impl FileManager {
    pub fn scan_db_repo() -> HashMap<OsString,HashMap<OsString,Vec<OsString>>>{
        let mut hm : HashMap<OsString,HashMap<OsString,Vec<OsString>>> = HashMap::new();
        let mut _asset = OsString::new(); let mut _source = OsString::new();
        for source_path in fs::read_dir("./db/").unwrap() {
            let mut sub_fields = HashMap::new();
            _source = source_path.as_ref().unwrap().file_name();
            for asset_path in fs::read_dir(&source_path.as_ref().unwrap().path()).unwrap() {
                _asset = asset_path.as_ref().unwrap().file_name();
                if SchemaURL::get_indexmap_path(&_source, &_asset).exists() && asset_path.as_ref().unwrap().path().is_dir() {
                    let mut blocs : Vec<OsString>  = Vec::new();
                    for bloc_path in fs::read_dir(&asset_path.as_ref().unwrap().path()).unwrap() {
                        blocs.push(bloc_path.as_ref().unwrap().file_name());
                    }
                    sub_fields.insert(_asset, blocs);
                }
            }
            hm.insert(_source, sub_fields);
        }
        hm
    }
    pub fn refresh_db_repo(&mut self){
        self.schema = FileManager::scan_db_repo();
    }
}
