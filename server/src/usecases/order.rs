use crate::database::RepositoryProvider;
use crate::entities::order::Order;
use crate::repositories::order::{OrderInput, OrdersRepository};

pub async fn create(repository_provider: &RepositoryProvider, input: OrderInput) -> Order {
    let orders = repository_provider.orders();
    orders.save(input).await.unwrap()
}

pub async fn get_all_order(repository_provider: &RepositoryProvider) -> Vec<Order> {
    let orders = repository_provider.orders();
    orders.get_all().await.unwrap()
}
