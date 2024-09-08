use async_graphql::MergedObject;

mod auth_mutation;

#[derive(MergedObject, Default)]
pub struct MutationRoot(auth_mutation::AuthMutation);
