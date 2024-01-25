use std::{collections::HashMap, fmt::format};

use crate::api::contributor_stats_query::Variables;
use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery, Response};
use octocrab::{
    auth, models::{self, Collaborator}, repos::RepoHandler, Octocrab, Result
};
use serde::{Deserialize, Serialize};

use self::contributor_stats_query::ContributorStatsQueryRepositoryDefaultBranchRefTarget;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.docs.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]

pub struct ContributorStatsQuery;
type GitTimestamp = String;
type URI = String;

#[derive(Debug, Deserialize)]
pub struct ContributorStatsResponse {
    data: contributor_stats_query::ResponseData,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitContribution {
    pub addition: u32,
    pub deletion: u32,
    pub commit: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueContribution {
    pub pr: u32,
    pub issue: u32,
    pub comment: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Contributor {
    pub author: String,
    pub commit: CommitContribution,
    pub issue: IssueContribution,
}

impl Contributor {
    pub fn new(author: String) -> Self {
        Self {
            author,
            ..Default::default()
        }
    }

    pub fn get_avatar_url(&self) -> String {
        format!("https://github.com/{}.png", self.author)
    }
}

pub struct ContributorStats {
    pub stats: HashMap<String, Contributor>,
}

impl ContributorStats {
    pub fn increase_addition(&mut self, user: &String, count: i64) {
        self.get_or_create_mut(user).commit.addition += count as u32;
    }

    pub fn increase_commit(&mut self, user: &String) {
        self.get_or_create_mut(user).commit.commit += 1;
    }

    pub fn increase_deletion(&mut self, user: &String, count: i64) {
        self.get_or_create_mut(user).commit.deletion += count as u32;
    }

    pub fn increase_issue(&mut self, user: &String) {
        self.get_or_create_mut(user).issue.issue += 1;
    }

    pub fn increase_pr(&mut self, user: &String) {
        self.get_or_create_mut(user).issue.pr += 1;
    }

    pub fn increase_comment(&mut self, user: &String) {
        self.get_or_create_mut(user).issue.comment += 1;
    }

    pub fn get_or_create_mut(&mut self, user: &String) -> &mut Contributor {
        if !self.stats.contains_key(user) {
            self.stats.insert(user.clone(), Contributor::new(user.to_string()));
        }
        self.stats.get_mut(user).unwrap()
    }
}

#[async_trait::async_trait]
pub trait ContributorExt {
    // async fn list_contributors_stats(&self, repository: String) -> Result<Vec<ContributorStat>>;
    async fn list_contributors_stats(
        &self,
        owner: &String,
        repo: &String,
        start: &String,
        end: &String,
    ) -> Result<Vec<Contributor>>;
}

const IGNORED_ACCOUNTS: &'static [&str] = &["actions-user", "github-classroom[bot]"];

pub fn response_to_contributor_stat(response: ContributorStatsResponse) -> Vec<Contributor>{
    let mut result = ContributorStats {
        stats: HashMap::new(),
    };

    for issue_opt in response.data.issues.edges.unwrap() {
        match issue_opt.unwrap().node.unwrap() {
            contributor_stats_query::ContributorStatsQueryIssuesEdgesNode::Issue(v) => {
                if let Some(author) = v.author {
                    result.increase_issue(&author.login)
                }
                for comment_opt in v.comments.edges.unwrap() {
                    if let Some(author) = comment_opt.unwrap().node.unwrap().author {
                        result.increase_comment(&author.login)
                    }
                }
            }
            _ => {}
        }
    }

    for issue_opt in response.data.prs.edges.unwrap() {
        match issue_opt.unwrap().node.unwrap() {
            contributor_stats_query::ContributorStatsQueryPrsEdgesNode::PullRequest(v) => {
                if let Some(author) = v.author {
                    result.increase_pr(&author.login)
                }
                for comment_opt in v.comments.edges.unwrap() {
                    if let Some(author) = comment_opt.unwrap().node.unwrap().author {
                        result.increase_comment(&author.login)
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(target) = response
        .data
        .repository
        .unwrap()
        .default_branch_ref
        .unwrap()
        .target
    {
        match target {
            ContributorStatsQueryRepositoryDefaultBranchRefTarget::Commit(c) => {
                let histories = c.history.edges.unwrap();
                for history in histories {
                    if let Some(commit) = history.unwrap().node {
                        if let Some(author) = commit.author.unwrap().user  {
                            result.increase_addition(&author.login, commit.additions);
                            result.increase_deletion(&author.login, commit.deletions);
                            result.increase_commit(&author.login);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    for ele in IGNORED_ACCOUNTS {
        result.stats.remove(*ele);
    }


    let mut stats = result
        .stats
        .values()
        .cloned()
        .collect::<Vec<_>>();
    stats.sort_by(|a, b| {
        b.commit
            .commit
            .cmp(&a.commit.commit)
            .then_with(|| b.commit.addition.cmp(&a.commit.addition))
            .then_with(|| b.commit.deletion.cmp(&a.commit.deletion))
    });
    stats
}

#[async_trait::async_trait]
impl ContributorExt for Octocrab {
    async fn list_contributors_stats(
        &self,
        owner: &String,
        repo: &String,
        start: &String,
        end: &String,
    ) -> Result<Vec<Contributor>> {
        let s1 = format!("repo:{owner}/{repo} type:issue created:{start}..{end}");
        let s2 = format!("repo:{owner}/{repo} type:pr created:{start}..{end}");
        let variables = Variables {
            owner: owner.to_string(),
            repo: repo.to_string(),
            start: start.to_string(),
            end: end.to_string(),
            s1,
            s2,
        };
        let query = ContributorStatsQuery::build_query(variables);
        let resp: ContributorStatsResponse = self.graphql(&query).await?;
        Ok(response_to_contributor_stat(resp))
    }
}
