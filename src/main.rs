#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bcrypt::*;
use diesel::prelude::*;
use rocket::{
    form::Form,
    fs::NamedFile,
    http::{ContentType, Status},
};
use std::path::{Path, PathBuf};
use twumc_backend::{establish_connection, models::User, schema::users};

// === Getters === //

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html"))
        .await
        .ok()
}

#[get("/<file..>")]
async fn file(file: PathBuf) -> Option<NamedFile> {
    let page_directory_path = format!("{}/dist", env!("CARGO_MANIFEST_DIR"));
    let path = Path::new(&page_directory_path).join(file);
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

#[post("/login", data = "<login_request>")]
fn login(login_request: Form<LoginRequest<'_>>) -> (Status, (ContentType, &'static str)) {
    let connection = establish_connection();

    let db_user = users::table
        .filter(users::name.eq(login_request.username))
        .select((users::id, users::name, users::password, users::permission))
        .first::<User>(&connection)
        .optional();

    match db_user {
        Ok(user) => {
            if let Some(user) = user {
                let verification = verify(login_request.password, &user.password);
                match verification {
                    Ok(v) => {
                        if v {
                            return (Status::Ok, (ContentType::JSON, "{ \"msg\": \"success!\"}"));
                        }
                        return (
                            Status::Unauthorized,
                            (
                                ContentType::JSON,
                                "{ \"error\": \"Invalid username/password!\"}",
                            ),
                        );
                    }
                    _ => return (
                        Status::InternalServerError,
                        (
                            ContentType::JSON,
                            "{ \"err\": \"Server failed validating password! Try again please\" }",
                        ),
                    ),
                }
            } else {
                return (
                    Status::Unauthorized,
                    (ContentType::JSON, "{ \"error\": \"User not found!\"}"),
                );
            }
        }
        Err(e) => {
            return (
                Status::InternalServerError,
                (ContentType::JSON, "{ \"error\": \"Server failure!!\"}"),
            )
        }
    }
}

// === And away we go! === //
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![file])
        .mount("/", routes![index])
        .mount("/api", routes![login])
}
