#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::{fs::NamedFile};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await.ok()
}

#[get("/<file..>")]
async fn file(file: PathBuf) -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![file]).mount("/", routes![index])
}
