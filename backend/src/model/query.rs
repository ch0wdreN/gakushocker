use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgPool, FromRow};

pub struct QueryRoot;

#[derive(FromRow, SimpleObject)]
pub struct MenuRecord {
    pub id: i32,
    pub menu: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(FromRow, SimpleObject)]
pub struct Customer {
    id: i32,
    menu: String,
    price: i32,
    ordered_at: chrono::NaiveDateTime,
}

#[Object]
impl QueryRoot {
    async fn get_all_menu(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MenuRecord>, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, stock from menu";
        let menu = sqlx::query_as(sql).fetch_all(pool).await?;
        Ok(menu)
    }

    async fn get_menu_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<MenuRecord, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, stock from menu where id = $1";
        let menu: MenuRecord = sqlx::query_as(sql).bind(id).fetch_one(pool).await?;
        Ok(menu)
    }

    async fn get_all_order(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<Customer>, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, ordered_at from customer";
        let customers: Vec<Customer> = sqlx::query_as(sql).fetch_all(pool).await?;
        Ok(customers)
    }

    async fn get_order_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Customer, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let sql = "select id, menu, price, ordered_at from customer where id = $1";
        let customer: Customer = sqlx::query_as(sql).bind(id).fetch_one(pool).await?;
        Ok(customer)
    }
}
