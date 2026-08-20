pub mod scylla;
pub mod redis;
pub mod elasticsearch;

use crate::config::Config;

pub struct Database {
    pub scylla: scylla::ScyllaDB,
    pub redis: redis::RedisClient,
    pub elasticsearch: elasticsearch::ElasticsearchClient,
}

impl Database {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        tracing::info!("Initializing database connections...");
        
        let scylla = scylla::ScyllaDB::new(config).await?;
        tracing::info!("✓ ScyllaDB connected");
        
        let redis = redis::RedisClient::new(config).await?;
        tracing::info!("✓ Redis connected");
        
        let elasticsearch = elasticsearch::ElasticsearchClient::new(config).await?;
        tracing::info!("✓ Elasticsearch connected");
        
        Ok(Self {
            scylla,
            redis,
            elasticsearch,
        })
    }
}
