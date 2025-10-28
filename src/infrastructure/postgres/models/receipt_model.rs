use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive};

use crate::{
    infrastructure::postgres::schema::receipts,
    domain::{
        entities::receipt::ReceiptEntity,
        value_objects::{money::Money, receipt_code::ReceiptCode},
    },
};

// ======================
// ReceiptModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = receipts)]
#[diesel(primary_key(id))]
pub struct ReceiptModel {
    pub id: i32,
    pub receipt_code: String,
    pub type_name: String,
    pub reference_id: i32,
    pub source: String,
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: BigDecimal,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<ReceiptModel> for ReceiptEntity {
    fn from(model: ReceiptModel) -> Self {
        Self {
            id: model.id,
            receipt_code: ReceiptCode::new(&model.receipt_code)
                .expect("Invalid receipt code format"),
            type_name: model.type_name,
            reference_id: model.reference_id,
            source: model.source,
            user_id: model.user_id,
            branch_id: model.branch_id,
            total_amount: Money::new(model.total_amount.to_f64().unwrap_or(0.0))
                .expect("Invalid total amount"),
            payment_method: model.payment_method,
            payment_ref: model.payment_ref,
            issued_at: model.issued_at,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<ReceiptEntity> for ReceiptModel {
    fn from(entity: ReceiptEntity) -> Self {
        Self {
            id: entity.id,
            receipt_code: entity.receipt_code.as_str().to_string(),
            type_name: entity.type_name,
            reference_id: entity.reference_id,
            source: entity.source,
            user_id: entity.user_id,
            branch_id: entity.branch_id,
            total_amount: BigDecimal::from_f64(entity.total_amount.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            payment_method: entity.payment_method,
            payment_ref: entity.payment_ref,
            issued_at: entity.issued_at,
            status: entity.status,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
