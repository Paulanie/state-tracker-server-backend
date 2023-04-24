use std::sync::Arc;
use azure_data_cosmos::clients::{CosmosClient, DatabaseClient, CollectionClient};
use azure_data_cosmos::prelude::{AuthorizationToken, Query, QueryDocuments};
use crate::configuration::ApplicationConfig;
use crate::data::entities::Amendment;
use crate::configuration::APPCONFIG;

pub struct Cosmos {
    cosmos_client: CosmosClient,
    database_client: DatabaseClient,
    collection_client: CollectionClient,
}

impl Cosmos {
    pub fn new(account: String, key: String, database: String, collection: String) -> Self {
        println!("Initializing Cosmos...");
        let authorization_token = AuthorizationToken::primary_from_base64(&key).expect("Unable to create authorization token for CosmosDb.");
        let cosmos_client = CosmosClient::new(account, authorization_token);
        let database_client = cosmos_client.database_client(database);
        let collection_client = database_client.collection_client(collection);
        Self { cosmos_client, database_client, collection_client }
    }

    pub fn get(&self, limit: Option<i32>, offset: Option<i32>) -> QueryDocuments<Amendment> {
        let query = format!("SELECT * FROM amendments OFFSET {} LIMIT {}", offset.unwrap_or(0), limit.unwrap_or(50));
        self.collection_client.query_documents(Query::new(query)).into_stream()
    }
}