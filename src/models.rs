use super::schema::users;

// 読み込み用のユーザ定義
#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

// 登録用のユーザ定義
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}
