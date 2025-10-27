use crate::domain::entities::receipt::ReceiptEntity;
use crate::domain::value_objects::receipt_code::ReceiptCode;

pub trait ReceiptRepository: Send + Sync {
    fn find_by_code(&self, code: &ReceiptCode) -> Result<Option<ReceiptEntity>, anyhow::Error>;
    fn find_by_reference(&self, reference_id: i32) -> Result<Vec<ReceiptEntity>, anyhow::Error>;
    fn save(&self, receipt: &ReceiptEntity) -> Result<(), anyhow::Error>;
}
