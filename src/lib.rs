#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![recursion_limit = "1024"]
// #![cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use]
extern crate log;
extern crate libc;
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}

mod error;


pub use ffi_util::*;

pub use cassandra::cass_bool_t;
pub use cassandra::cass_bool_t::{cass_true, cass_false};

pub use cassandra::CassError;
pub use cassandra::CassError_::*;
pub use cassandra::CassSslVerifyFlags::*;
pub use cassandra::CassBatchType_::*;
pub use cassandra::CassLogLevel_::CASS_LOG_INFO;
pub use cassandra::CassValueType_::*;
pub use cassandra::CassCollectionType_::{CASS_COLLECTION_TYPE_SET, CASS_COLLECTION_TYPE_LIST, CASS_COLLECTION_TYPE_MAP};

// pub use cassandra::ffi_util::raw2utf8;

mod cassandra;
pub mod ffi_util;
