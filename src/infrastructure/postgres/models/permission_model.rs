use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{
    infrastructure::postgres::schema::permissions,
    domain::entities::permission::PermissionEntity,
};

// ======================
// PermissionModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = permissions)]
#[diesel(primary_key(id))]
pub struct PermissionModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<PermissionModel> for PermissionEntity {
    fn from(model: PermissionModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
        }
    }
}

impl From<PermissionEntity> for PermissionModel {
    fn from(entity: PermissionEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
        }
    }
}
