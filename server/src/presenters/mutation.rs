use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::entities::product::Product;
use crate::repositories::order::OrderInput;
use crate::repositories::product::ProductInput;
use crate::usecases::order;
use crate::usecases::product;
use crate::usecases::user;
use async_graphql::{Context, Object};
use crate::entities::user::User;
use crate::repositories::user::UserInput;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_order(&self, ctx: &Context<'_>, input: OrderInput) -> Order {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
        let created_order = order::create(repository_provider, input).await;
        created_order
    }

    async fn create_product(&self, ctx: &Context<'_>, input: ProductInput) -> Product {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
        let created_product = product::create(repository_provider, input).await;
        created_product
    }

    async fn create_user(&self, ctx: &Context<'_>, input: UserInput) -> User {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
        let created_user = user::create(repository_provider, input).await;
        created_user
    }
}
