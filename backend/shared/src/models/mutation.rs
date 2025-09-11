use async_graphql::*;
use crate::{
    models::user::GraphQLUser,
    database::repositories::user_repository::UserRepository,
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
        let user_repo = ctx.data::<UserRepository>()?;

        let password_hash = format!("hashed_{}", input.password);
        let db_user = user_repo.create(&input.username, &input.email, &password_hash).await?;

        Ok(db_user.into())
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}