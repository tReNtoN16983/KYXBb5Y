use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rocket::Outcome::*;
use rocket::Request;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

// 定义数据库配置
#[derive(Clone)]
struct DbConfig {
    database_url: String,
}

// 实现数据库连接池的公平处理程序
pub struct DbFairing {
    db_config: DbConfig,
}

#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Transform,
            name: 