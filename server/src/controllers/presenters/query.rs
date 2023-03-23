use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::entities::product::Product;
use crate::entities::user::User;
use crate::usecases::order;
use crate::usecases::product;
use crate::usecases::user;
use async_graphql::{Context, Object};
use sqlx::Error;
use uuid::Uuid;

pub struct Query;

#[Object]
impl Query {
    async fn list_order(&self, ctx: &Context<'_>) -> Vec<Order> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        order::list_order(repo).await.unwrap()
    }

    async fn find_order_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Order>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        order::find_order_by_id(repo, id).await
    }

    async fn find_all_order(&self, ctx: &Context<'_>) -> Result<Vec<Order>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        order::list_order(repo).await
    }

    async fn list_product(&self, ctx: &Context<'_>) -> Result<Vec<Product>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        product::list_product(repo).await
    }

    async fn find_product_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<Product>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        product::find_product_by_id(repo, id).await
    }

    async fn list_user(&self, ctx: &Context<'_>) -> Result<Vec<User>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        user::list_user(repo).await
    }

    async fn find_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<User>, Error> {
        let repo = ctx.data::<RepositoryProvider>().unwrap();
        user::find_user_by_email(repo, email).await
    }
}
