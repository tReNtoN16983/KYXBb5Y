use rocket::serde::{Serialize, Deserialize};

// 定义一个简单的数据模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    pub username: String,
    pub email: String,
}

// 可以根据需要添加更多字段或方法到User

impl User {
    // 创建一个新的用户实例
    pub fn new(id: i32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
        }
    }

    // 一个简单的方法来验证电子邮件格式，可以根据需要进行扩展
    pub fn validate_email(&self) -> Result<(), String> {
        if self.email.contains('@') && self.email.contains('.') {
            Ok(())
        } else {
            Err("Email is not valid".to_string())
        }
    }
}

// 例子：如何使用User模型
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "example_user".to_string(), "user@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "example_user");
        assert_eq!(user.email, "user@example.com");
    }

    #[test]
    fn test_email_validation() {
        let user = User::new(1, "example_user".to_string(), "user@example.com".to_string());
        assert!(user.validate_email().is_ok());
    }
}
