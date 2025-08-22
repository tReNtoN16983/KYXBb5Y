use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

/// 缓存策略的实现
/// 这里使用一个简单的内存缓存来存储数据。
/// 数据存储在一个HashMap中，键是数据的标识符，值是数据本身。
/// 使用Mutex来保证线程安全。
#[macro_use] extern crate rocket;

#[derive(Debug, Clone)]
struct CacheItem {
    /// 缓存的数据
    data: String,
    /// 缓存的有效时间（秒）
    valid_until: i64,
}

/// 缓存服务
/// 提供接口来添加、获取和清理缓存数据。
struct CacheService {
    /// 缓存数据
    cache: Mutex<HashMap<String, CacheItem>>,
}

impl CacheService {
    /// 创建一个新的缓存服务
    fn new() -> Self {
        CacheService {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// 添加数据到缓存
    fn add_to_cache(&self, key: String, data: String, ttl: i64) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, CacheItem {
            data,
            valid_until: ttl,
        });
    }

    /// 从缓存中获取数据
    fn get_from_cache(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(key)
            .and_then(|item| if item.valid_until > chrono::Utc::now().timestamp() {
                Some(item.data.clone())
            } else {
                cache.remove(key);
                None
            })
    }

    /// 清理过期的缓存数据
    fn clean_expired_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        let now = chrono::Utc::now().timestamp();
        cache.retain(|_, item| item.valid_until > now);
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(CacheService::new())
        .mount("/cache", routes![add_cache, get_cache, clean_cache])
}

#[get("/add/<key>/<data>/<ttl>")]
fn add_cache(key: String, data: String, ttl: i64, cache: &State<CacheService>) -> String {
    cache.add_to_cache(key.clone(), data, ttl);
    format!("Added to cache: {} with TTL: {}", key, ttl)
}

#[get("/get/<key>")]
fn get_cache(key: String, cache: &State<CacheService>) -> Option<String> {
    cache.get_from_cache(&key)
}

#[get("/clean")]
fn clean_cache(cache: &State<CacheService>) -> String {
    cache.clean_expired_cache();
    "Cleaned expired cache".to_string()
}
