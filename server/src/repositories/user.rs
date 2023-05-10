use crate::entities::user::{User, UserInput};
use sqlx::Error;

#[axum::async_trait]
pub trait UsersRepository {
    async fn save(&self, input: UserInput) -> Result<User, Error>;
    async fn delete(&self, id: i32) -> Result<User, Error>;
    async fn list(&self) -> Result<Vec<User>, Error>;
}
