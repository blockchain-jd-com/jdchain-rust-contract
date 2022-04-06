// Types for JD Chain Contract Data Interaction

use std::ffi::CString;

use serde::{Deserialize, Serialize};

// request types
const LOG: u8 = 0;
const BEFORE_EVENT: u8 = 1;
const POST_EVENT: u8 = 2;
const GET_LEDGER_HASH: u8 = 3;
const GET_CONTRACT_ADDRESS: u8 = 4;
const GET_TX_HASH: u8 = 5;
const GET_TX_TIME: u8 = 6;
const GET_SIGNERS: u8 = 7;
const REGISTER_USER: u8 = 8;
const GET_USER: u8 = 9;
const REGISTER_DATA_ACCOUNT: u8 = 10;
const GET_DATA_ACCOUNT: u8 = 11;
const SET_TEXT: u8 = 12;
const SET_TEXT_WITH_VERSION: u8 = 13;
const SET_INT64: u8 = 14;
const SET_INT64_WITH_VERSION: u8 = 15;
const GET_VALUE_VERSION: u8 = 16;
const GET_VALUE: u8 = 17;

// log level
const LEVEL_DEBUG: u8 = 1;
const LEVEL_INFO: u8 = 2;
const LEVEL_ERROR: u8 = 3;

// value type
// const TEXT: u8 = 1;
// const INT64: u8 = 2;

// 运行时数据交互 - 请求
#[derive(Serialize, Deserialize)]
pub struct Request {
    // 请求类型，与 request types 对应
    rt: u8,
}

// 日志请求
#[derive(Serialize, Deserialize)]
struct LogRequest {
    // 请求类型，LEVEL_INFO
    rt: u8,
    // 等级 LEVEL_DEBUG/LEVEL_INFO/LEVEL_ERROR
    l: u8,
    // 内容
    m: String,
}

// 注册用户请求
#[derive(Serialize, Deserialize)]
struct RegisterUserRequest {
    // 请求类型，REGISTER_USER
    rt: u8,
    // seed
    s: String,
    // algorithm
    a: String,
}

// 查询用户请求
#[derive(Serialize, Deserialize)]
struct GetUserRequest {
    // 请求类型，GET_USER
    rt: u8,
    // address
    a: String,
}

// 注册数据账户请求
#[derive(Serialize, Deserialize)]
struct RegisterDataAccountRequest {
    // 请求类型，REGISTER_DATA_ACCOUNT
    rt: u8,
    // seed
    s: String,
    // algorithm
    a: String,
}

// 查询数据账户请求
#[derive(Serialize, Deserialize)]
struct GetDataAccountRequest {
    // 请求类型，GET_DATA_ACCOUNT
    rt: u8,
    // address
    a: String,
}

// 写KV，字符类型，不带版本
#[derive(Serialize, Deserialize)]
struct SetTextRequest {
    // 请求类型，SET_TEXT
    rt: u8,
    // address
    a: String,
    // key
    k: String,
    // value
    v: String,
}

// 写KV，字符类型
#[derive(Serialize, Deserialize)]
struct SetTextWithVersionRequest {
    // 请求类型，SET_TEXT_WITH_VERSION
    rt: u8,
    // address
    a: String,
    // key
    k: String,
    // value
    v: String,
    // version
    ver: i64,
}

// 写KV，数值类型，不带版本
#[derive(Serialize, Deserialize)]
struct SetInt64Request {
    // 请求类型，SET_INT64
    rt: u8,
    // address
    a: String,
    // key
    k: String,
    // value
    v: i64,
}

// 写KV，数值类型
#[derive(Serialize, Deserialize)]
struct SetInt64WithVersionRequest {
    // 请求类型，SET_INT64_WITH_VERSION
    rt: u8,
    // address
    a: String,
    // key
    k: String,
    // value
    v: i64,
    // version
    ver: i64,
}

// 查询数据版本
#[derive(Serialize, Deserialize)]
struct GetValueVersionRequest {
    // 请求类型，GET_VALUE_VERSION
    rt: u8,
    // address
    a: String,
    // key
    k: String,
}

// 查询数据
#[derive(Serialize, Deserialize)]
struct GetValueRequest {
    // 请求类型，GET_VALUE
    rt: u8,
    // address
    a: String,
    // key
    k: String,
    // version
    ver: i64,
}

// 数据交互请求构建
impl Request {
    pub fn log_debug(msg: String) -> String {
        serde_json::to_string(&LogRequest { rt: LOG, l: LEVEL_DEBUG, m: msg }).unwrap()
    }

    pub fn log_info(msg: String) -> String {
        serde_json::to_string(&LogRequest { rt: LOG, l: LEVEL_INFO, m: msg }).unwrap()
    }

    pub fn log_error(msg: String) -> String {
        serde_json::to_string(&LogRequest { rt: LOG, l: LEVEL_ERROR, m: msg }).unwrap()
    }

    pub fn before_event() -> CString {
        CString::new(serde_json::to_string(&Request { rt: BEFORE_EVENT }).unwrap()).unwrap()
    }

    pub fn post_event() -> CString {
        CString::new(serde_json::to_string(&Request { rt: POST_EVENT }).unwrap()).unwrap()
    }

    pub fn get_ledger_hash() -> String {
        serde_json::to_string(&Request { rt: GET_LEDGER_HASH }).unwrap()
    }
    pub fn get_contract_address() -> String {
        serde_json::to_string(&Request { rt: GET_CONTRACT_ADDRESS }).unwrap()
    }
    pub fn get_tx_hash() -> String {
        serde_json::to_string(&Request { rt: GET_TX_HASH }).unwrap()
    }

    pub fn get_tx_time() -> String {
        serde_json::to_string(&Request { rt: GET_TX_TIME }).unwrap()
    }

    pub fn get_signers() -> String {
        serde_json::to_string(&Request { rt: GET_SIGNERS }).unwrap()
    }

    pub fn register_user(seed: String) -> String {
        serde_json::to_string(&RegisterUserRequest { rt: REGISTER_USER, s: seed, a: "ED25519".to_string() }).unwrap()
    }
    pub fn get_user(address: String) -> String {
        serde_json::to_string(&GetUserRequest { rt: GET_USER, a: address }).unwrap()
    }
    pub fn register_data_account(seed: String) -> String {
        serde_json::to_string(&RegisterDataAccountRequest { rt: REGISTER_DATA_ACCOUNT, s: seed, a: "ED25519".to_string() }).unwrap()
    }
    pub fn get_data_account(address: String) -> String {
        serde_json::to_string(&GetDataAccountRequest { rt: GET_DATA_ACCOUNT, a: address }).unwrap()
    }
    pub fn set_text(address: String, key: String, value: String) -> String {
        serde_json::to_string(&SetTextRequest { rt: SET_TEXT, a: address, k: key, v: value }).unwrap()
    }
    pub fn set_text_with_version(address: String, key: String, value: String, version: i64) -> String {
        serde_json::to_string(&SetTextWithVersionRequest { rt: SET_TEXT_WITH_VERSION, a: address, k: key, v: value, ver: version }).unwrap()
    }
    pub fn set_int64(address: String, key: String, value: i64) -> String {
        serde_json::to_string(&SetInt64Request { rt: SET_INT64, a: address, k: key, v: value }).unwrap()
    }
    pub fn set_int64_with_version(address: String, key: String, value: i64, version: i64) -> String {
        serde_json::to_string(&SetInt64WithVersionRequest { rt: SET_INT64_WITH_VERSION, a: address, k: key, v: value, ver: version }).unwrap()
    }
    pub fn get_value_version(address: String, key: String) -> String {
        serde_json::to_string(&GetValueVersionRequest { rt: GET_VALUE_VERSION, a: address, k: key }).unwrap()
    }
    pub fn get_value(address: String, key: String, version: i64) -> String {
        serde_json::to_string(&GetValueRequest { rt: GET_VALUE, a: address, k: key, ver: version }).unwrap()
    }
}

pub const SUCCESS: u8 = 0;
pub const ERROR: u8 = 1;

// 运行时数据交互 - 返回
#[derive(Serialize, Deserialize)]
pub struct Result {
    // 响应编码，与 result codes 对应
    rc: u8,
}

// 获取账本哈希返回
#[derive(Serialize, Deserialize)]
pub struct GetLedgerHashResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 账本哈希
    pub lh: Option<String>,
}

// 获取合约地址返回
#[derive(Serialize, Deserialize)]
pub struct GetContractAddressResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 合约地址
    pub ca: Option<String>,
}

// 获取交易哈希返回
#[derive(Serialize, Deserialize)]
pub struct GetTxHashResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 交易哈希
    pub th: Option<String>,
}

// 获取交易时间返回
#[derive(Serialize, Deserialize)]
pub struct GetTxTimeResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 交易哈希
    pub tt: Option<u64>,
}

// 获取签名用户地址列表返回
#[derive(Serialize, Deserialize)]
pub struct GetSignersResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 交易签名用户列表
    pub ss: Option<Vec<String>>,
}

// 注册用户返回
#[derive(Serialize, Deserialize)]
pub struct RegisterUserResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 用户地址
    pub a: Option<String>,
}

// 查询用户返回
#[derive(Serialize, Deserialize)]
pub struct GetUserResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 用户地址
    pub a: Option<String>,
    // 用户公钥
    pub pk: Option<String>,
}

// 用户
pub struct User {
    // 用户地址
    pub address: String,
    // 用户公钥
    pub pubkey: String,
}

// 注册数据账户返回
#[derive(Serialize, Deserialize)]
pub struct RegisterDataAccountResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 数据账户地址
    pub a: Option<String>,
}

// 查询数据账户返回
#[derive(Serialize, Deserialize)]
pub struct GetDataAccountResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 数据账户地址
    pub a: Option<String>,
    // 数据账户公钥
    pub pk: Option<String>,
}

// 数据账户
pub struct DataAccount {
    // 数据账户地址
    pub address: String,
    // 数据账户公钥
    pub pubkey: String,
}

// 写 KV 返回
#[derive(Serialize, Deserialize)]
pub struct SetKVResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 数据版本
    pub ver: Option<i64>,
}

// 查 Key 版本返回
#[derive(Serialize, Deserialize)]
pub struct GetValueVersionResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // 数据版本
    pub ver: Option<i64>,
}

// 查 KV 返回
#[derive(Serialize, Deserialize)]
pub struct GetValueResult {
    // 响应编码，与 result codes 对应
    pub rc: u8,
    // key
    pub k: Option<String>,
    // value
    pub v: Option<String>,
    // value type
    pub t: Option<String>,
    // 数据版本
    pub ver: Option<i64>,
}

// KV数据
pub struct KVData {
    // key
    pub key: String,
    // value
    pub value: String,
    // value type
    pub value_type: String,
    // 数据版本
    pub version: i64,
}