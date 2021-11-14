#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub permission: i32,
}
