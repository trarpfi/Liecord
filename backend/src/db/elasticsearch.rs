use elasticsearch::{
    Elasticsearch, 
    http::transport::Transport,
    IndexParts, SearchParts,
};
use serde_json::{json, Value};
use crate::config::Config;

pub struct ElasticsearchClient {
    pub client: Elasticsearch,
    pub index_prefix: String,
}

impl ElasticsearchClient {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let transport = Transport::single_node(&config.elasticsearch_url)?;
        let client = Elasticsearch::new(transport);
        
        Ok(Self {
            client,
            index_prefix: config.elasticsearch_index_prefix.clone(),
        })
    }
    
    pub async fn index_message(&self, message_id: &str, message: Value) -> anyhow::Result<()> {
        let index_name = format!("{}_messages", self.index_prefix);
        
        self.client
            .index(IndexParts::IndexId(&index_name, message_id))
            .body(message)
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn search_messages(
        &self,
        query: &str,
        channel_id: Option<&str>,
        limit: i64,
    ) -> anyhow::Result<Vec<Value>> {
        let index_name = format!("{}_messages", self.index_prefix);
        
        let mut search_query = json!({
            "query": {
                "bool": {
                    "must": [
                        {
                            "match": {
                                "content": query
                            }
                        }
                    ]
                }
            },
            "size": limit,
            "sort": [
                { "created_at": { "order": "desc" } }
            ]
        });
        
        if let Some(channel_id) = channel_id {
            if let Some(must) = search_query["query"]["bool"]["must"].as_array_mut() {
                must.push(json!({
                    "term": { "channel_id": channel_id }
                }));
            }
        }
        
        let response = self.client
            .search(SearchParts::Index(&[&index_name]))
            .body(search_query)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let hits = json["hits"]["hits"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|hit| hit["_source"].clone().as_object().cloned())
                    .map(|obj| Value::Object(obj))
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(hits)
    }
}
