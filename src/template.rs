use crate::get_contributor_stats::ContributorStat;

macro_rules! gen_cell {
    ($($arg:ident),*) => {
        format!(r#"
<th>
    <table>
        <tr>
            <td style="text-align: center">
            <a href="https://github.com/{login}">
                <img src="{avatar}" alt="1" width=100px height=100px>
            </a>
            </td>
        </tr>
        <tr>
            <td style="text-align: center">
               <a href="https://github.com/{repo}/commits?author={login}">{login}</a>
            </td>
        </tr>
        <tr>
            <td style="text-align: center">
                <table border="0" cellspacing="0" cellpadding="0">
                    <tr>
                        <th width="50px" style="padding:1px">
                            $${{\small{{\color{{black}}\text{{{commit}}}}}}}$$
                        </th>
                        <th width="80px" style="padding:1px">
                            $${{\small{{\color{{green}}+\text{{{add}}}}}}}$$
                        </th>
                        <th width="80px" style="padding:1px">
                            $${{\small{{\color{{red}}-\text{{{del}}}}}}}$$
                        </th>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</th>
"#, $($arg=$arg),*)
    };
}

pub fn construct_table(repo: String, state: Vec<ContributorStat>) -> String {
    let mut builder = String::new();
//     builder.push_str(
//         r#"
// <style>
// #activity-table {
//     width: 50px;
//     text-align: center;
// }
// </style>"#,
//     );
    builder.push_str("<table>");
    let mut col_index = 0;
    let mut contributors = Vec::<(String, String, u32, u32, u32)>::new();

    for contributor in state {
        if contributor.author.login == "actions-user" {
            continue;
        }
        let mut commit = 0;
        let mut add = 0;
        let mut del = 0;
        for week in contributor.weeks {
            commit += week.c;
            add += week.a;
            del += week.d;
        }
        contributors.push((
            contributor.author.avatar_url.to_string(),
            contributor.author.login,
            commit,
            add,
            del,
        ));
    }

    contributors.sort_by(|a, b| {
        b.2.cmp(&a.2)
            .then_with(|| b.3.cmp(&a.3))
            .then_with(|| b.4.cmp(&a.4))
    });

    for (avatar, login, commit, add, del) in contributors {
        if col_index == 0 {
            builder.push_str("<tr>");
        }
        col_index += 1;
        let cell = gen_cell!(repo, avatar, login, commit, add, del);

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

    builder
}
