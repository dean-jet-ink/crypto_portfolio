use async_graphql::{Context, Object};

use crate::app::{
    domain::repositories::user::user_repository::UserRepository,
    presentation::resolvers::resolver_base::ResolverBase,
};

#[derive(Default)]
pub struct HelloQuery;

impl ResolverBase for HelloQuery {}

#[Object]
impl HelloQuery {
    pub async fn hello(&self, ctx: &Context<'_>) -> &str {
        let repo = self.resolve_ref::<dyn UserRepository>(ctx);

        repo.find_by_id("Hello, world!");

        "Hello world!"
    }
}
