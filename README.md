# Diesel Playground

Rust の ORM ライブラリ Diesel の検証用リポジトリ

## 構成情報

- Rust: 1.54.0
- Diesel: 1.4.7
- Diesel CLI: 1.4.1
- MySQL: 8.0.26

## ローカル環境構築手順

```sh
# docker を起動する
% docker compose up -d
# diesel のセットアップを行う
% docker compose exec rust diesel setup

# MySQL にログインし, 検証用データを登録する
% docker-compose exec mysql mysql -u diesel-user diesel -p
Enter password: # <= password を入力してくださいß

mysql> insert into users (name, age) values ('alice', 101);
Query OK, 1 row affected (0.01 sec)

mysql> exit
Bye

# docker を実行する
% docker compose exec rust cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/diesel-playground`
Displaying 1 users
User { id: 1, name: "alice", age: 101 }

# docker を停止する
% docker compose down
```

## diesel コマンド

```sh
# diesel のセットアップを行う
% docker compose exec rust diesel setup
# マイグレーションファイルを作成する
% docker compose exec rust diesel migration generate <generate_name>
# マイグレーションを実行する
% docker compose exec rust diesel migration run
```

## docker コマンド

```sh
# ボリュームの削除
% docker compose down
% docker volume rm diesel-mysql-data
# イメージの削除
% docker compose down
% docker rmi diesel-playground_rust
```

## ドキュメント

- https://docs.diesel.rs/diesel/index.html
- https://diesel.rs/guides/getting-started
- https://hub.docker.com/_/rust
- https://hub.docker.com/_/mysql
