use crate::{indexing::aggregator::Aggregator, datastruct::collection::Collection};




pub struct IndexResult<'a> {
    pub start_batchid : u64,
    pub start_ind : u64,
    pub end_ind : u64,
    pub end_batchid : u64,
    pub colec : &'a Collection,
    pub aggregator : Option<Aggregator>
}
impl<'a> IndexResult<'a> {
    pub fn create(start_batchid: &u64,  start_shift: &u64,  end_indice: &u64, end_batchid : &u64, colec: &'a Collection, aggregator : Option<Aggregator>) -> IndexResult<'a> {
        IndexResult { start_batchid: *start_batchid, start_ind: *start_shift, end_ind : *end_indice, end_batchid: *end_batchid, colec:&colec, aggregator:aggregator}
    }
}
