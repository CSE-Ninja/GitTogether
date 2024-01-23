use crate::api::{
    contributor_stats_query::ContributorStatsQueryRepositoryDefaultBranchRefTargetOnCommit,
    ContributorStats,
};

macro_rules! gen_cell {
    ($($arg:ident=$exp:expr),*) => {
        format!(r#"
<th>
    <table>
        <tr>
            <th>
                <table>
                    <tr>
                        <th>
                            <a href="https://github.com/{login}">
                                <img src="{avatar}" alt="1" width=100px height=100px>
                            </a>
                        </th>
                    <tr>
                    <tr>
                        <th>
                            <a href="https://github.com/{repo}/commits?author={login}">{login}</a>
                        </th>
                    <tr>
                </table>
            </th>
            <th>
                <table>
                    <tr>
                        <th align="left">
                            Commit: {commit}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Addition: {add}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Deletion: {del}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Issues: {issue}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            PRs: {pr}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Comments: {comment}
                        </th>
                    <tr>
                </table>
            </th>
        <tr>
    </table>
</th>
"#, $($arg=$exp),*)
    };
}

const IGNORED_ACCOUNTS: &'static [&str] = &["actions-user", "github-classroom[bot]"];

pub fn construct_table(repo: &String, stats: &ContributorStats) -> String {
    let mut builder = String::new();
    builder.push_str("<details>
    <summary>Click to expand!</summary>\n");
    builder.push_str("<table>");
    let mut col_index = 0;

    let mut results = stats
        .stats
        .values()
        .filter(|it| !IGNORED_ACCOUNTS.contains(&it.author.as_str()))
        .collect::<Vec<_>>();
    results.sort_by(|a, b| {
        b.commit
            .commit
            .cmp(&a.commit.commit)
            .then_with(|| b.commit.addition.cmp(&a.commit.addition))
            .then_with(|| b.commit.deletion.cmp(&a.commit.deletion))
    });

    for contributor in results {
        if col_index == 0 {
            builder.push_str("<tr>");
        }
        col_index += 1;
        let cell = gen_cell!(
            repo = repo,
            avatar = contributor.get_avatar_url(),
            login = contributor.author,
            commit = contributor.commit.commit,
            add = contributor.commit.addition,
            del = contributor.commit.deletion,
            issue = contributor.issue.issue,
            pr = contributor.issue.pr,
            comment = contributor.issue.comment
        );

        builder.push_str(cell.as_str());
        if col_index == 3 {
            builder.push_str("</tr>");
            col_index = 0;
        }
    }
    if col_index != 0 {
        builder.push_str("</tr>");
    }
    builder.push_str("</table>");
    builder.push_str("</details>\n\n");

    builder
}
