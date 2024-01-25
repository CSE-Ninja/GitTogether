use svg::{node::{element::ForeignObject, Text}, Document};

use crate::{api::{
    contributor_stats_query::ContributorStatsQueryRepositoryDefaultBranchRefTargetOnCommit, Contributor, ContributorStats
}, card::draw_card, period::Period};

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
                                <img src="{avatar}" alt="1" width=100px height=100px>
                        </th>
                    </tr>
                    <tr>
                        <th>
                            <a href="https://github.com/{login}">
                            {login}
                            </a>
                        </th>
                    </tr>
                </table>
            </th>
            <th>
                <table>
                    <tr>
                        <th align="left">
                            <a href="https://github.com/{repo}/commits?author={login}&since={start}&until={end}">
                            Commit: {commit}
                            </a>
                        </th>
                    </tr>
                    <tr>
                        <th align="left">
                            Addition: {add}
                        </th>
                    </tr>
                    <tr>
                        <th align="left">
                            Deletion: {del}
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            <a href="https://github.com/{repo}/issues?q=author%3A{login}+type%3Aissue+created%3A{start}..{end}">
                            Issues: {issue}
                            </a>
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            <a href="https://github.com/{repo}/pulls?q=author%3A{login}+type%3Apr+created%3A{start}..{end}">
                            PRs: {pr}
                            </a>
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

pub fn construct_table(repo: &String, stats: &Vec<Contributor>, period: &Period) -> String {
    let mut builder = String::new();
    // builder.push_str("<details>
    // <summary>Click to expand!</summary>\n");
    builder.push_str("<table>");
    let mut col_index = 0;

    for contributor in stats {
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
            comment = contributor.issue.comment,
            start = period.start,
            end = period.end
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


    // svg::save("image.svg", &draw_card(stats)).unwrap();
    // builder.push_str("</details>\n\n");

    builder
}
