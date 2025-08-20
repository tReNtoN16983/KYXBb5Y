use rocket::get;
use rocket::State;
use rocket_contrib::json::Json;
use chrono::prelude::*;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

// 定义任务状态结构体，使用Mutex保证线程安全
struct Scheduler {
    tasks: Mutex<Vec<Task>>
}

// 定义任务结构体
struct Task {
    name: String,
    schedule: Schedule,
    last_run: Mutex<DateTime<Utc>>
}

// 定义调度策略枚举
enum Schedule {
    Cron(String), // Cron表达式
}

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

// 定义定时任务调度器
#[get("/schedule")]
fn schedule(scheduler: &State<Scheduler>) -> Json<Vec<Task>> {
    let tasks = scheduler.tasks.lock().unwrap();
    Json(tasks.clone())
}

// 添加任务到调度器
#[get("/add_task")]
fn add_task(scheduler: &State<Scheduler>, task_name: String, schedule: String) -> Json<Task> {
    let mut tasks = scheduler.tasks.lock().unwrap();
    let task = Task {
        name: task_name,
        schedule: Schedule::Cron(schedule),
        last_run: Mutex::new(Utc::now())
    };
    tasks.push(task);
    Json(task)
}

// 定时执行任务
fn run_scheduler(scheduler: Scheduler) {
    loop {
        let tasks = scheduler.tasks.lock().unwrap();
        for task in tasks.iter() {
            match &task.schedule {
                Schedule::Cron(expr) => {
                    // 解析Cron表达式并执行任务
                    // 这里省略Cron表达式解析和任务执行的实现
                },
            }
        }
        sleep(Duration::from_secs(1)); // 每秒检查一次任务
    }
}

#[launch]
fn rocket() -> _ {
    let scheduler = Scheduler {
        tasks: Mutex::new(Vec::new())
    };

    rocket::build()
        .manage(scheduler)
        .mount("/", routes![schedule, add_task])
}
