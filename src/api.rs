use std::collections::HashMap;

use crate::api::contributor_stats_query::Variables;

use chrono::{DateTime, Utc};
use graphql_client::GraphQLQuery;
use octocrab::{Octocrab, Result};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use identicon_rs::Identicon;

use self::contributor_stats_query::{
    ContributorStatsQueryRepositoryDefaultBranchRefTarget,
    ContributorStatsQueryRepositoryDefaultBranchRefTargetOnCommitHistoryEdgesNodeAuthor,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.docs.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
pub struct ContributorStatsQuery;
type GitTimestamp = String;
type CommitAuthor = ContributorStatsQueryRepositoryDefaultBranchRefTargetOnCommitHistoryEdgesNodeAuthor;


#[allow(clippy::upper_case_acronyms)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AccountType {
    Github,
    Unkown
}

impl Default for AccountType {
    fn default() -> Self { AccountType::Github }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Contributor {
    pub author: String,
    pub commit: CommitContribution,
    pub issue: IssueContribution,
    pub account_type: AccountType
}

impl Contributor {
    pub fn new(author: String, account_type: AccountType) -> Self {
        Self {
            author,
            account_type,
            ..Default::default()
        }
    }

    pub fn get_avatar_url(&self) -> String {
        avatar_for_user(&self.author)
    }

    pub async fn get_avatar_base64(&self) -> String {
        avatar_base64_for_user(&self.author, &self.account_type).await
    }
}

pub fn avatar_for_user(user: &str) -> String {
    format!("https://github.com/{}.png", user)
}

pub async fn avatar_base64_for_user(user: &str, account_type: &AccountType) -> String {
    match account_type {
        AccountType::Github => {
            let response = reqwest::get(avatar_for_user(user)).await.unwrap();
            let bytes = response.bytes().await.unwrap();
            let encoded = general_purpose::STANDARD.encode(&bytes);
            format!("data:image/png;base64,{}", encoded)
        }
        _ => {
            let icon = Identicon::new(user);
            let bytes = icon.export_png_data().unwrap();
            let encoded = general_purpose::STANDARD.encode(&bytes);
            format!("data:image/png;base64,{}", encoded)
        }
    }
}

pub struct ContributorStats {
    pub stats: HashMap<String, Contributor>,
}

impl ContributorStats {
    pub fn increase_addition(&mut self, user: &CommitAuthor, count: i64) {
        self.get_or_create_from_user_mut(user).commit.addition += count as u32;
    }

    pub fn increase_commit(&mut self, user: &CommitAuthor) {
        self.get_or_create_from_user_mut(user).commit.commit += 1;
    }

    pub fn increase_deletion(&mut self, user: &CommitAuthor, count: i64) {
        self.get_or_create_from_user_mut(user).commit.deletion += count as u32;
    }

    pub fn increase_issue(&mut self, user: &str) {
        self.get_or_create_mut(user).issue.issue += 1;
    }

    pub fn increase_pr(&mut self, user: &str) {
        self.get_or_create_mut(user).issue.pr += 1;
    }

    pub fn increase_comment(&mut self, user: &str) {
        self.get_or_create_mut(user).issue.comment += 1;
    }

    pub fn get_or_create_from_user_mut(&mut self, user: &CommitAuthor) -> &mut Contributor {
        if let Some(gh_user) = &user.user {
            self.get_or_create_mut(&gh_user.login)
        } else {
            let username = user.name.as_deref().unwrap();
            if !self.stats.contains_key(username) {
                self.stats
                    .insert(username.to_owned(), Contributor::new(username.to_string(), AccountType::Unkown));
            }
            self.stats.get_mut(username).unwrap()
        }
    }

    pub fn get_or_create_mut(&mut self, user: &str) -> &mut Contributor {
        if !self.stats.contains_key(user) {
            self.stats
                .insert(user.to_owned(), Contributor::new(user.to_string(), AccountType::Github));
        }
        self.stats.get_mut(user).unwrap()
    }
}

#[async_trait::async_trait]
pub trait ContributorExt {
    async fn list_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Vec<Contributor>>;
}

const IGNORED_ACCOUNTS: &[&str] = &[
    "actions-user",
    "coveralls",
    "dependabot",
    "dependabot-preview[bot]",
    "dependabot[bot]",
    "github-classroom[bot]",
];

pub fn response_to_contributor_stat(response: ContributorStatsResponse) -> Vec<Contributor> {
    let mut result = ContributorStats {
        stats: HashMap::new(),
    };

    for issue_opt in response.data.issues.edges.unwrap() {
        if let contributor_stats_query::ContributorStatsQueryIssuesEdgesNode::Issue(v) =
            issue_opt.unwrap().node.unwrap()
        {
            if let Some(author) = v.author {
                result.increase_issue(&author.login)
            }
            for comment_opt in v.comments.edges.unwrap() {
                if let Some(author) = comment_opt.unwrap().node.unwrap().author {
                    result.increase_comment(&author.login)
                }
            }
        }
    }

    for issue_opt in response.data.prs.edges.unwrap() {
        if let contributor_stats_query::ContributorStatsQueryPrsEdgesNode::PullRequest(v) =
            issue_opt.unwrap().node.unwrap()
        {
            if let Some(author) = v.author {
                result.increase_pr(&author.login)
            }
            for comment_opt in v.comments.edges.unwrap() {
                if let Some(author) = comment_opt.unwrap().node.unwrap().author {
                    result.increase_comment(&author.login)
                }
            }
            if let Some(review) = v.reviews {
                for comment_opt in review.edges.unwrap() {
                    if let Some(author) = comment_opt.unwrap().node.unwrap().author {
                        result.increase_comment(&author.login)
                    }
                }
            }
        }
    }

    if let Some(ContributorStatsQueryRepositoryDefaultBranchRefTarget::Commit(c)) = response
        .data
        .repository
        .unwrap()
        .default_branch_ref
        .unwrap()
        .target
    {
        let histories = c.history.edges.unwrap();
        for history in histories {
            if let Some(commit) = history.unwrap().node {
                if commit.parents.total_count > 1 {
                    // This is a merge commit, let's skip it.
                    continue;
                }
                if let Some(author) = commit.author {
                    result.increase_addition(&author, commit.additions);
                    result.increase_deletion(&author, commit.deletions);
                    result.increase_commit(&author);
                }
            }
        }
    }

    for ele in IGNORED_ACCOUNTS {
        result.stats.remove(*ele);
    }

    let mut stats = result.stats.values().cloned().collect::<Vec<_>>();
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
        owner: &str,
        repo: &str,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Vec<Contributor>> {
        let start_s = start.to_rfc3339();
        let end_s = end.to_rfc3339();

        let s1 = format!("repo:{owner}/{repo} type:issue created:{start_s}..{end_s}");
        let s2 = format!("repo:{owner}/{repo} type:pr created:{start_s}..{end_s}");
        let variables = Variables {
            owner: owner.to_string(),
            repo: repo.to_string(),
            start: start_s,
            end: end_s,
            s1,
            s2,
        };
        let query = ContributorStatsQuery::build_query(variables);
        let resp: ContributorStatsResponse = self.graphql(&query).await?;
        Ok(response_to_contributor_stat(resp))
    }
}
