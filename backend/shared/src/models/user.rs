use uuid::Uuid;
use time::{OffsetDateTime, 
};
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::{
    database::models::db_user::DbUser,
};
use async_graphql::{Scalar, ScalarType, InputValueError, InputValueResult, Value, ID, SimpleObject, ComplexObject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<DbUser> for UserProfile {
    fn from(db_user: DbUser) -> Self {
        UserProfile {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}

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
    pub created_at: TimeOffsetDateTime,
    pub updated_at: TimeOffsetDateTime,
}

impl From<UserProfile> for GraphQLUser {
    fn from(profile: UserProfile) -> Self {
        GraphQLUser {
            id: ID(profile.id.to_string()),
            username: profile.username,
            email: profile.email,
            created_at: TimeOffsetDateTime(profile.created_at),
            updated_at: TimeOffsetDateTime(profile.updated_at),
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

impl From<UserProfile> for RestUser {
    fn from(profile: UserProfile) -> Self {
        RestUser {
            id: profile.id.to_string(),
            username: profile.username,
        }
    }
}