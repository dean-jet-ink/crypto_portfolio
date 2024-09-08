use async_graphql::{Context, Object};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn login(&self, _ctx: &Context<'_>) -> &str {
        "login"
    }

    pub async fn login_with_credentials(&self, _ctx: &Context<'_>) -> &str {
        "loginWithCredentials"
    }

    pub async fn signup(&self, _ctx: &Context<'_>) -> &str {
        "signup"
    }
}
