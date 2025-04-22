//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的 UNIX 时间戳（秒数）- 用于 tests7.rs
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    // 设置 TEST_FOO 环境变量 - 用于 tests7.rs
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 让 Cargo 在环境变量变化时重新运行构建脚本
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    
    // 启用 "pass" 特性 - 用于 tests8.rs
    println!("cargo:rustc-cfg=feature=\"pass\"");
}