use crate::database::RepositoryProvider;
use crate::entities::product::Product;
use crate::repositories::product::ProductsRepository;
use crate::repository_impl::product::ProductInput;
use sqlx::Error;

pub async fn save(repo: &RepositoryProvider, input: ProductInput) -> Result<Product, Error> {
    let products = repo.products();
    products.save(input).await
}

pub async fn delete(repo: &RepositoryProvider, id: i32) -> Result<Product, Error> {
    let products = repo.products();
    products.delete(id).await
}

pub async fn find_product_by_id(
    repo: &RepositoryProvider,
    id: i32,
) -> Result<Option<Product>, Error> {
    let products = repo.products();
    let all_product = products.list().await?;
    let product = all_product.into_iter().find(|product| product.id == id);
    Ok(product)
}

pub async fn list_product(repo: &RepositoryProvider) -> Result<Vec<Product>, Error> {
    let products = repo.products();
    products.list().await
}
