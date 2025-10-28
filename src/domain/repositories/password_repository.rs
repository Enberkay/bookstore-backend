use anyhow::Result;

#[async_trait::async_trait]
pub trait PasswordRepository: Send + Sync {
    // Hash a plain text password
    async fn hash(&self, password: &str) -> Result<String>;
    // Verify a plain text password against a hash
    async fn verify(&self, password: &str, hash: &str) -> Result<bool>;
}
