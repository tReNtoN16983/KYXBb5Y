use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

// Patch represents a single patch with its identifier and content
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Patch {
    id: String,
    content: String,
}

// PatchManager manages a collection of patches
struct PatchManager {
    patches: Mutex<HashMap<String, Patch>>,
}

#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

impl PatchManager {
    // Creates a new PatchManager
    pub fn new() -> Self {
        PatchManager {
            patches: Mutex::new(HashMap::new()),
        }
    }

    // Adds a new patch to the manager
    pub fn add_patch(&self, id: String, content: String) -> Result<(), String> {
        if self.patches.lock().unwrap().contains_key(&id) {
            Err("Patch with the same ID already exists.".to_string())
        } else {
            let mut patches = self.patches.lock().unwrap();
            patches.insert(id, Patch { id: id.clone(), content });
            Ok(())
        }
    }

    // Removes a patch from the manager
    pub fn remove_patch(&self, id: String) -> Result<(), String> {
        let mut patches = self.patches.lock().unwrap();
        if patches.remove(&id).is_none() {
            Err("Patch not found.".to_string())
        } else {
            Ok(())
        }
    }

    // Retrieves a patch from the manager
    pub fn get_patch(&self, id: String) -> Result<Patch, String> {
        match self.patches.lock().unwrap().get(&id) {
            Some(patch) => Ok(patch.clone()),
            None => Err("Patch not found.".to_string()),
        }
    }
}

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            add_patch,
            remove_patch,
            get_patch,
        ])
        .manage(PatchManager::new())
}

#[get("/patches/<id>/")]
fn get_patch_endpoint(id: String, patch_manager: &State<PatchManager>) -> Result<Json<Patch>, NotFound<&'static str>> {
    match patch_manager.get_patch(id) {
        Ok(patch) => Ok(Json(patch)),
        Err(_) => Err(NotFound::new("Patch not found.".to_string())),
    }
}

#[post("/patches/", format = "json", data = "<patch>"])
fn add_patch_endpoint(patch: Json<Patch>, patch_manager: &State<PatchManager>) -> Result<&'static str, String> {
    match patch_manager.add_patch(patch.id.clone(), patch.content.clone()) {
        Ok(_) => Ok("Patch added successfully."),
        Err(e) => Err(e),
    }
}

#[delete("/patches/<id>/")]
fn remove_patch_endpoint(id: String, patch_manager: &State<PatchManager>) -> Result<&'static str, String> {
    match patch_manager.remove_patch(id) {
        Ok(_) => Ok("Patch removed successfully."),
        Err(e) => Err(e),
    }
}
