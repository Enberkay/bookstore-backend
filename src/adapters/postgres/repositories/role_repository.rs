use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::{
    entities::role::RoleEntity,
    repositories::role_repository::RoleRepository,
};
use crate::adapters::postgres::models::role_model::RoleModel;

pub struct PostgresRoleRepository {
    pool: PgPool,
}

impl PostgresRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleEntity>> {
        let result = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(RoleEntity::from))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<RoleEntity>> {
        let results = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE id = ANY($1)
            ORDER BY id ASC
            "#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(RoleEntity::from).collect())
    }

    async fn find_all(&self) -> Result<Vec<RoleEntity>> {
        let results = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(RoleEntity::from).collect())
    }

    async fn save(&self, role: &RoleEntity) -> Result<i32> {
        let row = sqlx::query(
            r#"
            INSERT INTO roles
                (name, description, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(&role.name)
        .bind(&role.description)
        .bind(role.created_at)
        .bind(role.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get("id")?)
    }

    async fn update(
        &self,
        id: i32,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<RoleEntity> {
        let result = sqlx::query_as::<_, RoleModel>(
            r#"
            UPDATE roles
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, name, description, created_at, updated_at
            "#,
        )
        .bind(name.as_deref())
        .bind(description.as_deref())
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(RoleEntity::from(result))
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
