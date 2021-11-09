use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// データベースへの接続を確立する
pub fn establish_connection() -> MysqlConnection {
    // .env の内容を環境変数として設定する
    dotenv().ok();

    // 環境変数からデータベースへの接続先を取得する
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // データベースへの接続する
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}
