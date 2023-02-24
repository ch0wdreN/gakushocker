use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::entities::product::Product;
use crate::usecases::order;
use crate::usecases::product;
use crate::usecases::user;
use async_graphql::{Context, Object};
use crate::entities::user::User;

pub struct Query;

#[Object]
impl Query {
    async fn list_order(&self, ctx: &Context<'_>) -> Vec<Order> {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
        order::get_all_order(repository_provider).await
    }

    async fn list_product(&self, ctx: &Context<'_>) -> Vec<Product> {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
        product::get_all_product(repository_provider).await
    }

    async fn list_user(&self, ctx: &Context<'_>) -> Vec<User> {
        let repository_provider = ctx.data::<RepositoryProvider>().unwrap();
       user::get_all_user(repository_provider).await
    }
}
