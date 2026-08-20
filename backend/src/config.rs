use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub environment: String,
    
    // ScyllaDB
    pub scylla_nodes: Vec<String>,
    pub scylla_keyspace: String,
    pub scylla_replication_factor: usize,
    
    // Redis
    pub redis_url: String,
    pub redis_pool_size: usize,
    
    // Elasticsearch
    pub elasticsearch_url: String,
    pub elasticsearch_index_prefix: String,
    
    // JWT
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    
    // Upload
    pub upload_dir: String,
    pub max_upload_size: usize,
    
    // CORS
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            
            scylla_nodes: env::var("SCYLLA_NODES")
                .unwrap_or_else(|_| "127.0.0.1:9042".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            scylla_keyspace: env::var("SCYLLA_KEYSPACE")
                .unwrap_or_else(|_| "liecord".to_string()),
            scylla_replication_factor: env::var("SCYLLA_REPLICATION_FACTOR")
                .unwrap_or_else(|_| "3".to_string())
                .parse()?,
            
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            redis_pool_size: env::var("REDIS_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            
            elasticsearch_url: env::var("ELASTICSEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:9200".to_string()),
            elasticsearch_index_prefix: env::var("ELASTICSEARCH_INDEX_PREFIX")
                .unwrap_or_else(|_| "liecord".to_string()),
            
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()?,
            
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".to_string()),
            max_upload_size: env::var("MAX_UPLOAD_SIZE")
                .unwrap_or_else(|_| "52428800".to_string())
                .parse()?,
            
            cors_origin: env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }
}
