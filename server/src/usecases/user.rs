use crate::database::RepositoryProvider;
use crate::entities::user::User;
use crate::repositories::user::UsersRepository;
use crate::repository_impl::user::UserInput;
use sqlx::Error;

pub async fn save(repo: &RepositoryProvider, input: UserInput) -> Result<User, Error> {
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

pub async fn find_user_by_email(
    repo: &RepositoryProvider,
    email: String,
) -> Result<Option<User>, Error> {
    let users = repo.users();
    let all_user = users.list().await?;
    let user = all_user.into_iter().find(|user| user.email == email);
    Ok(user)
}

pub async fn find_user_by_user_id(repo: &RepositoryProvider, id: i32) -> Result<User, Error> {
    let users = repo.users();
    let all_user = users.list().await?;
    let user = all_user
        .into_iter()
        .filter(|user| user.id == id)
        .collect::<Vec<User>>();
    if user.len() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(user[0].clone())
}
