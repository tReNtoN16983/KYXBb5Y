use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;
use std::collections::HashMap;

// 定义任务结构体
struct Task {
    interval: Duration,
    callback: Box<dyn Fn() + Send + 'static>,
}

// 定义调度器结构体
pub struct Scheduler {
    tasks: Mutex<HashMap<String, Task>>,
}

impl Scheduler {
    // 创建一个新的调度器
    pub fn new() -> Self {
        Scheduler {
            tasks: Mutex::new(HashMap::new()),
        }
    }

    // 添加任务
    pub fn add_task<T>(&self, id: String, interval: Duration, callback: T) 
    where
        T: Fn() + Send + 'static,
    {
        let task = Task {
            interval,
            callback: Box::new(callback),
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(id, task);
    }

    // 启动调度器
    pub async fn start(&self) {
        let tasks = self.tasks.lock().await;
        for (_id, task) in tasks.iter() {
            let interval = task.interval;
            let callback = task.callback.clone();

            tokio::spawn(async move {
                loop {
                    interval.tick().await;
                    (callback)();
                }
            });
        }
    }
}

#[macro_export]
macro_rules! schedule {
    ($scheduler:expr, $id:expr, $interval:expr, $callback:expr) => {
        $scheduler.add_task($id.to_string(), $interval, $callback);
    };
}

// 定义一个简单的任务回调函数
fn my_task() {
    println!("Task executed!");
}

#[rocket::main]
async fn main() {
    let scheduler = Arc::new(Scheduler::new());

    // 添加任务
    schedule!(scheduler.clone(), "my_task", Duration::from_secs(5), my_task);

    // 启动调度器
    scheduler.start().await;

    // 启动Rocket应用
    rocket::build()
        .mount("/", routes![get_health])
        .manage(scheduler)
        .launch()
        .await;
}

// 定义一个健康检查端点
#[get("/health")]
fn get_health() -> &'static str {
    "Service is healthy"
}
