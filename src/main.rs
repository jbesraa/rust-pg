extern crate dotenv;
#[macro_use]
extern crate diesel;
use crate::error::handle_rejection;
use crate::pg::pg_pool;
use crate::routes::all_routes;
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;
mod error;
mod pg;
mod routes;
mod schema;
mod user;
mod utils;

#[tokio::main]
async fn main() {
    let service_name: &str = "@auto-profile-service";
    println!("Starting Service {}", &service_name);
    dotenv().ok();
    pretty_env_logger::init();
    env::var("DATABASE_URL").expect("DATABASE_URL env is not set");
    let default_port: u16 = 4062;
    let app_port = env::var("APP_PORT").unwrap_or(default_port.to_string());
    let socket = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        app_port.parse::<u16>().unwrap_or(default_port),
    );
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let pool = pg_pool(database_url.as_str());
    let routes = all_routes(pool).recover(handle_rejection);
    // TLS Certificates
    let tls_key_file = "localhost+2-key.pem";
    let tls_cert_file = "localhost+2.pem";

    warp::serve(routes.with(warp::log(service_name)))
        .tls()
        .key_path(tls_key_file)
        .cert_path(tls_cert_file)
        .run((socket.ip(), socket.port()))
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use user::{NewUser, UpdateUser};

    #[tokio::test]
    async fn health() {
        let res = warp::test::request()
            .method("GET")
            .path("/health")
            .reply(&routes::router::health_route())
            .await;

        assert_eq!(res.status(), 200, "return 200");
        assert_eq!(res.body(), "success", "return success in body");
    }

    #[tokio::test]
    async fn create_user() {
        let db_url="postgres://esraajbara:admin@localhost:5432/user-auth-service";
        let pool = pg_pool(db_url);
        let new_user = NewUser {
            wallet_address: "2x".to_string(),
            username: "1nam2".to_string(),
            user_info: "inf1me".to_string(),
        };
        let res = warp::test::request()
            .method("POST")
            .path("/users")
            .json(&new_user)
            .reply(&routes::router::create_user_route(pool))
            .await;

        assert_eq!(res.status(), 200, "return 200");
    }

    #[tokio::test]
    async fn get_user() {
        let db_url="postgres://esraajbara:admin@localhost:5432/user-auth-service";
        let pool = pg_pool(db_url);
        let res = warp::test::request()
            .method("GET")
            .path("/users/1")
            .reply(&routes::router::get_user_route(pool))
            .await;

        assert_eq!(res.status(), 200, "return 200");
    }

    #[tokio::test]
    async fn update_user() {
        let db_url="postgres://esraajbara:admin@localhost:5432/user-auth-service";
        let pool = pg_pool(db_url);
        let update = UpdateUser {
            username: "unam2".to_string(),
            user_info: "inf1me".to_string(),
        };
        let res = warp::test::request()
            .method("PUT")
            .path("/users/1")
            .json(&update)
            .reply(&routes::router::update_user_route(pool))
            .await;

        assert_eq!(res.status(), 200, "return 200");
    }
}
