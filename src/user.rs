use super::schema::users;
use diesel::pg::types::sql_types::Jsonb;
use diesel::pg::Pg;
use diesel::serialize::Output;
use diesel::types::{FromSql, ToSql};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

#[derive(FromSqlRow, AsExpression, Serialize, Deserialize, Debug, Default)]
#[sql_type = "Jsonb"]
pub struct SocialNetworks {
    pub instagram: Option<String>,
}

impl FromSql<Jsonb, Pg> for SocialNetworks {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for SocialNetworks {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub wallet_address: String,
    pub username: String,
    pub user_info: String,
    pub social_networks: SocialNetworks,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub wallet_address: String,
    pub username: String,
    pub user_info: String,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: String,
    pub user_info: String,
}
