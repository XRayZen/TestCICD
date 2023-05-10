use async_trait::async_trait;
use crate::db::db_model::DbModel;

#[async_trait]
#[mockall::automock]
pub trait DbRepoTrait {
    fn new(collection_name: String) -> Self;
    async fn connect_aws(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn put_item(
        &self,
        item: &DbModel,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn query_item(
        &self,
        user_id: String,
    ) -> Result<DbModel, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn delete_item(
        &self,
        user_id: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn get_item(
        &self,
        user_id: String,
    ) -> Result<DbModel, Box<dyn std::error::Error + Send + Sync + 'static>>;
}



