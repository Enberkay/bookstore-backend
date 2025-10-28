use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{
    infrastructure::postgres::schema::roles,
    domain::entities::role::RoleEntity,
};

// ======================
// RoleModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = roles)]
#[diesel(primary_key(id))]
pub struct RoleModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<RoleModel> for RoleEntity {
    fn from(model: RoleModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
        }
    }
}

impl From<RoleEntity> for RoleModel {
    fn from(entity: RoleEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
        }
    }
}
