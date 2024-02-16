use svg::node::{
    self,
    element::{self, Anchor, Group, Use},
};

use super::Style;

pub struct CompactStyle{}

impl Style for CompactStyle {
    fn card_width(&self) -> u32 {
        190
    }

    fn draw_contribution_item(
        &self,
        icon: svg::node::element::SVG,
        _info: &str,
        value: u32,
        link: &str,
    ) -> svg::node::element::Group {
        let mut group = Group::new();
        // .set("transform", format!("translate(0, {})", offset * 25));
        group = group.add(icon);

        let text = element::Text::new()
            .add(node::Text::new(value.to_string()))
            .set("class", "stat")
            .set("x", 25)
            .set("y", 12.5);
        if !link.is_empty() {
            group.add(Anchor::new().set("xlink:href", link).add(text))
        } else {
            group.add(text)
        }
    }

    fn draw_title(&self, title: &str, avatar: &str) -> svg::node::element::Group {
        let title = Group::new().add(
            Anchor::new()
                .add(
                    element::Text::new()
                        .add(node::Text::new(title))
                        .set("class", "stat bold"),
                )
                .set(
                    "xlink:href",
                    format!("https://github.com/{}", title).to_string(),
                ),
        );
        let img = Group::new()
            .add(
                Use::new()
                    .set("xlink:href", avatar),
            )
            .set("transform", "translate(0, 20)");
        Group::new()
            .set("transform", "translate(25, 20)")
            .add(title)
            .add(img)
    }

    fn user_per_row(&self) -> u32 {
        5
    }
}