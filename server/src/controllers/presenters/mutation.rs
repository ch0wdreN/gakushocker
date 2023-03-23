use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::entities::product::Product;
use crate::entities::user::User;
use crate::repository_impl::order::OrderInput;
use crate::repository_impl::product::ProductInput;
use crate::repository_impl::user::UserInput;
use crate::usecases::order;
use crate::usecases::product;
use crate::usecases::user;
use async_graphql::{Context, Object};
use sqlx::Error;
use uuid::Uuid;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_order(&self, ctx: &Context<'_>, input: OrderInput) -> Result<Order, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        order::create(repo, input).await
    }

    async fn delete_order(&self, ctx: &Context<'_>, id: Uuid) -> Result<Order, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        order::delete(repo, id).await
    }

    async fn create_product(
        &self,
        ctx: &Context<'_>,
        input: ProductInput,
    ) -> Result<Product, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        product::save(repo, input).await
    }

    async fn delete_product(&self, ctx: &Context<'_>, id: i32) -> Result<Product, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        product::delete(repo, id).await
    }

    async fn create_user(&self, ctx: &Context<'_>, input: UserInput) -> Result<User, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        user::save(repo, input).await
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<User, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        user::delete_user(repo, id).await
    }
}
