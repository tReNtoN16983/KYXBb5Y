use rocket::get;
use rocket::Route;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::net::LookupHost;
use rocket::response::status;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rocket::response::NamedFile;
use rocket::fs::FileServer;
use rocket::figment::Figment;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Deserialize, Serialize};

// Define the configuration struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DnsCacheConfig {
    cache_duration: Duration,
}

// Define the DNS cache struct
struct DnsCache {
    data: Mutex<HashMap<String, (String, Duration)>>,
}

impl DnsCache {
    pub fn new() -> Self {
        DnsCache {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn lookup(&self, hostname: &str) -> Option<(String, Duration)> {
        let mut cache = self.data.lock().unwrap();
        cache.get(hostname).cloned()
    }

    pub fn insert(&self, hostname: &str, ip_address: &str, duration: Duration) {
        let mut cache = self.data.lock().unwrap();
        cache.insert(hostname.to_string(), (ip_address.to_string(), duration));
    }
}

lazy_static! {
    static ref DNS_CACHE: DnsCache = DnsCache::new();
}

#[get("/<hostname>")]
fn resolve_dns(hostname: String) -> Result<Json<(String, Duration)>, status::NotFound<String>> {
    match DNS_CACHE.lookup(&hostname) {
        Some((ip, duration)) => Ok(Json((ip, duration))),
        None => {
            match hostname.to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        let ip = addr.to_string();
                        DNS_CACHE.insert(&hostname, &ip, Duration::from_secs(60));
                        Ok(Json((ip, Duration::from_secs(60))))
                    } else {
                        Err(status::NotFound(Some("No address found".to_string())))
                    }
                },
                Err(_) => Err(status::NotFound(Some("DNS resolution failed".to_string()))),
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DNS_CACHE.clone())
        .mount("/", routes![resolve_dns])
        .mount("/", FileServer::from("public"))
        .register("/config", routes![])
}
