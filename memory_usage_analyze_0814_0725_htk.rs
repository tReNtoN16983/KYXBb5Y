use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use std::process::Command;
use std::io::Error;
use rocket::fairing::AdHoc;
use memory_usage::MemoryUsage;
use std::sync::Mutex;

// 定义全局内存使用情况变量
lazy_static::lazy_static! {
    static ref MEMORY_USAGE: Mutex<MemoryUsage> = Mutex::new(MemoryUsage::new());
}

#[macro_use]
extern crate lazy_static;

// 定义MemoryUsage结构
struct MemoryUsage {
    bytes_allocated: u64,
    bytes_deallocated: u64,
    bytes_heap: u64,
    bytes_heap_used: u64,
    allocations_count: u64,
    deallocations_count: u64,
}

impl MemoryUsage {
    fn new() -> Self {
        MemoryUsage {
            bytes_allocated: 0,
            bytes_deallocated: 0,
            bytes_heap: 0,
            bytes_heap_used: 0,
            allocations_count: 0,
            deallocations_count: 0,
        }
    }

    fn update(&mut self) {
        let output = Command::new("/usr/bin/env").arg("rustc").arg("-Vv")
            .arg("-Z