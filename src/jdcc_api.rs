// JD Chain Contract API

use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::os::raw::c_char;

use crate::jdcc_types::*;

extern "C" {
    pub fn sys_call(req_len: i32, req_ptr: *mut c_char) -> usize;
    pub fn sys_msg(msg_len: i32, msg_ptr: *mut c_char) -> *mut c_char;
}

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    mem::forget(buffer);

    ptr as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(ptr: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, capacity);
    }
}

#[no_mangle]
pub extern fn drop_string(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

// 账本服务接口
pub struct LedgerService {
    logger: Logger,
}

impl LedgerService {
    pub fn default() -> Self {
        LedgerService {
            logger: Logger {}
        }
    }

    fn call_and_get_sys_msg(&self, req: &str) -> &str {
        let req_len = req.len();
        let msg_len;
        unsafe {
            msg_len = sys_call(req_len as i32, CString::new(req).unwrap().into_raw()) as usize;
        };
        let msg_ptr = allocate(msg_len) as *mut c_char;
        let msg_ptr = unsafe {
            sys_msg(msg_len as i32, msg_ptr)
        };
        let msg = unsafe { CStr::from_ptr(msg_ptr).to_str().unwrap() };
        deallocate(msg_ptr as *mut c_void, msg_len);

        msg
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    // 获取账本哈希
    pub fn get_ledger_hash(&self) -> Option<String> {
        let req = Request::get_ledger_hash();
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetLedgerHashResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetLedgerHashResult { rc: ERROR, lh: None },
        };
        match result.rc {
            SUCCESS => result.lh,
            _ => None
        }
    }

    // 获取合约地址
    pub fn get_contract_address(&self) -> Option<String> {
        let req = Request::get_contract_address();
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetContractAddressResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetContractAddressResult { rc: ERROR, ca: None },
        };
        match result.rc {
            SUCCESS => result.ca,
            _ => None
        }
    }

    // 获取交易哈希
    pub fn get_tx_hash(&self) -> Option<String> {
        let req = Request::get_tx_hash();
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetTxHashResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetTxHashResult { rc: ERROR, th: None },
        };
        match result.rc {
            SUCCESS => result.th,
            _ => None
        }
    }

    // 获取交易时间
    pub fn get_tx_time(&self) -> Option<u64> {
        let req = Request::get_tx_time();
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetTxTimeResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetTxTimeResult { rc: ERROR, tt: None },
        };
        match result.rc {
            SUCCESS => result.tt,
            _ => None
        }
    }

    // 获取交易签名用户地址列表
    pub fn get_signers(&self) -> Option<Vec<String>> {
        let req = Request::get_signers();
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetSignersResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetSignersResult { rc: ERROR, ss: None },
        };
        match result.rc {
            SUCCESS => result.ss,
            _ => None
        }
    }

    // 注册用户
    pub fn register_user(&self, seed_ptr: *mut c_char) -> Option<String> {
        let seed = unsafe { CStr::from_ptr(seed_ptr).to_str().unwrap() };
        let req = Request::register_user(seed.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: RegisterUserResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => RegisterUserResult { rc: ERROR, a: None },
        };
        match result.rc {
            SUCCESS => result.a,
            _ => None
        }
    }

    // 查询用户
    pub fn get_user(&self, address_ptr: *mut c_char) -> Option<User> {
        let address = unsafe { CStr::from_ptr(address_ptr).to_str().unwrap() };
        let req = Request::get_user(address.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetUserResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetUserResult { rc: ERROR, a: None, pk: None },
        };
        match result.rc {
            SUCCESS => Some(User { address: result.a.unwrap(), pubkey: result.pk.unwrap() }),
            _ => None
        }
    }

    // 注册数据账户
    pub fn register_data_account(&self, seed_ptr: *mut c_char) -> Option<String> {
        let seed = unsafe { CStr::from_ptr(seed_ptr).to_str().unwrap() };
        let req = Request::register_data_account(seed.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: RegisterDataAccountResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => RegisterDataAccountResult { rc: ERROR, a: None },
        };
        match result.rc {
            SUCCESS => result.a,
            _ => None
        }
    }

    // 查询数据账户
    pub fn get_data_account(&self, address_ptr: *mut c_char) -> Option<DataAccount> {
        let address = unsafe { CStr::from_ptr(address_ptr).to_str().unwrap() };
        let req = Request::get_data_account(address.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetDataAccountResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetDataAccountResult { rc: ERROR, a: None, pk: None },
        };
        match result.rc {
            SUCCESS => Some(DataAccount { address: result.a.unwrap(), pubkey: result.pk.unwrap() }),
            _ => None
        }
    }

    // 写KV，字符类型，不带版本
    pub fn set_text(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: *mut c_char) -> Option<i64> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let value = unsafe { CStr::from_ptr(value_ptr).to_str().unwrap() };
        let req = Request::set_text(address.to_string(), key.to_string(), value.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: SetKVResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => SetKVResult { rc: ERROR, ver: None },
        };
        match result.rc {
            SUCCESS => result.ver,
            _ => None
        }
    }

    // 写KV，字符类型
    pub fn set_text_with_version(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char, value_ptr: *mut c_char, version: i64) -> Option<i64> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let value = unsafe { CStr::from_ptr(value_ptr).to_str().unwrap() };
        let req = Request::set_text_with_version(address.to_string(), key.to_string(), value.to_string(), version);
        let ret = self.call_and_get_sys_msg(&req);
        let result: SetKVResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => SetKVResult { rc: ERROR, ver: None },
        };
        match result.rc {
            SUCCESS => result.ver,
            _ => None
        }
    }

    // 写KV，数值类型，不带版本
    pub fn set_int64(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char, value: i64) -> Option<i64> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let req = Request::set_int64(address.to_string(), key.to_string(), value);
        let ret = self.call_and_get_sys_msg(&req);
        let result: SetKVResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => SetKVResult { rc: ERROR, ver: None },
        };
        match result.rc {
            SUCCESS => result.ver,
            _ => None
        }
    }

    // 写KV，数值类型
    pub fn set_int64_with_version(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char, value: i64, version: i64) -> Option<i64> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let req = Request::set_int64_with_version(address.to_string(), key.to_string(), value, version);
        let ret = self.call_and_get_sys_msg(&req);
        let result: SetKVResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => SetKVResult { rc: ERROR, ver: None },
        };
        match result.rc {
            SUCCESS => result.ver,
            _ => None
        }
    }

    // 查询数据版本
    pub fn get_value_version(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char) -> Option<i64> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let req = Request::get_value_version(address.to_string(), key.to_string());
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetValueVersionResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetValueVersionResult { rc: ERROR, ver: None },
        };
        match result.rc {
            SUCCESS => result.ver,
            _ => None
        }
    }

    // 查询数据
    pub fn get_value(&self, addr_ptr: *mut c_char, key_ptr: *mut c_char, version: i64) -> Option<KVData> {
        let address = unsafe { CStr::from_ptr(addr_ptr).to_str().unwrap() };
        let key = unsafe { CStr::from_ptr(key_ptr).to_str().unwrap() };
        let req = Request::get_value(address.to_string(), key.to_string(), version);
        let ret = self.call_and_get_sys_msg(&req);
        let result: GetValueResult = match serde_json::from_str(ret) {
            Ok(val) => val,
            Err(_) => GetValueResult { rc: ERROR, k: None, v: None, t: None, ver: None },
        };
        match result.rc {
            SUCCESS => Some(KVData {
                key: result.k.unwrap(),
                value: result.v.unwrap(),
                value_type: result.t.unwrap(),
                version: result.ver.unwrap(),
            }),
            _ => None
        }
    }
}

// 日志接口
pub struct Logger {}

impl Logger {
    pub fn debug(&self, msg: String) {
        let data = Request::log_debug(msg);
        let data_len = data.len() as i32;
        unsafe {
            sys_call(data_len, CString::new(data).unwrap().into_raw());
        }
    }

    pub fn info(&self, msg: String) {
        let data = Request::log_info(msg);
        let data_len = data.len() as i32;
        unsafe {
            sys_call(data_len, CString::new(data).unwrap().into_raw());
        }
    }

    pub fn error(&self, msg: String) {
        let data = Request::log_error(msg);
        let data_len = data.len() as i32;
        unsafe {
            sys_call(data_len, CString::new(data).unwrap().into_raw());
        }
    }
}