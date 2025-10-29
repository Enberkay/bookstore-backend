use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::{
    entities::user::UserEntity,
    repositories::user_repository::UserRepository,
};
use crate::infrastructure::postgres::models::user_model::UserModel;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   branch_id, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(UserEntity::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   branch_id, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(UserEntity::from))
    }

    async fn find_all(&self) -> Result<Vec<UserEntity>> {
        let results = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   branch_id, is_active, created_at, updated_at
            FROM users
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(UserEntity::from).collect())
    }

    async fn save(&self, user: &UserEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users
                (id, fname, lname, email, age, sex, phone, password,
                 branch_id, is_active, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id)
            DO UPDATE SET
                fname = EXCLUDED.fname,
                lname = EXCLUDED.lname,
                email = EXCLUDED.email,
                age = EXCLUDED.age,
                sex = EXCLUDED.sex,
                phone = EXCLUDED.phone,
                password = EXCLUDED.password,
                branch_id = EXCLUDED.branch_id,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
            user.id,
            user.first_name,
            user.last_name,
            user.email.as_str(),
            user.age,
            user.sex,
            user.phone,
            user.password,
            user.branch_id,
            user.is_active,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
