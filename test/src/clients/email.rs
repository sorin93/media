use crate::error::Result;

#[derive(Clone)]
pub struct Client {}

impl Client {
    pub fn new() -> Self { Self {} }
    
    pub async fn send_verify_code(&self, email: &str, code: &str) -> Result<()> {
        tracing::info!("📧 EMAIL to {}: Code is {}", email, code);
        Ok(())
    }
}