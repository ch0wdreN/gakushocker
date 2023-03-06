use crate::database::RepositoryProvider;
use crate::entities::user::User;
use crate::repositories::user::UsersRepository;
use crate::repository_impl::user::UserInput;
use sqlx::Error;

pub async fn create(repo: &RepositoryProvider, input: UserInput) -> Result<User, Error> {
    let users = repo.users();
    users.save(input).await
}

pub async fn delete_user(repo: &RepositoryProvider, id: i32) -> Result<User, Error> {
    let users = repo.users();
    users.delete(id).await
}

pub async fn list_user(repo: &RepositoryProvider) -> Result<Vec<User>, Error> {
    let users = repo.users();
    users.list().await
}

pub async fn find_user_by_email(repo: &RepositoryProvider, email: String) -> User {
    let users = repo.users();
    let all_user = users.list().await.unwrap();
    let user = all_user
        .into_iter()
        .filter(|user| user.email == email)
        .collect::<Vec<User>>();
    user[0].clone()
}
