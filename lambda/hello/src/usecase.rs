use crate::db::{
    db_model::DbModel,
    db_repo::{DbRepoTrait, ImplDbRepo},
};

// リポジトリをまとめるトレイト
pub trait RepoTrait {
    type DbRepo: DbRepoTrait;

    fn db_repo(&self) -> Self::DbRepo;
}
// 依存するリポジトリをまとめるDI構造体
//リポジトリtraitの具体型を決定する、静的なDI (Dependency Injection) をする
pub struct ImplRepos {
    db_repo: ImplDbRepo,
}

impl RepoTrait for ImplRepos {
    type DbRepo = ImplDbRepo;

    fn db_repo(&self) -> Self::DbRepo {
        self.db_repo.clone()
    }
}

impl ImplRepos {
    pub fn new(db_repo: ImplDbRepo /*More Impl Repo...*/) -> Self {
        Self { db_repo }
    }
}

pub struct Usecase<T: RepoTrait> {
    db_repo: T::DbRepo,
}

impl<T: RepoTrait> Usecase<T> {
    pub fn new(repo: T) -> Self {
        Self {
            db_repo: repo.db_repo(),
        }
    }
    pub async fn add(
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
}
