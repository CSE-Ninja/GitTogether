use svg::node::element::{Group, SVG};



use self::{compact::CompactStyle, full::FullStyle};

pub mod compact;
pub mod full;

pub trait Style {
    fn draw_contribution_item(&self, icon: SVG, info: &str, value: u32, link: &str) -> Group;
    fn draw_title(&self, title: &str, avatar: &str) -> Group;
    fn user_per_row(&self) -> u32;
    fn card_width(&self) -> u32;
}

pub fn get_style(style: &str) -> Box<dyn Style> {
    if style == "compact" {
        Box::new(CompactStyle {})
    } else {
        Box::new(FullStyle {})
    }
}