use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

// Define the structure for a scheduled task
struct ScheduledTask {
    task_name: String,
    interval: Duration,
    last_run: Option<chrono::DateTime<chrono::Utc>>,
}

// Define a struct to hold the task scheduler
struct TaskScheduler {
    tasks: Arc<Mutex<HashMap<String, ScheduledTask>>>
}

impl TaskScheduler {
    // Initialize a new task scheduler
    fn new() -> Self {
        TaskScheduler {
            tasks: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    // Add a new scheduled task
    fn add_task(&mut self, task_name: String, interval: Duration) {
        let task = ScheduledTask {
            task_name: task_name.clone(),
            interval,
            last_run: None
        };
        self.tasks.lock().unwrap().insert(task_name, task);
    }

    // Run the task scheduler
    fn run(&self) {
        loop {
            let tasks = self.tasks.lock().unwrap();
            for (task_name, task) in tasks.iter() {
                if let Some(last_run) = task.last_run {
                    let now = chrono::Utc::now();
                    if now - last_run > task.interval {
                        // Execute the task
                        println!("Running task: {}", task_name);
                        // Here you would add the actual task execution logic
                        
                        // Update last run time
                        let mut updated_task = tasks.lock().unwrap();
                        updated_task.get_mut(task_name).unwrap().last_run = Some(chrono::Utc::now());
                    }
                } else {
                    // If task has never run, run it now
                    println!("Running task for the first time: {}", task_name);
                    // Here you would add the actual task execution logic
                    
                    // Update last run time
                    let mut updated_task = tasks.lock().unwrap();
                    updated_task.get_mut(task_name).unwrap().last_run = Some(chrono::Utc::now());
                }
            }
            thread::sleep(Duration::from_secs(1)); // Check tasks every second
        }
    }
}

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

// Define a Rocket configuration for the service
#[launch]
fn rocket() -> Rocket {
    let scheduler = TaskScheduler::new();
    scheduler.add_task("task1".to_string(), Duration::from_secs(10)); // Add a task that runs every 10 seconds

    // Run the scheduler in a separate thread
    thread::spawn(move || {
        scheduler.run();
    });

    rocket::build()
        .mount("/", StaticFiles::from("static"))
        .attach(AdHoc::on_attach("Task Scheduler", |rocket| {
            Ok(rocket)
        }));
}
