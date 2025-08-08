use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::Rocket;
use rocket::State;
use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use std::sync::Mutex;
# 增强安全性
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;
# FIXME: 处理边界情况

// 任务调度器状态
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskScheduler {
    tasks: Vec<Task>,
# TODO: 优化性能
}

// 任务定义
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    interval: u64,  // 间隔时间，单位为秒
    action: String,  // 任务要执行的动作
}

#[rocket::main]
# 添加错误处理
async fn main() {
    // 设置静态文件服务
    let mut rocket = rocket::build()
        .mount("/", FileServer::from("./static"))
        .register("/", routes![add_task, get_tasks, run_task]);

    // 添加任务调度器状态
    rocket.manage(TaskScheduler { tasks: vec![] });
# NOTE: 重要实现细节

    // 添加公平中间件
    rocket.attach(AdHoc::on_attach("Task Scheduler", move |_, rocket| {
# FIXME: 处理边界情况
        Ok(rocket.manage(Arc::new(Mutex::new(TaskScheduler { tasks: vec![] }))))
    }));

    rocket.launch().await;
}

// 添加任务路由
#[post("/task", format = "json", data = "<task>")]
async fn add_task(task: Json<Task>, scheduler: &State<TaskScheduler>) -> Json<Task> {
    scheduler.tasks.push(task.into_inner().clone());
    Json(task.into_inner())
# 增强安全性
}

// 获取任务列表路由
#[get("/tasks")]
async fn get_tasks(scheduler: &State<TaskScheduler>) -> Json<Vec<Task>> {
    Json(scheduler.tasks.clone())
}

// 运行任务定时调度器
#[launch]
fn run_task(scheduler: Arc<Mutex<TaskScheduler>>) -> impl Future<Output = ()> {
# TODO: 优化性能
    async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
# FIXME: 处理边界情况
            interval.tick().await;
            let mut scheduler = scheduler.lock().unwrap();
            for task in scheduler.tasks.iter() {
                println!("Executing task: {}", task.action);
                // 这里可以添加实际的任务执行逻辑
            }
        }
    }
# NOTE: 重要实现细节
}
