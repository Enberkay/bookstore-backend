use std::sync::Arc;
use anyhow::Result;

use crate::application::dtos::branch_dto::{
    BranchResponse,
    CreateBranchRequest,
    UpdateBranchRequest,
};
use crate::domain::{
    entities::branch::BranchEntity,
    repositories::branch_repository::BranchRepository,
};

/// BranchService — application-level orchestration for Branch entity
pub struct BranchService {
    branch_repo: Arc<dyn BranchRepository>,
}

impl BranchService {
    pub fn new(branch_repo: Arc<dyn BranchRepository>) -> Self {
        Self { branch_repo }
    }

    /// Create a new branch
    pub async fn create_branch(&self, req: CreateBranchRequest) -> Result<BranchResponse> {
        let branch = BranchEntity::new(
            req.name,
            req.address,
            req.phone,
        )?;

        self.branch_repo.save(&branch).await?;
        Ok(BranchResponse::from(branch))
    }

    /// Get a branch by id
    pub async fn get_branch_by_id(&self, id: i32) -> Result<Option<BranchResponse>> {
        let branch_opt = self.branch_repo.find_by_id(id).await?;
        Ok(branch_opt.map(BranchResponse::from))
    }

    /// Get all branches
    pub async fn get_all_branches(&self) -> Result<Vec<BranchResponse>> {
        let branches = self.branch_repo.find_all().await?;
        Ok(branches.into_iter().map(BranchResponse::from).collect())
    }

    /// Update branch info
    pub async fn update_branch(&self, id: i32, req: UpdateBranchRequest) -> Result<()> {
        let mut branch = match self.branch_repo.find_by_id(id).await? {
            Some(b) => b,
            None => anyhow::bail!("Branch not found"),
        };

        if let Some(name) = req.name {
            branch.name = name;
        }
        if let Some(address) = req.address {
            branch.address = Some(address);
        }
        if let Some(phone) = req.phone {
            branch.phone = Some(phone);
        }

        self.branch_repo.save(&branch).await
    }

    /// Delete branch
    pub async fn delete_branch(&self, id: i32) -> Result<()> {
        self.branch_repo.delete(id).await
    }
}
