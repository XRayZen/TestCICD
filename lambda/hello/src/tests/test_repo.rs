use crate::{db::db_repo::MockDbRepoTrait, repo_trait::RepoTrait, };

struct TestRepos {
    pub test_repo: MockDbRepoTrait,
}

impl RepoTrait for TestRepos {
    type DbRepo = MockDbRepoTrait;

    fn db_repo(&self) -> &Self::DbRepo {
        &self.test_repo
    }
}

impl TestRepos {
    pub fn new(test_repo: MockDbRepoTrait) -> Self {
        Self { test_repo }
    }
}
