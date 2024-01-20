use octocrab::{Octocrab, Result, models, repos::RepoHandler};
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Contribution {
    pub w: u32,
    pub a: u32,
    pub d: u32,
    pub c: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContributorStat {
    pub author: models::Author,
    pub total: u32,
    pub weeks: Vec<Contribution>
}

#[async_trait::async_trait]
pub trait ContributorExt {
    async fn list_contributors_stats(&self, repository: String) -> Result<Vec<ContributorStat>>;
}

#[async_trait::async_trait]
impl ContributorExt for Octocrab {
    async fn list_contributors_stats(&self, repository: String) -> Result<Vec<ContributorStat>> {
        let route = format!("/repos/{repository}/stats/contributors");
        self.get(route, None::<&()>).await
    }
}