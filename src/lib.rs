pub mod bloc;
pub mod collection;
pub mod source;
pub mod indexmap;
pub mod system;
pub mod encodemap;
pub mod filemanager;
pub mod schemaurl;
pub mod tspoint;
pub mod channel;




pub const BASE_URL: &str = "./db/";
pub const DEFAULT_STEP : u64 =  60;
pub const BATCH_SIZE : u64 =  256;
pub const MAX_LINE_BLOC : u64 = 2560;