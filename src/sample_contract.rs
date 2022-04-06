use std::ffi::CString;
use std::os::raw::c_char;

use jdcc_api::*;

use crate::jdcc_api;
use crate::jdcc_types::SUCCESS;

#[no_mangle]
// 合约方法前置操作
// 每次合约方法调用前执行，如无特殊处理逻辑可不暴露此方法
pub fn before_event() {
    let service = LedgerService::default();
    service.logger().info("start call".to_string());
}

#[no_mangle]
// 合约方法后置操作
// 每次合约方法调用后执行，如无特殊处理逻辑可不暴露此方法
// 此处运行时仅将合约执行过程中是否出错传递过来，具体错误信息请查阅PEER节点日志
pub fn post_event(code: i32) {
    let service = LedgerService::default();
    match code as u8 {
        SUCCESS => service.logger().info("end success call".to_string()),
        _ => service.logger().info("end error call".to_string())
    }
}

// 下列方法为 JD Chain 支持的合约操作
// 可直接用于合约方法逻辑，对外暴露使用；可组合多个基础方法构建复杂逻辑

#[no_mangle]
// 获取账本哈希
pub fn get_ledger_hash() -> *mut c_char {
    let service = LedgerService::default();
    let ledger = service.get_ledger_hash();
    let ledger: String = match ledger {
        Some(val) => val,
        _ => "".to_string()
    };
    CString::new(ledger).unwrap().into_raw()
}

#[no_mangle]
// 获取合约地址
pub fn get_contract_address() -> *mut c_char {
    let service = LedgerService::default();
    let contract = service.get_contract_address();
    let contract: String = match contract {
        Some(val) => val,
        _ => "".to_string()
    };
    CString::new(contract).unwrap().into_raw()
}

#[no_mangle]
// 获取合约地址
pub fn get_tx_hash() -> *mut c_char {
    let service = LedgerService::default();
    let tx = service.get_tx_hash();
    let tx: String = match tx {
        Some(val) => val,
        _ => "".to_string()
    };
    CString::new(tx).unwrap().into_raw()
}

#[no_mangle]
// 获取交易时间
pub fn get_tx_time() -> u64 {
    let service = LedgerService::default();
    let time = service.get_tx_time();
    match time {
        Some(val) => val,
        _ => 0
    }
}

#[no_mangle]
// 获取交易签名用户地址列表
pub fn get_signers() -> *mut c_char {
    let service = LedgerService::default();
    let signers = service.get_signers();
    let signers: String = match signers {
        Some(val) => val.join(","),
        _ => "".to_string()
    };
    CString::new(signers).unwrap().into_raw()
}

#[no_mangle]
// 注册用户
pub extern fn register_user(seed_ptr: *mut c_char) -> *mut c_char {
    let service = LedgerService::default();
    let address = service.register_user(seed_ptr);
    let address: String = match address {
        Some(val) => val,
        _ => "".to_string()
    };
    CString::new(address).unwrap().into_raw()
}

#[no_mangle]
// 查询用户
pub fn get_user(address_ptr: *mut c_char) -> *mut c_char {
    let service = LedgerService::default();
    let user = service.get_user(address_ptr);
    let pub_key: String = match user {
        Some(val) => val.pubkey,
        _ => "".to_string()
    };
    CString::new(pub_key).unwrap().into_raw()
}

#[no_mangle]
// 注册数据账户
pub fn register_data_account(seed_ptr: *mut c_char) -> *mut c_char {
    let service = LedgerService::default();
    let address = service.register_data_account(seed_ptr);
    let address: String = match address {
        Some(val) => val,
        _ => "".to_string()
    };
    CString::new(address).unwrap().into_raw()
}

#[no_mangle]
// 查询数据账户
pub fn get_data_account(address_ptr: *mut c_char) -> *mut c_char {
    let service = LedgerService::default();
    let da = service.get_data_account(address_ptr);
    let pub_key: String = match da {
        Some(val) => val.pubkey,
        _ => "".to_string()
    };
    CString::new(pub_key).unwrap().into_raw()
}

#[no_mangle]
// 写KV，字符类型，不带版本
pub fn set_text(addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: *mut c_char) -> i64 {
    let service = LedgerService::default();
    let ver = service.set_text(addr_ptr, key_ptr, value_ptr);
    match ver {
        Some(val) => val,
        _ => -1
    }
}

#[no_mangle]
// 写KV，字符类型
pub fn set_text_with_version(addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: *mut c_char, version: i64) -> i64 {
    let service = LedgerService::default();
    let ver = service.set_text_with_version(addr_ptr, key_ptr, value_ptr, version);
    match ver {
        Some(val) => val,
        _ => -1
    }
}

#[no_mangle]
// 写KV，数值类型，不带版本
pub fn set_int64(addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: i64) -> i64 {
    let service = LedgerService::default();
    let ver = service.set_int64(addr_ptr, key_ptr, value_ptr);
    match ver {
        Some(val) => val,
        _ => -1
    }
}

#[no_mangle]
// 写KV，数值类型
pub fn set_int64_with_version(addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: i64, version: i64) -> i64 {
    let service = LedgerService::default();
    let ver = service.set_int64_with_version(addr_ptr, key_ptr, value_ptr, version);
    match ver {
        Some(val) => val,
        _ => -1
    }
}

#[no_mangle]
// 查询数据版本
pub fn get_value_version(addr_ptr: *mut c_char, key_ptr: *mut c_char) -> i64 {
    let service = LedgerService::default();
    let ver = service.get_value_version(addr_ptr, key_ptr);
    match ver {
        Some(val) => val,
        _ => -1
    }
}

#[no_mangle]
// 查询数据
pub fn get_value(addr_ptr: *mut c_char, key_ptr: *mut c_char, version: i64) -> *mut c_char {
    let service = LedgerService::default();
    let kv = service.get_value(addr_ptr, key_ptr, version);
    let value: String = match kv {
        Some(val) => val.value,
        _ => "".to_string()
    };
    CString::new(value).unwrap().into_raw()
}