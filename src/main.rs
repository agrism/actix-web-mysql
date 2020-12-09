extern crate r2d2;
extern crate r2d2_mysql;

use actix_web::{get, web, App, HttpServer, Responder};
use r2d2_mysql::mysql::{OptsBuilder, QueryResult, from_row};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use std::env;
use dotenv::dotenv;

fn get_pool() -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let mut o = OptsBuilder::new();

    let db_name = env::var("DB_NAME").expect("db name not set!");
    let db_host = env::var("DB_HOST").expect("db host not set!");
    let db_port = env::var("DB_PORT").expect("db port not set!");
    let port = db_port.parse::<u16>().unwrap();
    let db_username = env::var("DB_USERNAME").expect("db username not set!");
    let db_password = env::var("DB_PASSWORD").expect("db pass password not swt");

    o.db_name(Option::Some(db_name));
    o.user(Option::Some(db_username));
    o.pass(Option::Some(db_password));
    o.ip_or_hostname(Option::Some(db_host));
    o.tcp_port(port);

    let manager = r2d2_mysql::MysqlConnectionManager::new(o);

    println!("Getting pool");

    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
    Option::Some(pool)
}

struct AppState {
    app_name: String,
    pool: Arc<Pool<MysqlConnectionManager>>,
}

#[get("/persons/{id}")]
async fn index(info: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let pool = &data.pool;

    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    let param = info.into_inner();
    let qr: QueryResult = conn.prep_exec("select id, name from persons where id = ?", (param, )).unwrap();

    let mut rec: Option<(i32, String)> = None;
    for row in qr {
        rec = Some(from_row(row.unwrap()));
        break;
    }

    let unwrap_rec = rec.unwrap();
    format!("Hello {} ({})! \n from {}", unwrap_rec.1, unwrap_rec.0, app_name)

}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let app_data= web::Data::new(AppState{
        app_name: String::from("Rust with MySql"),
        pool: get_pool().unwrap(),
    });

    HttpServer::new( move || {
        App::new().app_data(app_data.clone()).service(index)
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await

}