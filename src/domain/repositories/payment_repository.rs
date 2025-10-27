use crate::domain::entities::payment::PaymentEntity;

pub trait PaymentRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<PaymentEntity>, anyhow::Error>;
    fn find_by_order(&self, order_id: i32) -> Result<Vec<PaymentEntity>, anyhow::Error>;
    fn save(&self, payment: &PaymentEntity) -> Result<(), anyhow::Error>;
}
