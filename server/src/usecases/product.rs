use crate::database::RepositoryProvider;
use crate::entities::product::Product;
use crate::repositories::product::{ProductInput, ProductsRepository};

pub async fn create(repository_provider: &RepositoryProvider, input: ProductInput) -> Product {
    let products = repository_provider.products();
    products.save(input).await.unwrap()
}

pub async fn get_all_product(repository_provider: &RepositoryProvider) -> Vec<Product> {
    let products = repository_provider.products();
    products.get_all().await.unwrap()
}
