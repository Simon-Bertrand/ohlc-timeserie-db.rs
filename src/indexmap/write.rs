
use std::{ffi::OsString, fs::File, collections::BTreeMap};


use crate::{schemaurl::SchemaURL, DEFAULT_STEP,BATCH_SIZE, MAX_LINE_BLOC, channel::Channel, tspoint::TsPoint, encodemap::EncodeMap};

use super::{AppendIndexAnswerKind, IndexMap};


impl IndexMap {
    pub fn saves_change(&self) {
        Channel::write_to_file(&self.path,&Channel::indexmap_to_bytes(self));
    }
    

    pub fn dyn_insert_index_and_data(&mut self, file : &mut File,  point : &TsPoint, insert_pos : &bool) -> bool {
        let n_samples =(self.maxts-self.mints)/self.step;
        let inserted_sample_position = Channel::append_linecompressed_to_dynfile(file, &EncodeMap::deserialize(&vec![&point.data]));
        if n_samples  % BATCH_SIZE == 0 && *insert_pos {
            self.pos.insert(n_samples/BATCH_SIZE +1,inserted_sample_position);
        }
        return true
    }

    pub fn create(source: &OsString, collection : &OsString) -> Option<IndexMap> {
       if !SchemaURL::get_indexmap_path(source,collection).exists() {
            let path = SchemaURL::get_indexmap_path(source,collection);
            File::create(&path).unwrap();
            let im = IndexMap { mints : 0, maxts : 0, step : DEFAULT_STEP, pos : BTreeMap::new(), path : path.to_owned(), name : collection.to_owned()};
            Channel::write_to_file(&path,&Channel::indexmap_to_bytes(&im) );
            Some(im)
       } else {return None}
    }


    pub fn append_index(&mut self, future_ts : &u64, creating_bloc : &mut bool) -> AppendIndexAnswerKind {
        if *future_ts == 0 {
            return AppendIndexAnswerKind::RbFirstTsCannotBeZero
        } else {
            if self.maxts==0 && self.mints==0 {
                self.mints = future_ts.clone();
                self.maxts = *future_ts;
                AppendIndexAnswerKind::Init
            } 
            else if self.maxts == self.mints {
                self.maxts = *future_ts;
                AppendIndexAnswerKind::NewBloc
            }
            else if self.maxts + self.step == *future_ts {
                if (future_ts-self.mints)/self.step % MAX_LINE_BLOC == 0 && !*creating_bloc {
                    return AppendIndexAnswerKind::BlocIsFull
                } else {
                    *creating_bloc=false;
                    self.maxts = *future_ts;
                    AppendIndexAnswerKind::Continue 
                }
            }
            else { 
                AppendIndexAnswerKind::RbInvalidNextTs
            }
        }
    }
}
   
