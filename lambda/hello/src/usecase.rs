use crate::{db::{
    db_model::DbModel,
    db_repo::{},
}, repo_trait::RepoTrait, app::repo_trait::DbRepoTrait};

pub struct Usecase<'a, T: RepoTrait> {
    db_repo: &'a T::DbRepo,
}

impl<'a,T: RepoTrait> Usecase<'a,T> {
    pub fn new(repo:&'a T) -> Self {
        Self {
            db_repo: repo.db_repo(),
        }
    }
    pub async fn add_user(
        &self,
        user_id: &str,
        user_name: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let db_model = DbModel::builder()
            .user_id(user_id.to_string())
            .user_name(user_name.to_string())
            .build();
        self.db_repo.put_item(&db_model).await?;
        Ok(format!("add {} {}", user_id, user_name))
    }
    pub async fn get(
        &self,
        user_id: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let db_model = self.db_repo.get_item(user_id.to_string()).await?;
        Ok(format!("get {user_id} {}", db_model.user_name))
    }
    pub async fn find(
        &self,
        user_id: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let db_model = self.db_repo.query_item(user_id.to_string()).await?;
        Ok(format!("find {user_id} {}", db_model.user_name))
    }
}
