// tab_switcher.rs

// 引入Rocket框架和serde用于序列化和反序列化
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

// 定义标签页数据结构
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Tab {
    name: String,
    content: String,
}

// 定义一个包含多个标签页的结构体
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TabSwitcher {
    tabs: Vec<Tab>,
    active_tab_index: usize,
}

// 实现TabSwitcher，提供切换标签页的方法
impl TabSwitcher {
    fn new() -> Self {
        TabSwitcher {
            tabs: vec![
                Tab { name: "Home".to_string(), content: "Welcome to the home page.".to_string() },
                Tab { name: "About".to_string(), content: "This is the about page.".to_string() },
            ],
            active_tab_index: 0,
        }
    }

    // 切换到下一个标签页
    fn next_tab(&mut self) {
        self.active_tab_index = (self.active_tab_index + 1) % self.tabs.len();
    }

    // 切换到上一个标签页
    fn prev_tab(&mut self) {
        self.active_tab_index = if self.active_tab_index == 0 {
            self.tabs.len() - 1
        } else {
            self.active_tab_index - 1
        };
    }

    // 获取当前活动的标签页
    fn active_tab(&self) -> &Tab {
        &self.tabs[self.active_tab_index]
    }
}

// 定义一个请求处理器，返回当前活动的标签页内容
#[get("/tab")]
fn get_active_tab(switcher: rocket::State<TabSwitcher>) -> Json<Tab> {
    Json(switcher.active_tab().clone())
}

// 定义一个请求处理器，切换到下一个标签页
#[get("/next_tab")]
fn next_tab_handler(switcher: rocket::State<TabSwitcher>) -> Json<Tab> {
    switcher.next_tab();
    Json(switcher.active_tab().clone())
}

// 定义一个请求处理器，切换到上一个标签页
#[get("/prev_tab")]
fn prev_tab_handler(switcher: rocket::State<TabSwitcher>) -> Json<Tab> {
    switcher.prev_tab();
    Json(switcher.active_tab().clone())
}

// 启动Rocket服务器，挂载状态和路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TabSwitcher::new())
        .mount("/", routes![get_active_tab, next_tab_handler, prev_tab_handler])
}
