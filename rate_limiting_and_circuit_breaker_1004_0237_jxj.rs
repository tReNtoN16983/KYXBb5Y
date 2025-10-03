use rocket::get;
    use rocket::http::Status;
    use rocket::State;
    use std::sync::Mutex;
    use std::collections::HashMap;
    use std::time::{SystemTime, Duration};
    use std::sync::Arc;

    // 限流器结构体
    #[derive(Default)]
    struct RateLimiter {
        limits: HashMap<String, (u32, SystemTime)>,
    }

    impl RateLimiter {
        // 检查请求是否超过了速率限制
        fn check_rate_limit(&mut self, key: &str) -> bool {
            let now = SystemTime::now();
            let limit = 10; // 允许的请求次数

            match self.limits.get_mut(key) {
                Some((count, last_time)) => {
                    if now.duration_since(*last_time).unwrap_or(Duration::from_secs(0)) >= Duration::from_secs(60) {
                        *count = 1; // 重置时间
                        *last_time = now;
                    } else if *count >= limit {
                        return false;
                    }
                    *count += 1;
                },
                None => {
                    self.limits.insert(key.to_string(), (1, now));
                },
            }
            true
        }
    }

    // 熔断器结构体
    struct CircuitBreaker {
        failures: u32,
        threshold: u32,
        window: Duration,
        last_reset: SystemTime,
    }

    impl CircuitBreaker {
        // 创建一个新的熔断器
        fn new(threshold: u32, window: Duration) -> Self {
            CircuitBreaker {
                failures: 0,
                threshold,
                window,
                last_reset: SystemTime::now(),
            }
        }

        // 检查熔断器状态
        fn allow_request(&mut self) -> bool {
            let now = SystemTime::now();
            if now.duration_since(self.last_reset).unwrap_or(Duration::from_secs(0)) > self.window {
                self.failures = 0;
                self.last_reset = now;
            }
            if self.failures >= self.threshold {
                false
            } else {
                true
            }
        }

        // 报告失败
        fn report_failure(&mut self) {
            self.failures += 1;
        }
    }

    // 共享状态
    struct SharedState {
        rate_limiter: Mutex<RateLimiter>,
        circuit_breaker: Mutex<CircuitBreaker>,
    }

    #[get(