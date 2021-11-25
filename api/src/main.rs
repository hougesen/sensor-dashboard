mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
    #[pg_mapper(table = "measurements")]
    pub struct Measurements {
        pub measurement_id: Option<i32>,
        pub measurement_type_id: i32,
        pub location_id: i32,
        pub measurement_value: f32,
        pub measurement_time: String,
    }

    #[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
    #[pg_mapper(table = "locations")]
    pub struct Locations {
        location_id: Option<i32>,
        location_name: String,
    }

    #[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
    #[pg_mapper(table = "measurementtypes")]
    pub struct MeasurementTypes {
        measurement_type_id: Option<i32>,
        measurement_type_name: String,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use crate::{errors::MyError, models::*};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    pub async fn insert_measurement(
        client: &Client,
        measurement: Measurements,
    ) -> Result<Measurements, MyError> {
        println!("db insert_measurement {:#?}", measurement);
        let _stmt = include_str!("../queries/insert_measurement.sql");
        let _stmt = _stmt.replace("$table_fields", &Measurements::sql_table_fields());

        println!("_stmt {:?}", _stmt);

        let stmt = client.prepare(&_stmt).await.unwrap();
        println!("done prema");
        let res = client
            .query(
                &stmt,
                &[
                    &measurement.measurement_type_id,
                    &measurement.location_id,
                    &measurement.measurement_value,
                    &measurement.measurement_time,
                ],
            )
            .await?
            .iter()
            .map(|row| Measurements::from_row_ref(row).unwrap())
            .collect::<Vec<Measurements>>()
            .pop()
            .ok_or(MyError::NotFound);

        println!("res {:?}", res);
        res
    }

    pub async fn insert_measurement_type(client: &Client) -> Result<MeasurementTypes, MyError> {
        let _stmt = include_str!("../queries/insert_measurement_type.sql");
        let _stmt = _stmt.replace("$table_fields", &MeasurementTypes::sql_table_fields());
        println!("_stmt {:?}", _stmt);

        let stmt = client.prepare(&_stmt).await.unwrap();

        println!("done prema");
        client
            .query(&stmt, &[&String::from("test")])
            .await?
            .iter()
            .map(|row| MeasurementTypes::from_row_ref(row).unwrap())
            .collect::<Vec<MeasurementTypes>>()
            .pop()
            .ok_or(MyError::NotFound)
    }
}

mod handlers {
    use crate::{db, errors::MyError, models::*};
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    pub async fn insert_measurement(
        db_pool: web::Data<Pool>,
        measurement_data: web::Json<Measurements>,
    ) -> Result<HttpResponse, Error> {
        let measurement: Measurements = measurement_data.into_inner();
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_measurement = db::insert_measurement(&client, measurement).await?;
        println!("done inserting");
        println!("new_measurement {:?}", new_measurement);

        Ok(HttpResponse::Ok().json(new_measurement))
    }

    pub async fn insert_measurement_type(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_measurement = db::insert_measurement_type(&client).await?;
        println!("done inserting");
        println!("new_measurement {:?}", new_measurement);

        Ok(HttpResponse::Ok().json(new_measurement))
    }
}

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use handlers::*;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Server is starting up");

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(web::resource("/measurement/new").route(web::post().to(insert_measurement)))
            .service(
                web::resource("/measurement-type/new")
                    .route(web::post().to(insert_measurement_type)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server is running");

    server.await
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("GET /");

    HttpResponse::Ok().body("hello fam swal")
}
