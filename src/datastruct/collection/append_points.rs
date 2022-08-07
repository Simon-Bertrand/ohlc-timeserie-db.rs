use std::fs::{OpenOptions};
use std::io::{Error, ErrorKind};
use std::{ffi::OsString};


use crate::datastruct::bloc::Bloc;
use crate::datastruct::indexmap::indexmap::AppendIndexAnswerKind;
use crate::datastruct::tspoint::TsPoint;

use super::Collection;
impl Collection {
    pub fn append_points(&mut self, points : &[TsPoint], create_bloc : &mut bool, inserted_count : &mut u32) -> Result<(), Error> {
        if points.len() == 0 { return Ok(());}
        let bloc_number = self.get_max_bloc_number();
        let mut file = OpenOptions::new().append(true).open(&self.path.join(&bloc_number)).expect("Cannot open file");
        points.iter().position(|x| x.t == self.map.maxts + self.map.step )
                    .expect(&format!("append_points - No sample has the future required timestamp {}", self.map.maxts + self.map.step));
        
        for (i,point) in points.iter().enumerate() {
            match self.map.append_index(&point.t, create_bloc) {
                AppendIndexAnswerKind::Continue => {

                    match self.map.dyn_insert_index_and_data(&mut file, &point,  &true) {
                        true => {*inserted_count+=1;}, _ => ()
                    };
                },
                AppendIndexAnswerKind::NewBloc => {
                    match self.map.dyn_insert_index_and_data(&mut file, &point, &false) {
                        true => {*inserted_count+=1;}, _ => ()
                    };
                },
                AppendIndexAnswerKind::Init => {
                    match self.map.dyn_insert_index_and_data(&mut file, &point,  &true) {
                        true => {*inserted_count+=1;}, _ => ()
                    };
                },
                AppendIndexAnswerKind::BlocIsFull => {
                    let new_bloc_number = OsString::from(((&bloc_number.parse::<u64>().expect("append_points - Invalid Bloc File number name parse") + 1)).to_string());     
                    self.blocs.insert(new_bloc_number.to_owned(),
                    Bloc::create(&self.path.parent().unwrap().file_name().expect("append_points - Self path is missing a source").to_owned(),
                                    &self.path.file_name().expect("append_points - Self path is missing").to_owned(),
                                    &new_bloc_number.to_owned()).unwrap());
                    *create_bloc=true;
                    return self.append_points(&points[i..], create_bloc, inserted_count)
                },

                AppendIndexAnswerKind::RbInvalidNextTs => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("IndexMap Rule breaking : Given timestamp is not equal to maxTs + step | Var: {} (s) | Future : {} Waited : {}", self.map.maxts + self.map.step - &point.t,&point.t, self.map.maxts + self.map.step )))
                },
                AppendIndexAnswerKind::RbFirstTsCannotBeZero => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("IndexMap Rule breaking : The first sample timestamp cannot be zero")))
                }
        }
        self.map.saves_change(); 
    }
    Ok(())
}


} 
