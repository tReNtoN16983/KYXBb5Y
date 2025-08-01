It provides endpoints to fetch system information such as CPU usage, memory usage, and disk usage.
*/

#[macro_use] extern crate rocket;
extern crate sysinfo;

use rocket::serde::json::Json;
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, NetworkExt};
use std::sync::Arc;
use rocket::State;

// Define the SystemInfo structure to hold system information.
#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    cpu_usage: f32,
    disk_usage: Vec<DiskUsage>,
    network_usage: Vec<NetworkUsage>,
}

// Define the DiskUsage structure to hold disk usage information.
#[derive(Debug, Serialize, Deserialize)]
struct DiskUsage {
    device_name: String,
    total_size: u64,
    used_size: u64,
    free_space: u64,
}

// Define the NetworkUsage structure to hold network usage information.
#[derive(Debug, Serialize, Deserialize)]
struct NetworkUsage {
    device_name: String,
    received: u64,
    transmitted: u64,
}

// Define the system_monitor route to fetch system information.
#[get("/system_monitor")]
fn system_monitor(system: &State<System>) -> Json<SystemInfo> {
    let mut disks_usage = Vec::new();
    for disk in system.disks().iter() {
        disks_usage.push(DiskUsage {
            device_name: disk.name().to_string(),
            total_size: disk.total_space(),
            used_size: disk.used_space(),
            free_space: disk.free_space(),
        });
    }

    let mut network_usage = Vec::new();
    for (device_name, network_device) in system.networks().iter() {
        network_usage.push(NetworkUsage {
            device_name: device_name.clone(),
            received: network_device.received(),
            transmitted: network_device.transmitted(),
        });
    }

    Json(SystemInfo {
        total_memory: system.total_memory(),
        used_memory: system.used_memory(),
        free_memory: system.free_memory(),
        cpu_usage: system.global_processor_info().cpu_usage(),
        disk_usage: disks_usage,
        network_usage: network_usage,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(System::new_all())
        .mount("/", routes![system_monitor])
}
