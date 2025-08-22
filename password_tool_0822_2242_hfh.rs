 * Features:
 * - Encrypts and decrypts passwords using a symmetric key algorithm.
 * - Provides clear error handling and user-friendly messages.
 * - Includes documentation and comments for maintainability and extendability.
 */

use rocket::get;
use rocket::post;
use rocket::serde::{json::Json, ser::json::JsonError};
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use rocket::response::content;
use rocket::response::Responder;
use rocket::Request;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use block_modes::BlockModeError;
use rand::{distributions::Alphanumeric, Rng};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

#[macro_use]
mod common;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EncryptRequest {
    #[serde(crate = "rocket::serde")]
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EncryptResponse {
    #[serde(crate = "rocket::serde")]
    encrypted_password: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DecryptRequest {
    #[serde(crate = "rocket::serde")]
    encrypted_password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecryptResponse {
    #[serde(crate = "rocket::serde")]
    decrypted_password: String,
}

#[post("/encrypt", format = "json", data = "<encrypt_request>")]
fn encrypt(encrypt_request: Json<EncryptRequest>) -> Result<content::Json<EncryptResponse>, JsonError> {
    let password = encrypt_request.into_inner().password;
    let key = generate_key();
    let encrypted_password = encrypt_password(&password, &key).unwrap_or_else(|e| {
        eprintln!("Error encrypting password: {}", e);
        return Err(JsonError::new(Status::InternalServerError, "Internal Server Error"));
    });
    Ok(content::Json(EncryptResponse { encrypted_password }))
}

#[post("/decrypt", format = "json", data = "<decrypt_request>")]
fn decrypt(decrypt_request: Json<DecryptRequest>) -> Result<content::Json<DecryptResponse>, JsonError> {
    let encrypted_password = decrypt_request.into_inner().encrypted_password;
    let key = generate_key();
    let decrypted_password = decrypt_password(&encrypted_password, &key).unwrap_or_else(|e| {
        eprintln!("Error decrypting password: {}", e);
        return Err(JsonError::new(Status::InternalServerError, "Internal Server Error"));
    });
    Ok(content::Json(DecryptResponse { decrypted_password }))
}

fn generate_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = rng.sample(Alphanumeric).take(32).collect();
    key
}

fn encrypt_password(password: &str, key: &[u8]) -> Result<String, BlockModeError> {
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(key, b"my_iv")?;
    let encrypted_password = cipher.encrypt_vec(password.as_bytes())?;
    Ok(base64::encode(encrypted_password))
}

fn decrypt_password(encrypted_password: &str, key: &[u8]) -> Result<String, BlockModeError> {
    let encrypted_password = base64::decode(encrypted_password)?;
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(key, b"my_iv")?;
    let decrypted_password = cipher.decrypt_vec(&encrypted_password)?;
    Ok(String::from_utf8(decrypted_password)?)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/password", routes![encrypt, decrypt])
}