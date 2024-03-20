use std::collections::{HashSet};

use svg::{
    node::{
        self,
        element::{self, Definitions, Group, Image, Path, Rectangle, Style, SVG},
    },
    parser::Event,
    Document,
};

use crate::{
    api::{avatar_base64_for_user, AccountType, Contributor},
    period::Period,
    styles,
};

// (TODO) This should be optimized in the future.

pub fn draw(source: &str) -> SVG {
    let mut parser = svg::parser::Parser::new(source);
    let mut path = Path::new();
    let result = parser.next().unwrap();
    if let Event::Tag(_path, _, attributes) = result {
        *path.get_attributes_mut() = attributes;
    }
    SVG::new().add(path).set("class", "icon")
}

const CSS: &str = r#"
.header {
    font: 600 18px 'Segoe UI', Ubuntu, Sans-Serif;
    fill: #007A00;
    animation: fadeInAnimation 0.8s ease-in-out forwards;
}
.stat {
    font: 600 14px 'Segoe UI', Ubuntu, "Helvetica Neue", Sans-Serif; fill: #444;
}
.bold { font-weight: 700 }
.icon {
    fill: #003D00;
    display: block;
}
"#;

pub fn draw_plus() -> SVG {
    draw(r#"<path fill-rule="evenodd" d="M24 10h-10v-10h-4v10h-10v4h10v10h4v-10h10z"/>"#)
        .set("width", "15")
        .set("height", "15")
        .set("viewBox", "0 0 25 25")
}

pub fn draw_minus() -> SVG {
    draw(r#"<path fill-rule="evenodd" d="M0 10h24v4h-24z"/>"#)
        .set("width", "15")
        .set("height", "15")
        .set("viewBox", "0 0 25 25")
}

pub fn draw_pr() -> SVG {
    draw(
        r#"<path fill-rule="evenodd" d="M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z"/>"#,
    )
}

pub fn draw_commit() -> SVG {
    draw(
        r#"<path fill-rule="evenodd" d="M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z"/>"#,
    )
}

pub fn draw_issue() -> SVG {
    draw(
        r#"<path fill-rule="evenodd" d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zm-.25-6.25a.75.75 0 00-1.5 0v3.5a.75.75 0 001.5 0v-3.5z"/>"#,
    )
}

pub fn draw_discussion() -> SVG {
    draw(
        r#"<path fill-rule="evenodd" d="M1.75 1h8.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 10.25 10H7.061l-2.574 2.573A1.458 1.458 0 0 1 2 11.543V10h-.25A1.75 1.75 0 0 1 0 8.25v-5.5C0 1.784.784 1 1.75 1ZM1.5 2.75v5.5c0 .138.112.25.25.25h1a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h3.5a.25.25 0 0 0 .25-.25v-5.5a.25.25 0 0 0-.25-.25h-8.5a.25.25 0 0 0-.25.25Zm13 2a.25.25 0 0 0-.25-.25h-.5a.75.75 0 0 1 0-1.5h.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 14.25 12H14v1.543a1.458 1.458 0 0 1-2.487 1.03L9.22 12.28a.749.749 0 0 1 .326-1.275.749.749 0 0 1 .734.215l2.22 2.22v-2.19a.75.75 0 0 1 .75-.75h1a.25.25 0 0 0 .25-.25Z" />"#,
    )
}

pub struct CardDrawer<'a, S: styles::Style + ?Sized> {
    contributor: &'a Contributor,
    start: &'a str,
    end: &'a str,
    repo: &'a str,
    style: &'a S,
}

impl<'a, S: styles::Style + ?Sized> CardDrawer<'a, S> {
    pub fn create_detail(&self) -> Group {
        let y_space = 20;
        let detail = Group::new()
            .set("transform", "translate(140, 25)")
            .add(
                self.style
                    .draw_contribution_item(
                        draw_commit(),
                        "Commit",
                        self.contributor.commit.commit,
                        format!(
                            "https://github.com/{}/commits?author={}&amp;since={}&amp;until={}",
                            self.repo, self.contributor.author, self.start, self.end
                        )
                        .as_str(),
                    )
                    .set("transform", format!("translate(0, {})", 0)),
            )
            .add(
                self.style
                    .draw_contribution_item(
                        draw_plus(),
                        "Addition",
                        self.contributor.commit.addition,
                        "",
                    )
                    .set("transform", format!("translate(0, {})", y_space)),
            )
            .add(
                self.style
                    .draw_contribution_item(
                        draw_minus(),
                        "Deletion",
                        self.contributor.commit.deletion,
                        "",
                    )
                    .set("transform", format!("translate(0, {})", 2 * y_space)),
            )
            .add(
                self.style
                    .draw_contribution_item(
                        draw_issue(),
                        "Issue",
                        self.contributor.issue.issue,
                        format!(
                    "https://github.com/{}/issues?q=author%3A{}+type%3Aissue+created%3A{}..{}",
                    self.repo, self.contributor.author, self.start, self.end
                )
                        .as_str(),
                    )
                    .set("transform", format!("translate(0, {})", 3 * y_space)),
            )
            .add(
                self.style
                    .draw_contribution_item(
                        draw_pr(),
                        "PR",
                        self.contributor.issue.pr,
                        format!(
                            "https://github.com/{}/pulls?q=author%3A{}+type%3Apr+created%3A{}..{}",
                            self.repo, self.contributor.author, self.start, self.end
                        )
                        .as_str(),
                    )
                    .set("transform", format!("translate(0, {})", 4 * y_space)),
            )
            .add(
                self.style
                    .draw_contribution_item(
                        draw_discussion(),
                        "Discussion",
                        self.contributor.issue.comment,
                        "",
                    )
                    .set("transform", format!("translate(0, {})", 5 * y_space)),
            );
        detail
    }

    pub async fn contributor_info(&self) -> Group {
        let title = self.style.draw_title(
            &self.contributor.author,
            format!("#{}", &self.contributor.author).as_str(),
        );
        let detail = self.create_detail();
        Group::new().add(title).add(detail)
    }
}

pub async fn draw_period(
    stat: &Vec<Contributor>,
    start: &str,
    end: &str,
    repo: &str,
    style: &dyn styles::Style,
) -> (Group, u32) {
    let mut doc = Group::new();
    let mut offset = 0;

    for contributor in stat {
        let drawer = CardDrawer {
            contributor,
            start,
            end,
            repo,
            style,
        };
        let x_offset = offset % style.user_per_row();
        let y_offset = offset / style.user_per_row();
        doc = doc.add(drawer.contributor_info().await.set(
            "transform",
            format!(
                "translate({}, {})",
                style.card_width() * x_offset,
                170 * y_offset
            ),
        ));
        offset += 1
    }
    (
        doc,
        (offset + style.user_per_row() - 1) / style.user_per_row(),
    )
}

pub async fn draw_svg(
    data: &Vec<(Period, Vec<Contributor>)>,
    repo: &str,
    style: &dyn styles::Style,
) -> Document {
    let mut doc = Document::new().add(
        Rectangle::new()
            .set("x", 0.5)
            .set("y", 0.5)
            .set("rx", 4.5)
            .set("height", "99.5%")
            .set("width", "99.5%")
            .set("fill", "#CCE4CC")
            .set("stroke", "#003D00"),
    );

    let authors: HashSet<(&str, &AccountType)> = data
        .iter()
        .flat_map(|it| &it.1)
        .map(|it| (it.author.as_str(), &it.account_type))
        .collect();

    let mut defs = Definitions::new();
    for author in authors {
        defs = defs.add(
            Image::new()
                .set("id", author.0)
                .set("xlink:href", avatar_base64_for_user(author.0, &author.1).await)
                .set("height", "100")
                .set("width", "100"),
        )
    }
    doc = doc.add(defs);

    let mut height = 30;

    for ele in data {
        if ele.1.is_empty() {
            continue;
        }

        let title = format!(
            "{} ({}-{})\n",
            ele.0.name,
            &ele.0.start[..10],
            &ele.0.end[..10]
        );
        let title = Group::new().set("transform", "translate(25, 10)").add(
            element::Text::new(title)
                .set("class", "header"),
        );
        let (mut card, offset) = draw_period(&ele.1, &ele.0.start, &ele.0.end, repo, style).await;
        card = card.set("transform", "translate(0, 25)");

        doc = doc.add(
            Group::new()
                .set("transform", format!("translate(0, {})", height))
                .add(title)
                .add(card),
        );
        height += offset * 170 + 30;
    }
    doc.set("height", height)
        .set("width", style.card_width() * style.user_per_row() + 20)
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .add(Style::new(CSS))
}

#[cfg(test)]
mod tests {
    use svg::{
        node::{element::ForeignObject, Text},
        Document,
    };

    #[test]
    fn test_tag() {
        let obj = ForeignObject::new().add(Text::new("123"));
        let mut doc = Document::new();
        doc = doc.add(obj);
        println!("{:#?}", &doc);
    }
}
