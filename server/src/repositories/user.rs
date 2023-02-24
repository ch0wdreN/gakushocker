use crate::database::ConnectionPool;
use crate::entities::user::User;
use anyhow::Result;
use async_graphql::InputObject;

#[axum::async_trait]
pub trait UsersRepository {
    async fn save(&self, input: UserInput) -> Result<User>;
    async fn delete(&self, id: i32) -> Result<User>;
    async fn find_by_email(&self, email: String) -> Result<User>;
    async fn get_all(&self) -> Result<Vec<User>>;
}

#[derive(InputObject)]
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
    async fn save(&self, input: UserInput) -> Result<User> {
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
                password=EXCLUDED.password
                point=EXCLUDED.point
            RETURNING
                id, display_name, email, password, point
        "#;
        let tx = pool.begin().await.unwrap();
        let saved_user = sqlx::query_as(&sql)
            .bind(input.display_name)
            .bind(input.email)
            .bind(input.password)
            .bind(input.point)
            .fetch_one(pool)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        Ok(saved_user)
    }

    async fn delete(&self, id: i32) -> Result<User> {
        let pool = self.pool;
        let sql = r#"
            DELETE FROM
                users
            WHERE
                id=$1
            RETURNING
                id, display_name, email, password, point
        "#;
        let tx = pool.begin().await.unwrap();
        let deleted_user = sqlx::query_as(&sql).bind(id).fetch_one(pool).await.unwrap();
        tx.commit().await.unwrap();
        Ok(deleted_user)
    }

    async fn find_by_email(&self, email: String) -> Result<User> {
        let pool = self.pool;
        let sql = r#"
            SELECT * FROM
                users
            WHERE
                email=$1
        "#;
        let user = sqlx::query_as(&sql)
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap();
        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<User>> {
        let pool = self.pool;
        let sql = "SELECT * FROM users";
        let users = sqlx::query_as(&sql).fetch_all(pool).await.unwrap();
        Ok(users)
    }
}
