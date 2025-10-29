use async_trait::async_trait;

use crate::domain::entities::payment::PaymentEntity;

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<PaymentEntity>, anyhow::Error>;
    async fn find_by_order(&self, order_id: i32) -> Result<Vec<PaymentEntity>, anyhow::Error>;
    async fn save(&self, payment: &PaymentEntity) -> Result<(), anyhow::Error>;
}
