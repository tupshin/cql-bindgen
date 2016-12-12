use std::fmt;
use std::result;

use cassandra::CassError_;

error_chain! {
errors {
    LIB_BAD_PARAMS(t:CassError_){}
    LIB_NO_STREAMS (t:CassError_){}
    LIB_UNABLE_TO_INIT(t:CassError_){}
    LIB_MESSAGE_ENCODE(t:CassError_){}
    LIB_HOST_RESOLUTION(t:CassError_){}
    LIB_UNEXPECTED_RESPONSE(t:CassError_){}
    LIB_REQUEST_QUEUE_FULL(t:CassError_){}
    LIB_NO_AVAILABLE_IO_THREAD(t:CassError_){}
    LIB_WRITE_ERROR(t:CassError_){}
    LIB_NO_HOSTS_AVAILABLE(t:CassError_){}
    LIB_INDEX_OUT_OF_BOUNDS(t:CassError_){}
    LIB_INVALID_ITEM_COUNT(t:CassError_){}
    LIB_INVALID_VALUE_TYPE(t:CassError_){}
    LIB_REQUEST_TIMED_OUT(t:CassError_){}
    LIB_UNABLE_TO_SET_KEYSPACE(t:CassError_){}
    LIB_CALLBACK_ALREADY_SET(t:CassError_){}
    LIB_INVALID_STATEMENT_TYPE(t:CassError_){}
    LIB_NAME_DOES_NOT_EXIST(t:CassError_){}
    LIB_UNABLE_TO_DETERMINE_PROTOCOL(t:CassError_){}
    LIB_NULL_VALUE(t:CassError_){}
    LIB_NOT_IMPLEMENTED(t:CassError_){}
    LIB_UNABLE_TO_CONNECT(t:CassError_){}
    LIB_UNABLE_TO_CLOSE(t:CassError_){}
    LIB_NO_PAGING_STATE(t:CassError_){}
    LIB_PARAMETER_UNSET(t:CassError_){}
    LIB_INVALID_ERROR_RESULT_TYPE(t:CassError_){}
    LIB_INVALID_FUTURE_TYPE(t:CassError_){}
    LIB_INTERNAL_ERROR(t:CassError_){}
    LIB_INVALID_CUSTOM_TYPE(t:CassError_){}
    LIB_INVALID_DATA(t:CassError_){}
    LIB_NOT_ENOUGH_DATA(t:CassError_){}
    LIB_INVALID_STATE(t:CassError_){}
    LIB_NO_CUSTOM_PAYLOAD(t:CassError_){}
    SERVER_SERVER_ERROR(t:CassError_){}
    SERVER_PROTOCOL_ERROR(t:CassError_){}
    SERVER_BAD_CREDENTIALS(t:CassError_){}
    SERVER_UNAVAILABLE(t:CassError_){}
    SERVER_OVERLOADED(t:CassError_){}
    SERVER_IS_BOOTSTRAPPING(t:CassError_){}
    SERVER_TRUNCATE_ERROR(t:CassError_){}
    SERVER_WRITE_TIMEOUT(t:CassError_){}
    SERVER_READ_TIMEOUT(t:CassError_){}
    SERVER_READ_FAILURE(t:CassError_){}
    SERVER_FUNCTION_FAILURE(t:CassError_){}
    SERVER_WRITE_FAILURE(t:CassError_){}
    SERVER_SYNTAX_ERROR(t:CassError_){}
    SERVER_UNAUTHORIZED(t:CassError_){}
    SERVER_INVALID_QUERY(t:CassError_){}
    SERVER_CONFIG_ERROR(t:CassError_){}
    SERVER_ALREADY_EXISTS(t:CassError_){}
    SERVER_UNPREPARED(t:CassError_){}
    SSL_INVALID_CERT(t:CassError_){}
    SSL_INVALID_PRIVATE_KEY(t:CassError_){}
    SSL_NO_PEER_CERT(t:CassError_){}
    SSL_INVALID_PEER_CERT(t:CassError_){}
    SSL_IDENTITY_MISMATCH(t:CassError_){}
    SSL_PROTOCOL_ERROR(t:CassError_){}
    LAST_ENTRY(t:CassError_){}
    }
}

impl ::std::error::Error for CassError_ {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl fmt::Display for CassError_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl CassError_ {
    pub fn to_result<T>(self, t: T) -> Result<T> {
        match self {
            CassError_::CASS_OK => Ok(t),
            CassError_::CASS_ERROR_LIB_BAD_PARAMS => {
                Err(ErrorKind::LIB_BAD_PARAMS(CassError_::CASS_ERROR_LIB_BAD_PARAMS).into())
            }
            CassError_::CASS_ERROR_LIB_NO_STREAMS => {
                Err(ErrorKind::LIB_NO_STREAMS(CassError_::CASS_ERROR_LIB_NO_STREAMS).into())
            }
            CassError_::CASS_ERROR_LIB_UNABLE_TO_INIT => {
                Err(ErrorKind::LIB_UNABLE_TO_INIT(CassError_::CASS_ERROR_LIB_UNABLE_TO_INIT).into())
            }
            CassError_::CASS_ERROR_LIB_MESSAGE_ENCODE => {
                Err(ErrorKind::LIB_MESSAGE_ENCODE(CassError_::CASS_ERROR_LIB_MESSAGE_ENCODE).into())
            }
            CassError_::CASS_ERROR_LIB_HOST_RESOLUTION => {
                Err(ErrorKind::LIB_HOST_RESOLUTION(CassError_::CASS_ERROR_LIB_HOST_RESOLUTION).into())
            }
            CassError_::CASS_ERROR_LIB_UNEXPECTED_RESPONSE => {
                Err(ErrorKind::LIB_UNEXPECTED_RESPONSE(CassError_::CASS_ERROR_LIB_UNEXPECTED_RESPONSE).into())
            }
            CassError_::CASS_ERROR_LIB_REQUEST_QUEUE_FULL => {
                Err(ErrorKind::LIB_REQUEST_QUEUE_FULL(CassError_::CASS_ERROR_LIB_UNEXPECTED_RESPONSE).into())
            }
            CassError_::CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD => {
                Err(ErrorKind::LIB_NO_AVAILABLE_IO_THREAD(CassError_::CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD).into())
            }
            CassError_::CASS_ERROR_LIB_WRITE_ERROR => {
                Err(ErrorKind::LIB_WRITE_ERROR(CassError_::CASS_ERROR_LIB_WRITE_ERROR).into())
            }
            CassError_::CASS_ERROR_LIB_NO_HOSTS_AVAILABLE => {
                Err(ErrorKind::LIB_NO_HOSTS_AVAILABLE(CassError_::CASS_ERROR_LIB_NO_HOSTS_AVAILABLE).into())
            }
            CassError_::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS => {
                Err(ErrorKind::LIB_INDEX_OUT_OF_BOUNDS(CassError_::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_ITEM_COUNT => {
                Err(ErrorKind::LIB_INVALID_ITEM_COUNT(CassError_::CASS_ERROR_LIB_INVALID_ITEM_COUNT).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_VALUE_TYPE => {
                Err(ErrorKind::LIB_INVALID_VALUE_TYPE(CassError_::CASS_ERROR_LIB_INVALID_VALUE_TYPE).into())
            }
            CassError_::CASS_ERROR_LIB_REQUEST_TIMED_OUT => {
                Err(ErrorKind::LIB_REQUEST_TIMED_OUT(CassError_::CASS_ERROR_LIB_REQUEST_TIMED_OUT).into())
            }
            CassError_::CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE => {
                Err(ErrorKind::LIB_UNABLE_TO_SET_KEYSPACE(CassError_::CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE).into())
            }
            CassError_::CASS_ERROR_LIB_CALLBACK_ALREADY_SET => {
                Err(ErrorKind::LIB_CALLBACK_ALREADY_SET(CassError_::CASS_ERROR_LIB_CALLBACK_ALREADY_SET).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_STATEMENT_TYPE => {
                Err(ErrorKind::LIB_INVALID_STATEMENT_TYPE(CassError_::CASS_ERROR_LIB_INVALID_STATEMENT_TYPE).into())
            }
            CassError_::CASS_ERROR_LIB_NAME_DOES_NOT_EXIST => {
                Err(ErrorKind::LIB_NAME_DOES_NOT_EXIST(CassError_::CASS_ERROR_LIB_NAME_DOES_NOT_EXIST).into())
            }
            CassError_::CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL => {
                Err(ErrorKind::LIB_UNABLE_TO_DETERMINE_PROTOCOL(CassError_::CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL).into())
            }
            CassError_::CASS_ERROR_LIB_NULL_VALUE => {
                Err(ErrorKind::LIB_NULL_VALUE(CassError_::CASS_ERROR_LIB_NULL_VALUE).into())
            }
            CassError_::CASS_ERROR_LIB_NOT_IMPLEMENTED => {
                Err(ErrorKind::LIB_NOT_IMPLEMENTED(CassError_::CASS_ERROR_LIB_NOT_IMPLEMENTED).into())
            }
            CassError_::CASS_ERROR_LIB_UNABLE_TO_CONNECT => {
                Err(ErrorKind::LIB_UNABLE_TO_CONNECT(CassError_::CASS_ERROR_LIB_UNABLE_TO_CONNECT).into())
            }
            CassError_::CASS_ERROR_LIB_UNABLE_TO_CLOSE => {
                Err(ErrorKind::LIB_UNABLE_TO_CLOSE(CassError_::CASS_ERROR_LIB_UNABLE_TO_CLOSE).into())
            }
            CassError_::CASS_ERROR_LIB_NO_PAGING_STATE => {
                Err(ErrorKind::LIB_NO_PAGING_STATE(CassError_::CASS_ERROR_LIB_NO_PAGING_STATE).into())
            }
            CassError_::CASS_ERROR_LIB_PARAMETER_UNSET => {
                Err(ErrorKind::LIB_PARAMETER_UNSET(CassError_::CASS_ERROR_LIB_PARAMETER_UNSET).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE => {
                Err(ErrorKind::LIB_INVALID_ERROR_RESULT_TYPE(CassError_::CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE)
                    .into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_FUTURE_TYPE => {
                Err(ErrorKind::LIB_INVALID_FUTURE_TYPE(CassError_::CASS_ERROR_LIB_INVALID_FUTURE_TYPE).into())
            }
            CassError_::CASS_ERROR_LIB_INTERNAL_ERROR => {
                Err(ErrorKind::LIB_INTERNAL_ERROR(CassError_::CASS_ERROR_LIB_INTERNAL_ERROR).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_CUSTOM_TYPE => {
                Err(ErrorKind::LIB_INVALID_CUSTOM_TYPE(CassError_::CASS_ERROR_LIB_INVALID_CUSTOM_TYPE).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_DATA => {
                Err(ErrorKind::LIB_INVALID_DATA(CassError_::CASS_ERROR_LIB_INVALID_DATA).into())
            }
            CassError_::CASS_ERROR_LIB_NOT_ENOUGH_DATA => {
                Err(ErrorKind::LIB_NOT_ENOUGH_DATA(CassError_::CASS_ERROR_LIB_NOT_ENOUGH_DATA).into())
            }
            CassError_::CASS_ERROR_LIB_INVALID_STATE => {
                Err(ErrorKind::LIB_INVALID_STATE(CassError_::CASS_ERROR_LIB_INVALID_STATE).into())
            }
            CassError_::CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD => {
                Err(ErrorKind::LIB_NO_CUSTOM_PAYLOAD(CassError_::CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD).into())
            }
            CassError_::CASS_ERROR_SERVER_SERVER_ERROR => {
                Err(ErrorKind::SERVER_SERVER_ERROR(CassError_::CASS_ERROR_SERVER_SERVER_ERROR).into())
            }
            CassError_::CASS_ERROR_SERVER_PROTOCOL_ERROR => {
                Err(ErrorKind::SERVER_PROTOCOL_ERROR(CassError_::CASS_ERROR_SERVER_PROTOCOL_ERROR).into())
            }
            CassError_::CASS_ERROR_SERVER_BAD_CREDENTIALS => {
                Err(ErrorKind::SERVER_BAD_CREDENTIALS(CassError_::CASS_ERROR_SERVER_BAD_CREDENTIALS).into())
            }
            CassError_::CASS_ERROR_SERVER_UNAVAILABLE => {
                Err(ErrorKind::SERVER_UNAVAILABLE(CassError_::CASS_ERROR_SERVER_UNAVAILABLE).into())
            }
            CassError_::CASS_ERROR_SERVER_OVERLOADED => {
                Err(ErrorKind::SERVER_OVERLOADED(CassError_::CASS_ERROR_SERVER_OVERLOADED).into())
            }
            CassError_::CASS_ERROR_SERVER_IS_BOOTSTRAPPING => {
                Err(ErrorKind::SERVER_IS_BOOTSTRAPPING(CassError_::CASS_ERROR_SERVER_IS_BOOTSTRAPPING).into())
            }
            CassError_::CASS_ERROR_SERVER_TRUNCATE_ERROR => {
                Err(ErrorKind::SERVER_TRUNCATE_ERROR(CassError_::CASS_ERROR_SERVER_TRUNCATE_ERROR).into())
            }
            CassError_::CASS_ERROR_SERVER_WRITE_TIMEOUT => {
                Err(ErrorKind::SERVER_WRITE_TIMEOUT(CassError_::CASS_ERROR_SERVER_WRITE_TIMEOUT).into())
            }
            CassError_::CASS_ERROR_SERVER_READ_TIMEOUT => {
                Err(ErrorKind::SERVER_READ_TIMEOUT(CassError_::CASS_ERROR_SERVER_READ_TIMEOUT).into())
            }
            CassError_::CASS_ERROR_SERVER_READ_FAILURE => {
                Err(ErrorKind::SERVER_READ_FAILURE(CassError_::CASS_ERROR_SERVER_READ_FAILURE).into())
            }
            CassError_::CASS_ERROR_SERVER_FUNCTION_FAILURE => {
                Err(ErrorKind::SERVER_FUNCTION_FAILURE(CassError_::CASS_ERROR_SERVER_FUNCTION_FAILURE).into())
            }
            CassError_::CASS_ERROR_SERVER_WRITE_FAILURE => {
                Err(ErrorKind::SERVER_WRITE_FAILURE(CassError_::CASS_ERROR_SERVER_WRITE_FAILURE).into())
            }
            CassError_::CASS_ERROR_SERVER_SYNTAX_ERROR => {
                Err(ErrorKind::SERVER_SYNTAX_ERROR(CassError_::CASS_ERROR_SERVER_WRITE_FAILURE).into())
            }
            CassError_::CASS_ERROR_SERVER_UNAUTHORIZED => {
                Err(ErrorKind::SERVER_UNAUTHORIZED(CassError_::CASS_ERROR_SERVER_UNAUTHORIZED).into())
            }
            CassError_::CASS_ERROR_SERVER_INVALID_QUERY => {
                Err(ErrorKind::SERVER_INVALID_QUERY(CassError_::CASS_ERROR_SERVER_INVALID_QUERY).into())
            }
            CassError_::CASS_ERROR_SERVER_CONFIG_ERROR => {
                Err(ErrorKind::SERVER_CONFIG_ERROR(CassError_::CASS_ERROR_SERVER_CONFIG_ERROR).into())
            }
            CassError_::CASS_ERROR_SERVER_ALREADY_EXISTS => {
                Err(ErrorKind::SERVER_ALREADY_EXISTS(CassError_::CASS_ERROR_SERVER_ALREADY_EXISTS).into())
            }
            CassError_::CASS_ERROR_SERVER_UNPREPARED => {
                Err(ErrorKind::SERVER_UNPREPARED(CassError_::CASS_ERROR_SERVER_UNPREPARED).into())
            }
            CassError_::CASS_ERROR_SSL_INVALID_CERT => {
                Err(ErrorKind::SSL_INVALID_CERT(CassError_::CASS_ERROR_SSL_INVALID_CERT).into())
            }
            CassError_::CASS_ERROR_SSL_INVALID_PRIVATE_KEY => {
                Err(ErrorKind::SSL_INVALID_PRIVATE_KEY(CassError_::CASS_ERROR_SSL_INVALID_PRIVATE_KEY).into())
            }
            CassError_::CASS_ERROR_SSL_NO_PEER_CERT => {
                Err(ErrorKind::SSL_NO_PEER_CERT(CassError_::CASS_ERROR_SSL_NO_PEER_CERT).into())
            }
            CassError_::CASS_ERROR_SSL_INVALID_PEER_CERT => {
                Err(ErrorKind::SSL_INVALID_PEER_CERT(CassError_::CASS_ERROR_SSL_INVALID_PEER_CERT).into())
            }
            CassError_::CASS_ERROR_SSL_IDENTITY_MISMATCH => {
                Err(ErrorKind::SSL_IDENTITY_MISMATCH(CassError_::CASS_ERROR_SSL_IDENTITY_MISMATCH).into())
            }
            CassError_::CASS_ERROR_SSL_PROTOCOL_ERROR => {
                Err(ErrorKind::SSL_PROTOCOL_ERROR(CassError_::CASS_ERROR_SSL_PROTOCOL_ERROR).into())
            }
            CassError_::CASS_ERROR_LAST_ENTRY => Err(ErrorKind::LAST_ENTRY(CassError_::CASS_ERROR_LAST_ENTRY).into()),
        }
    }
}
