use crate::pg::PgPool;
use crate::user::User;
use warp::{Filter, Reply, reply::Response};

pub fn all_routes(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    router::update_user_route(pool.clone())
        .or(router::get_user_route(pool.clone()))
        .or(router::create_user_route(pool.clone()))
        .or(router::health_route())
}

// Routes
//

pub mod router {
    use crate::user::{NewUser, UpdateUser};
    use crate::{
        pg::{with_db_access_manager, PgPool},
        utils::json_body,
    };
    use warp::Filter;

    pub fn create_user_route(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("users")
            .and(warp::post())
            .and(with_db_access_manager(pool))
            .and(json_body::<NewUser>())
            .and_then(super::handlers::create_user_handler)
    }

    pub fn update_user_route(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("users")
            .and(warp::put())
            .and(warp::path::param())
            .and(with_db_access_manager(pool))
            .and(json_body::<UpdateUser>())
            .and_then(super::handlers::update_user_handler)
    }

    pub fn get_user_route(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("users")
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db_access_manager(pool))
            .and_then(super::handlers::get_user_handler)
    }

    pub fn health_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("health")
            .and(warp::get())
            .and_then(super::handlers::health_handler)
    }
}

impl Reply for User {
    fn into_response(self) -> Response {
        Response::new
            (format!(
                    "username: {}, user_info: {}, id: {}, wallet_address: {}",
                    self.username,
                    self.user_info,
                    self.id,
                    self.wallet_address
            ).into())
    }
}


// Handlers
//
pub mod handlers {
    use crate::{
        pg::DBAccessManager,
        user::{NewUser, UpdateUser, User},
    };
    use warp::{Rejection, Reply};

    pub async fn health_handler() -> Result<impl Reply, Rejection> {
        Ok("success")
    }

    pub async fn get_user_handler(uid: i32, db: DBAccessManager) -> Result<impl Reply, Rejection> {
        let user: User = db.get_user(uid)?;
        Ok(warp::reply::json(&user).into_response())
    }

    pub async fn update_user_handler(
        param: String,
        db: DBAccessManager,
        data: UpdateUser,
    ) -> Result<impl Reply, Rejection> {
        let uid = param.parse::<i32>().unwrap();
        let user: User = db.update_user(uid, data)?;
        Ok(warp::reply::json(&user).into_response())
    }

    pub async fn create_user_handler(
        db: DBAccessManager,
        data: NewUser,
    ) -> Result<impl Reply, Rejection> {
        let user: User = db.create_user(data)?;
        Ok(warp::reply::json(&user).into_response())
    }
}
