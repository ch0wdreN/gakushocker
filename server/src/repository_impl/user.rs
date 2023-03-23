use crate::database::ConnectionPool;
use crate::entities::user::User;
use crate::repositories::user::UsersRepository;
use async_graphql::InputObject;
use serde::Deserialize;
use sqlx::Error;

#[derive(InputObject, Deserialize)]
pub struct UserInput {
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub point: i32,
}

pub struct UsersImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> UsersRepository for UsersImpl<'a> {
    async fn save(&self, input: UserInput) -> Result<User, Error> {
        let pool = self.pool;
        let sql = r#"
            INSERT INTO
                users (display_name, email, password, point)
            VALUES
                ($1, $2, $3, $4)
            ON CONFLICT
                (email)
            DO UPDATE SET
                display_name=EXCLUDED.display_name,
                email=EXCLUDED.email,
                password=EXCLUDED.password,
                point=EXCLUDED.point
            RETURNING
                id, display_name, email, password, point
        "#;
        let mut tx = pool.begin().await?;
        let saved_user: User = match sqlx::query_as(&sql)
            .bind(input.display_name)
            .bind(input.email)
            .bind(input.password)
            .bind(input.point)
            .fetch_one(&mut tx)
            .await {
            Ok(u) => u,
            Err(_) => {
                tx.rollback().await?;
                return Err(Error::RowNotFound);
            }
        };
        tx.commit().await?;
        Ok(saved_user)
    }

    async fn delete(&self, id: i32) -> Result<User, Error> {
        let pool = self.pool;
        let sql = r#"
            DELETE FROM
                users
            WHERE
                id=$1
            RETURNING
                id, display_name, email, password, point
        "#;
        let mut tx = pool.begin().await?;
        let deleted_user: User = match sqlx::query_as(&sql).bind(id).fetch_one(&mut tx).await {
            Ok(u) => u,
            Err(_) => {
                tx.rollback().await?;
                return Err(Error::RowNotFound);
            }
        };
        tx.commit().await?;
        Ok(deleted_user)
    }

    async fn list(&self) -> Result<Vec<User>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM users";
        let users = sqlx::query_as(&sql).fetch_all(pool).await;
        users
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::db_pool;
    use crate::usecases::order::delete;

    #[tokio::test]
    async fn test_user_repository() -> Result<(), Error> {
        let pool = db_pool().await;
        let user_impl = UsersImpl { pool: &pool };
        let user = UserInput {
            display_name: "test".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
            point: 0,
        };

        let tx = pool.begin().await?;
        let saved_user = user_impl.save(user).await?;
        let expected_user = User {
            id: saved_user.id,
            display_name: "test".to_string(),
            email: "test@email.com".to_string(),
            password: "password".to_string(),
            point: 0,
        };
        assert_eq!(expected_user, saved_user);
        let all_user = user_impl.list().await?;
        let expect_found_user = expected_user;
        let found_user = all_user
            .into_iter()
            .find(|user| user.id == expect_found_user.id);
        assert!(found_user.is_some());
        let expect_delete_user = expect_found_user;
        let deleted_user = user_impl.delete(saved_user.id).await?;
        assert_eq!(expect_delete_user, deleted_user);
        Ok(())
    }
}
