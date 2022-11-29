use async_graphql::{Context, InputObject, Object};
use sqlx::postgres::PgPool;

use crate::model::query::Customer;
use crate::model::query::MenuRecord;

#[derive(InputObject)]
struct CustomerInput {
    menu: String,
    price: i32,
}

#[derive(InputObject)]
struct Menu {
    menu: String,
    price: i32,
    stock: i32,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_new_order(
        &self,
        ctx: &Context<'_>,
        input: CustomerInput,
    ) -> Result<Customer, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql =
            "insert into customer (menu, price, ordered_at) values ($1, $2, CURRENT_TIMESTAMP) returning id, menu, price, ordered_at;";
        let customer: Customer = sqlx::query_as(sql)
            .bind(input.menu)
            .bind(input.price)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(customer)
    }

    async fn add_new_menu(
        &self,
        ctx: &Context<'_>,
        input: Menu,
    ) -> Result<MenuRecord, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql = "insert into menu (menu, price, stock) values ($1, $2, $3) returning id, menu, price, stock;";
        let menu: MenuRecord = sqlx::query_as(sql)
            .bind(input.menu)
            .bind(input.price)
            .bind(input.stock)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(menu)
    }
}
