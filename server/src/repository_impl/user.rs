use crate::database::ConnectionPool;
use crate::entities::user::User;
use crate::repositories::user::UsersRepository;
use async_graphql::InputObject;
use sqlx::Error;

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
        let tx = pool.begin().await.unwrap();
        let saved_user = sqlx::query_as(&sql)
            .bind(input.display_name)
            .bind(input.email)
            .bind(input.password)
            .bind(input.point)
            .fetch_one(pool)
            .await;
        tx.commit().await.unwrap();
        saved_user
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
        let tx = pool.begin().await.unwrap();
        let deleted_user = sqlx::query_as(&sql).bind(id).fetch_one(pool).await;
        tx.commit().await.unwrap();
        deleted_user
    }

    async fn list(&self) -> Result<Vec<User>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM users";
        let users = sqlx::query_as(&sql).fetch_all(pool).await;
        users
    }
}
