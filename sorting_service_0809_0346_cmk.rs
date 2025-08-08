// 排序服务模块

use rocket::Route;

// 引入Rust标准库中的排序功能
use std::cmp::Ordering;

// 定义排序算法枚举
#[derive(Debug, Clone, Copy)]
enum SortingAlgorithm {
    // 冒泡排序
    BubbleSort,
    // 快速排序
    QuickSort
}

// 排序服务结构体
struct SortingService;

impl SortingService {
    // 冒泡排序实现
    fn bubble_sort<T: Ord + Copy>(mut list: Vec<T>) -> Vec<T> {
        // 遍历所有数组元素
        for i in 0..list.len() {
            // 最后i个元素已经是排好序的了
            for j in 0..list.len() - i - 1 {
                if list[j] > list[j + 1] {
                    // 交换元素
                    list.swap(j, j + 1);
                }
            }
        }
        list
    }

    // 快速排序实现
    fn quick_sort<T: Ord + Copy>(list: Vec<T>) -> Vec<T> {
        if list.len() <= 1 {
            return list;
        }
        let pivot = list[0];
        let mut left = list[1..]
            .into_iter()
            .filter(|x| x <= pivot)
            .collect::<Vec<_>>();
        let mut right = list[1..]
            .into_iter()
            .filter(|x| x > pivot)
            .collect::<Vec<_>>();
        Self::quick_sort(left) + vec![pivot] + Self::quick_sort(right)
    }

    // 排序方法，接受排序算法和数据
    fn sort<T: Ord + Copy + Send + Sync>(
        algorithm: SortingAlgorithm,
        data: Vec<T>,
    ) -> Result<Vec<T>, String> {
        match algorithm {
            SortingAlgorithm::BubbleSort => Ok(Self::bubble_sort(data)),
            SortingAlgorithm::QuickSort => Ok(Self::quick_sort(data)),
        }
    }
}

// 火箭路由模块
#[macro_use]
extern crate rocket;

// 定义火箭路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sort_endpoint])
}

// 排序API端点
#[get("/sort?<algorithm>&<data>")]
fn sort_endpoint<algorithm: String, data: String>() -> String {
    // 解析排序算法
    let algorithm = match algorithm.parse::<SortingAlgorithm>() {
        Ok(alg) => alg,
        Err(_) => return "Invalid sorting algorithm".to_string(),
    };

    // 解析数据
    let data: Vec<i32> = match data
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect() {
        Ok(d) => d,
        Err(_) => return "Invalid data format".to_string(),
    };

    // 执行排序
    match SortingService::sort(algorithm, data) {
        Ok(sorted_data) => format!("Sorted data: {:?}