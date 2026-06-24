#[macro_use] extern crate rocket;

use std::path::PathBuf;

#[launch]
fn rocket() -> _ {
    // Pfad relativ zum Workspace-Root (cargo run -p backend von workspace root)
    let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("frontend")
        .join("dist")
        .canonicalize()
        .expect("frontend/dist existiert nicht — trunk build ausführen");

    rocket::build()
        .mount("/", rocket::fs::FileServer::from(dist))
}