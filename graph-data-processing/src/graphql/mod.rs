/// Collection of graphql entities
mod entities;

use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder,
};
use entities::{DataCollection, DataProcessing};
use models::data_collection_file_attachment;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// The GraphQL schema exposed by the service
pub type RootSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// A schema builder for the service
pub fn root_schema_builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).enable_federation()
}

/// The root query of the service
#[derive(Debug, Clone, Default)]
pub struct Query;

#[ComplexObject]
impl DataCollection {
    /// Fetched all the processed data from data collection during a session
    async fn processed_data(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DataProcessing>, async_graphql::Error> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(data_collection_file_attachment::Entity::find()
            .filter(data_collection_file_attachment::Column::DataCollectionId.eq(self.id))
            .all(database)
            .await?
            .into_iter()
            .map(DataProcessing::from)
            .collect())
    }
}

#[Object]
impl Query {
    /// Reference datasets resolver for the router
    #[graphql(entity)]
    async fn router_data_collection(&self, id: u32) -> DataCollection {
        DataCollection { id }
    }
}