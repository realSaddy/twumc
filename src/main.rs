#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::{fs::NamedFile, form::Form};

// === Getters === // 

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await.ok()
}

#[get("/<file..>")]
async fn file(file: PathBuf) -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    let path =  Path::new(&page_directory_path).join(file);
    if path.exists() {
        NamedFile::open(path).await.ok()
    } else {
        index().await
    }
}

// === API === //

#[derive(FromForm)]
struct LoginRequest<'r> {
    r#username: &'r str,
    r#password: &'r str,
}

#[post("/login", data="<login_request>")]
fn login(login_request: Form<LoginRequest<'_>>) {
   println!("{}", login_request.username);
   println!("{}", login_request.password);
}

// === And away we go! === //
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![file]).mount("/", routes![index]).mount("/api", routes![login])
}
