use crate::collection::Collection;

// pub struct IndexResult {
//     pub start_batchid : u64,
//     pub batchid_shift : u64,
//     pub rows_take : u64,
//     pub colec : &'static Collection
// }

#[derive(Debug)]
pub struct IndexResult<'a> {
    pub start_batchid : u64,
    pub start_ind : u64,
    pub end_ind : u64,
    pub end_batchid : u64,
    pub colec : &'a Collection
}
impl<'a> IndexResult<'a> {
    pub fn create(start_batchid: &u64,  start_shift: &u64,  end_indice: &u64, end_batchid : &u64, colec: &'a Collection) -> IndexResult<'a> {
        IndexResult { start_batchid: *start_batchid, start_ind: *start_shift, end_ind : *end_indice, end_batchid: *end_batchid, colec:&colec}
    }
}

// impl IndexResult {
//     pub fn create(start_batchid: u64, bloc_rows_shift: u64, rows_take: u64, colec: &'static Collection) -> IndexResult {
//         IndexResult { start_batchid: start_batchid, batchid_shift: bloc_rows_shift, rows_take : rows_take, colec: colec}
//     }
// }