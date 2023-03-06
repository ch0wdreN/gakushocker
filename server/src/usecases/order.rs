use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::repositories::order::OrdersRepository;
use crate::repository_impl::order::OrderInput;
use sqlx::Error;
use uuid::Uuid;

pub async fn create(repo: &RepositoryProvider, input: OrderInput) -> Result<Order, Error> {
    let orders = repo.orders();
    orders.save(input).await
}

pub async fn delete(repo: &RepositoryProvider, id: Uuid) -> Result<Order, Error> {
    let orders = repo.orders();
    orders.delete(id).await
}

pub async fn find_order_by_id(repo: &RepositoryProvider, id: Uuid) -> Result<Order, Error> {
    let orders = repo.orders();
    let all_order = orders.list().await?;
    let order = all_order
        .into_iter()
        .filter(|order| order.id == id)
        .collect::<Vec<Order>>();
    Ok(order[0].clone())
}

pub async fn list_order(repo: &RepositoryProvider) -> Result<Vec<Order>, Error> {
    let orders = repo.orders();
    orders.list().await
}
