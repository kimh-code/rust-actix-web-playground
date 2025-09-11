use time::{OffsetDateTime, 
};
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::{
    database::models::db_user::DbUser,
};
use async_graphql::{Scalar, ScalarType, InputValueError, InputValueResult, Value, ID, SimpleObject, ComplexObject};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeOffsetDateTime(pub OffsetDateTime);

#[Scalar]
impl ScalarType for TimeOffsetDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                OffsetDateTime::parse(&s, &time::format_description::well_known::Iso8601::DEFAULT)
                    .map(TimeOffsetDateTime)
                    .map_err(|_| InputValueError::custom("Invalid date format"))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }
    fn to_value(&self) -> Value {
        Value::String(
            self.0.format(&time::format_description::well_known::Iso8601::DEFAULT)
                .unwrap_or_default()
        )
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct GraphQLUser {
    pub id: ID,
    pub username: String,
    pub email: String,
    // password_hash 제외,
    // is_deleted 제외,
    // internal_notes 제외,
    pub created_at: TimeOffsetDateTime,
    pub updated_at: TimeOffsetDateTime,
}

impl From<DbUser> for GraphQLUser {
    fn from(db_user: DbUser) -> Self {
        GraphQLUser {
            id: ID(db_user.id.to_string()),
            username: db_user.username,
            email: db_user.email,
            created_at: TimeOffsetDateTime(db_user.created_at),
            updated_at: TimeOffsetDateTime(db_user.updated_at),
        }
    }
}

#[ComplexObject]
impl GraphQLUser {
    async fn profile_id(&self) -> ID {
        ID(format!("profile_{}", self.id.0))
    }
    async fn display_name(&self) -> String {
        format!("@{}", self.username)
    }
}

#[derive(Serialize)]
pub struct RestUser {
    pub id: String,
    pub username: String,
}

impl From<DbUser> for RestUser {
    fn from(db_user: DbUser) -> Self {
        RestUser {
            id: db_user.id.to_string(),
            username: db_user.username,
        }
    }
}