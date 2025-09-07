use rocket::serde::{json::Json, Deserialize, Serialize};
    use rocket::http::Status;
    use rocket::request::{FromRequest, Outcome, Request};
    use rocket::outcome::IntoOutcome;
    use rocket::State;
    use std::sync::RwLock;
    use std::collections::HashMap;

    // 用户身份认证结构
    #[derive(Serialize)]
    struct AuthResponse {
        message: String,
    }

    // 用户请求结构
    #[derive(Deserialize, Debug)]
    struct AuthRequest {
        username: String,
        password: String,
    }

    // 用户身份认证状态
    struct AuthState {
        users: RwLock<HashMap<String, String>>,
    }

    // 实现从请求中提取用户身份
    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for AuthState {
        type Error = ();

        async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
            match request.guard::<&State<AuthState>>() {
                Ok(state) => Outcome::Success(state.clone()),
                Err(_) => Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }

    // 用户身份认证逻辑
    async fn authenticate(
        auth: AuthState,
        mut request: Json<AuthRequest>,
    ) -> Result<Json<AuthResponse>, Status> {
        let username = request.username.clone();
        let password = request.password.clone();

        let users = auth.users.read().unwrap();
        if users.get(&username).is_some() && users.get(&username).unwrap() == &password {
            Ok(Json(AuthResponse {
                message: format!(