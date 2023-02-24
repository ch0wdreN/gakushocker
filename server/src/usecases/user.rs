use crate::database::RepositoryProvider;
use crate::entities::user::User;
use crate::repositories::user::{UserInput, UsersRepository};

pub async fn create(repository_provider: &RepositoryProvider, input: UserInput) -> User {
    let users = repository_provider.users();
    users.save(input).await.unwrap()
}

pub async fn get_all_user(repository_provider: &RepositoryProvider) -> Vec<User> {
    let users = repository_provider.users();
    users.get_all().await.unwrap()
}