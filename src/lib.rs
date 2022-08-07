pub mod channels;
pub mod compression;
pub mod datastruct;
pub mod indexing;
pub mod system;
pub mod sysutils;



pub const BASE_URL: &str = "./db/";
pub const DEFAULT_STEP : u64 =  60;
pub const BATCH_SIZE : u64 =  256;
pub const MAX_LINE_BLOC : u64 = 2560;