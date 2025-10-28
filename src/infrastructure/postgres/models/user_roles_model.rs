use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable, Identifiable};

use crate::infrastructure::postgres::schema::user_roles;

// ======================
// UserRoleModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = user_roles)]
#[diesel(primary_key(user_id, role_id))]
pub struct UserRoleModel {
    pub user_id: i32,
    pub role_id: i32,
    pub assigned_at: DateTime<Utc>,
}
