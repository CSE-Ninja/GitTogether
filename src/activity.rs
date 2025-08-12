use crate::{
    api::Contributor,
    api::ContributorExt,
    period::Period,
};

use anyhow::{Context, Result};

pub struct Activity {
    pub period: Period,
    pub contributors: Vec<Contributor>
}

impl Activity {
    pub async fn for_period(
        octocrab: &octocrab::Octocrab,
        owner: &str,
        repo: &str,
        period: Period,
    ) -> Result<Activity> {
        let contributors = octocrab
            .list_contributors_stats(owner, repo, &period.start, &period.end)
            .await
            .with_context(|| format!(
                "failed to fetch contributor stats for {owner}/{repo} in '{}' ({}..{})",
                period.name, period.start, period.end,
            ))?;
        Ok(Activity { period, contributors })
    }

    pub async fn for_periods(
        octocrab: &octocrab::Octocrab,
        owner: &str,
        repo: &str,
        periods: Vec<Period>,
    ) -> Result<Vec<Activity>> {
        let mut activities = Vec::new();
        for period in periods {
            activities.push(Self::for_period(octocrab, owner, repo, period).await?);
        }
        Ok(activities)
    }
}
