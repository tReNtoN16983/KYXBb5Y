use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::{HashMap, HashSet};

// 定义一个结构体来存储搜索算法的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SearchItem {
    id: i32,
# 优化算法效率
    name: String,
    // 可以根据需要添加更多的字段
}

// 定义一个搜索服务结构体
struct SearchService {
    items: Vec<SearchItem>,
}

// 实现SearchService的方法
impl SearchService {
    // 构造函数
    pub fn new(items: Vec<SearchItem>) -> Self {
        SearchService { items }
    }

    // 搜索函数，接受一个搜索词，返回匹配的搜索项
    pub fn search(&self, query: &str) -> Vec<SearchItem> {
        self.items
            .iter()
            .filter(|item| item.name.contains(query))
            .cloned()
            .collect()
    }
}

// 定义一个状态管理结构体，用于Rocket应用
struct AppState {
# 增强安全性
    search_service: SearchService,
}

// 定义一个API端点，用于搜索
#[rocket::get(