use rocket::get;
use rocket::Route;
use std::sync::mpsc::channel;
# 扩展功能模块
use std::thread;
use std::time::{Duration, Instant};

// 定义一个定时任务结构体
struct ScheduledTask {
    interval: Duration,
    task: Box<dyn Fn() + Send + 'static>,
}

// 实现ScheduledTask方法
impl ScheduledTask {
    // 创建一个新的定时任务
    pub fn new(interval: Duration, task: Box<dyn Fn() + Send + 'static>) -> Self {
        ScheduledTask { interval, task }
    }

    // 启动定时任务
    pub fn start(&self) {
        // 使用线程分离任务
# 增强安全性
        thread::spawn(move || loop {
# NOTE: 重要实现细节
            (self.task)();
            thread::sleep(self.interval);
        });
    }
}
# 添加错误处理

// 定义一个调度器结构体
struct Scheduler {
    tasks: Vec<ScheduledTask>,
}

// 实现Scheduler方法
impl Scheduler {
# 优化算法效率
    // 创建一个新的调度器
    pub fn new() -> Self {
        Scheduler { tasks: Vec::new() }
    }

    // 添加任务到调度器
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }

    // 启动所有任务
    pub fn start(&self) {
        for task in &self.tasks {
# FIXME: 处理边界情况
            task.start();
        }
    }
}

// 定义一个简单的Web服务
#[macro_use] extern crate rocket;

#[get("/start")]
fn start_scheduler() -> &'static str {
    let mut scheduler = Scheduler::new();

    // 添加一个每10秒打印一次时间的任务
    scheduler.add_task(ScheduledTask::new(Duration::from_secs(10), Box::new(|| {
        println!("Current time: {}", Instant::now().elapsed().as_secs());
    })));

    // 启动调度器
    scheduler.start();
# 优化算法效率

    "Scheduler started"
}

#[launch]
fn rocket() -> _ {
# 改进用户体验
    rocket::build().mount("/", routes![start_scheduler])
}