use serde::Serialize;

pub const RESP_CODE_INVALID_PARAM: u8 = 0;
pub const RESP_CODE_SUCCESS: u8 = 1;
pub const RESP_CODE_SYNTAX_ERROR: u8 = 2;
pub const RESP_CODE_INVALID_OPTIONS: u8 = 3;
pub const RESP_CODE_UNSUCCESSFUL_EXECUTION: u8 = 4;
pub const RESP_CODE_INTERNAL_ERROR: u8 = 5;
pub const RESP_CODE_EXECUTION_TIMEOUT: u8 = 6;
// pub const RESP_CODE_ENQUEUED: u8 = 7;
// pub const RESP_CODE_NOT_SUPPORTED: u8 = 8;
// pub const RESP_CODE_NOT_FOUND: u8 = 9;

#[derive(Serialize)]
pub struct ResponseOf<T> {
    pub code: u8,
    pub data: Option<T>
}

impl <T> ResponseOf<T> {
    pub fn success(data: T) -> ResponseOf<T> {
        ResponseOf{
            code: RESP_CODE_SUCCESS,
            data: Some(data)
        }
    }

    pub fn error() -> ResponseOf<T> {
        ResponseOf {
            code: RESP_CODE_INTERNAL_ERROR,
            data: None
        }
    }

    pub fn non_success(code: u8) -> ResponseOf<T> {
        ResponseOf {
            code,
            data: None
        }
    }

    // pub fn new(code: u8, data: Option<T>) -> ResponseOf<T> {
    //     ResponseOf {
    //         code,
    //         data
    //     }
    // }
}