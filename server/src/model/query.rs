use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgPool, FromRow};

pub struct Query;

#[derive(FromRow, SimpleObject)]
pub struct MenuRecord {
    pub id: i32,
    pub menu: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(FromRow, SimpleObject)]
pub struct Order {
    id: i32,
    menu: String,
    price: i32,
    ordered_at: chrono::NaiveDateTime,
}

#[Object]
impl Query {
    async fn list_menu(&self, ctx: &Context<'_>) -> Result<Vec<MenuRecord>, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, stock from menu";
        let menu = sqlx::query_as(sql).fetch_all(pool).await?;
        Ok(menu)
    }

    async fn get_menu(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<MenuRecord, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, stock from menu where id = $1";
        let menu: MenuRecord = sqlx::query_as(sql).bind(id).fetch_one(pool).await?;
        Ok(menu)
    }

    async fn list_order(&self, ctx: &Context<'_>) -> Result<Vec<Order>, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, ordered_at from orders";
        let orders: Vec<Order> = sqlx::query_as(sql).fetch_all(pool).await?;
        Ok(orders)
    }

    async fn get_order(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Order, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, ordered_at from orders where id = $1";
        let order: Order = sqlx::query_as(sql).bind(id).fetch_one(pool).await?;
        Ok(order)
    }
}
