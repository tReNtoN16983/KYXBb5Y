use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;

// 定义一个数据模型结构体 `User`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// 实现一个基本的数据模型服务
#[derive(Default)]
pub struct ModelService;

// 定义错误类型
#[derive(Debug)]
pub enum ModelError {
    NotFound,
    InvalidInput,
}

// 实现错误转换为HTTP状态码
impl From<ModelError> for Status {
    fn from(error: ModelError) -> Self {
        match error {
            ModelError::NotFound => Status::NotFound,
            ModelError::InvalidInput => Status::BadRequest,
        }
    }
}

impl ModelService {
    // 创建一个新的用户
    pub fn create_user(user: &User) -> Result<Json<User>, ModelError> {
        if user.name.is_empty() || user.email.is_empty() {
            return Err(ModelError::InvalidInput);
        }

        // 这里可以添加数据库操作代码，例如插入用户数据

        Ok(Json(user.clone()))
    }

    // 通过ID获取用户信息
    pub fn get_user(user_id: u32) -> Result<Json<User>, ModelError> {
        // 这里可以添加数据库查询代码，根据ID查找用户

        // 假设查找失败，返回错误
        Err(ModelError::NotFound)
    }
}

// 定义ROCKET路由，用于处理HTTP请求
#[rocket::get(