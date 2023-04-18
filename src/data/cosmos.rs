use std::sync::Arc;
use azure_data_cosmos::clients::CosmosClient;
use coi::Inject;
use crate::data::entities::Amendment;

trait ICosmos: Inject {
    fn get(&self) -> Amendment;
}

#[derive(Inject)]
//#[coi(provides dyn ICosmos with Cosmos::new(appconfig))]
struct Cosmos {
    cosmos_client: CosmosClient,
}

impl Cosmos {

}