use crate::db::db_repo::{DbRepoTrait, ImplDbRepo};



// リポジトリをまとめるトレイト
pub trait RepoTrait {
    type DbRepo: DbRepoTrait;

    fn db_repo(&self) -> &Self::DbRepo;
}
// 依存するリポジトリをまとめるDI構造体
//リポジトリtraitの具体型を決定する、静的なDI (Dependency Injection) をする
pub struct ImplRepos {
    db_repo: ImplDbRepo,
}

impl RepoTrait for ImplRepos {
    type DbRepo = ImplDbRepo;

    fn db_repo(&self) -> &Self::DbRepo {
        &self.db_repo
    }
}

impl ImplRepos {
    pub fn new(db_repo: ImplDbRepo /*More Impl Repo...*/) -> Self {
        Self { db_repo }
    }
}
