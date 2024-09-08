use async_graphql::Context;
use shaku::{HasComponent, Interface};

use crate::di::di_container::DiContainer;

pub trait ResolverBase {
    fn di_container<'a>(&self, ctx: &Context<'a>) -> &'a DiContainer {
        ctx.data::<DiContainer>()
            .expect("DiContainer not found in context")
    }

    fn resolve_ref<'a, T: Interface + ?Sized>(&self, ctx: &Context<'a>) -> &'a T
    where
        DiContainer: HasComponent<T>,
    {
        let container = self.di_container(ctx);
        container.resolve_ref()
    }
}
