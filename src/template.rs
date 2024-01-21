use crate::get_contributor_stats::ContributorStat;

macro_rules! gen_cell {
    ($($arg:tt)*) => {
        format!(r#"
<th>
    <table>
        <tr>
            <td style="text-align: center">
                <img src="{}" alt="1" width=100px height=100px>
            </td>
        </tr>
        <tr>
            <td style="text-align: center">
               {}
            </td>
        </tr>
        <tr>
            <td style="text-align: center">
                <table>
                    <tr>
                        <th id="activity-table">
                            $${{\color{{black}}+\text{{{}}}}}$$
                        </th>
                        <th id="activity-table">
                            $${{\color{{green}}+\text{{{}}}}}$$
                        </th>
                        <th id="activity-table">
                            $${{\color{{red}}-\text{{{}}}}}$$
                        </th>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</th>
"#, $($arg)*)
    };
}

pub fn construct_table(state: Vec<ContributorStat>) -> String {
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
        let cell = gen_cell!(avatar, login, commit, add, del);

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
