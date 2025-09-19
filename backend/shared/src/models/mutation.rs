use async_graphql::*;
use crate::{
    models::user::GraphQLUser,
    database::repositories::user_repository::UserRepository,
    database::services::user_service::UserService,
};

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<GraphQLUser> {
        let user_service = ctx.data::<UserService>()?;

        let password_hash = format!("hashed_{}", input.password);
        let user_profile = user_service.create(&input.username, &input.email, &password_hash).await?;

        Ok(user_profile.into())
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}