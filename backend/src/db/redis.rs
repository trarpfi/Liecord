use redis::{aio::ConnectionManager, Client};
use crate::config::Config;

pub struct RedisClient {
    pub connection: ConnectionManager,
}

impl RedisClient {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let client = Client::open(config.redis_url.as_str())?;
        let connection = ConnectionManager::new(client).await?;
        
        Ok(Self { connection })
    }
    
    pub async fn set_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry_seconds: usize,
    ) -> anyhow::Result<()> {
        let mut conn = self.connection.clone();
        redis::cmd("SETEX")
            .arg(key)
            .arg(expiry_seconds)
            .arg(value)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
    
    pub async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.connection.clone();
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        Ok(value)
    }
    
    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.connection.clone();
        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
    
    pub async fn set_user_online(&self, user_id: &str) -> anyhow::Result<()> {
        let key = format!("user:online:{}", user_id);
        self.set_with_expiry(&key, "1", 300).await // 5 minute TTL
    }
    
    pub async fn is_user_online(&self, user_id: &str) -> anyhow::Result<bool> {
        let key = format!("user:online:{}", user_id);
        Ok(self.get(&key).await?.is_some())
    }
}
