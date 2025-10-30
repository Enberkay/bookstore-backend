use serde::{Serialize, Deserialize};
use crate::domain::entities::branch::BranchEntity;

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl From<BranchEntity> for BranchResponse {
    fn from(entity: BranchEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            address: entity.address,
            phone: entity.phone,
        }
    }
}
