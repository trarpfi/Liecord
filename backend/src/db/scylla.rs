use scylla::{Session, SessionBuilder};
use crate::config::Config;

pub struct ScyllaDB {
    pub session: Session,
    pub keyspace: String,
}

impl ScyllaDB {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let session = SessionBuilder::new()
            .known_nodes(&config.scylla_nodes)
            .build()
            .await?;

        // Create keyspace if it doesn't exist
        session
            .query(
                format!(
                    "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class': 'NetworkTopologyStrategy', 'datacenter1': {}}}",
                    config.scylla_keyspace, config.scylla_replication_factor
                ),
                &[],
            )
            .await?;

        // Use keyspace
        session
            .use_keyspace(&config.scylla_keyspace, false)
            .await?;

        Ok(Self {
            session,
            keyspace: config.scylla_keyspace.clone(),
        })
    }
}
