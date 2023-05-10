use crate::database::RepositoryProvider;
use crate::entities::{
    order::{Order, OrderInput},
    user::UserInput,
};
use crate::repositories::order::OrdersRepository;
use crate::repository_impl::product::ProductInput;
use crate::usecases::product;
use crate::usecases::user;
use sqlx::Error;
use uuid::Uuid;

pub async fn create(repo: &RepositoryProvider, input: OrderInput) -> Result<Order, Error> {
    let found_user = user::find_user_by_user_id(repo, input.user_id).await?;
    let updated_user = UserInput {
        display_name: found_user.display_name,
        email: found_user.email,
        password: found_user.password,
        point: found_user.point - input.total,
        is_admin: found_user.is_admin,
    };
    user::save(repo, updated_user).await?;
    let orders = repo.orders();
    let saved_order = orders.save(input.clone()).await?;
    let order_items = input.items.iter().map(|item| async move {
        if let Some(saved_item) = product::find_product_by_id(repo, item.product_id).await? {
            Ok(ProductInput {
                name: saved_item.name,
                price: saved_item.price,
                stock: saved_item.stock - item.quantity,
            })
        } else {
            return Err(Error::RowNotFound);
        }
    });
    for p in order_items {
        product::save(repo, p.await?).await?;
    }
    Ok(saved_order)
}

pub async fn delete(repo: &RepositoryProvider, id: Uuid) -> Result<Order, Error> {
    let orders = repo.orders();
    orders.delete(id).await
}

pub async fn find_order_by_id(repo: &RepositoryProvider, id: Uuid) -> Result<Option<Order>, Error> {
    let orders = repo.orders();
    let all_order = orders.list().await?;
    let order = all_order.into_iter().find(|order| order.id == id);
    Ok(order)
}

pub async fn list_order(repo: &RepositoryProvider) -> Result<Vec<Order>, Error> {
    let orders = repo.orders();
    orders.list().await
}
