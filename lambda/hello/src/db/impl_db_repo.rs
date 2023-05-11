use crate::app::repo_trait::DbRepoTrait;

use super::db_model::DbModel;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Clone, Debug)]
pub struct ImplDbRepo {
    table_name: String,
}

#[async_trait]
impl DbRepoTrait for ImplDbRepo {
    fn new(collection_name: String) -> Self {
        Self {
            table_name: collection_name,
        }
    }

    async fn put_item(
        &self,
        item: &DbModel,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let config = aws_config::from_env().load().await;
        let db_client = aws_sdk_dynamodb::Client::new(&config);
        let _res = db_client
            .put_item()
            .table_name(&self.table_name)
            .item("user_id", AttributeValue::S(item.user_id.clone()))
            .item("user_name", AttributeValue::S(item.user_name.clone()))
            .item("user_age", AttributeValue::N(item.user_age.to_string()))
            .item("user_address", AttributeValue::S(item.user_address.clone()))
            .send()
            .await?;
        Ok(())
    }

    async fn query_item(
        &self,
        user_id: String,
    ) -> Result<DbModel, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let config = aws_config::from_env().load().await;
        let db_client = aws_sdk_dynamodb::Client::new(&config);
        let res = db_client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("user_id = :user_id")
            .expression_attribute_values(":user_id", AttributeValue::S(user_id.clone()))
            .send()
            .await?;
        let item = res.items.unwrap_or_default();
        let user_name = item
            .iter()
            .find_map(|x| {
                x.get("user_name")
                    .map(|v| v.as_s().unwrap_or(&"".to_string()).to_string())
            })
            .unwrap_or_default();
        let user_address = item
            .iter()
            .find_map(|x| {
                x.get("user_address")
                    .map(|v| v.as_s().unwrap_or(&"".to_string()).to_owned())
            })
            .unwrap_or_default();
        let user_age: u32 = item
            .iter()
            .find_map(|x| {
                x.get("user_age")
                    .map(|v| v.as_n().unwrap_or(&"".to_string()).parse().unwrap())
            })
            .unwrap_or_default();
        let db_model = DbModel::builder()
            .user_id(user_id)
            .user_name(user_name.to_string())
            .user_age(user_age)
            .user_address(user_address.to_string())
            .build();
        Ok(db_model)
    }

    async fn delete_item(
        &self,
        user_id: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let config = aws_config::from_env().load().await;
        let db_client = aws_sdk_dynamodb::Client::new(&config);
        let _res = db_client
            .delete_item()
            .table_name(&self.table_name)
            .key("user_id", AttributeValue::S(user_id.clone()))
            .send()
            .await?;
        Ok(())
    }

    async fn get_item(
        &self,
        user_id: String,
    ) -> Result<DbModel, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let config = aws_config::from_env().load().await;
        let db_client = aws_sdk_dynamodb::Client::new(&config);
        let res = db_client
            .get_item()
            .table_name(&self.table_name)
            .key("user_id", AttributeValue::S(user_id.clone()))
            .send()
            .await?;
        let item = res.item.unwrap_or_default();
        let user_name = item
            .get("user_name")
            .map(|v| v.as_s().unwrap_or(&"".to_string()).to_string())
            .unwrap_or_default();
        let user_address = item
            .get("user_address")
            .map(|v| v.as_s().unwrap_or(&"".to_string()).to_owned())
            .unwrap_or_default();
        let user_age: u32 = item
            .get("user_age")
            .map(|v| v.as_n().unwrap_or(&"".to_string()).parse().unwrap())
            .unwrap_or_default();
        let db_model = DbModel::builder()
            .user_id(user_id)
            .user_name(user_name.to_string())
            .user_age(user_age)
            .user_address(user_address.to_string())
            .build();
        Ok(db_model)
    }
}
