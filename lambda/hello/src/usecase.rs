use crate::db::{db_model::DbModel, db_repo::DbRepoTrait};

// リポジトリをまとめるトレイト
pub trait RepoTrait {
    type DbRepo: DbRepoTrait;

    fn db_repo(&self) -> Self::DbRepo;
}

pub trait UsecaseTrait {
    fn open() -> String;
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
    pub fn open(&self) -> String {
       todo!()
    }
}
