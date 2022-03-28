use crate::error::{AppError, ErrorType};
use crate::schema::users::dsl::*;
use crate::user::{NewUser, User, UpdateUser};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use warp::{reject, Filter};

pub struct DBAccessManager {
    connection: PooledPg,
}

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn get_user(&self, uid: i32) -> Result<User, AppError> {
        let result = users.filter(id.eq(&uid))
            .first(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while getting user"));
        let result = result.unwrap();

        Ok(result)
    }

    pub fn update_user(&self, uid: i32, user: UpdateUser) -> Result<User, AppError> {
        let target = users::filter(crate::schema::users::table, id.eq(&uid));
        diesel::update(target)
            .set((
                user_info.eq(user.user_info),
                username.eq(user.username)
            ))
            .get_result(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating user"))
    }

    pub fn create_user(&self, user: NewUser) -> Result<User, AppError> {
        diesel::insert_into(crate::schema::users::table)
            .values(user)
            .get_result(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating user"))
    }
}

pub fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}
