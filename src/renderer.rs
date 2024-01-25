use std::path::{Path, PathBuf};

use color_eyre::{Result, eyre::bail};
use usvg_text_layout::TreeTextToPath;


#[derive(Debug, Clone)]
enum ColorType {
    Array(Vec<String>),
    Object(Vec<(String, String)>),
    None
}


#[derive(Debug, Clone)]
pub struct Renderer {
    fontdb: usvg_text_layout::fontdb::Database,
    colors: ColorType,
    size: (u32, u32),
    pub count: u64,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let mut db = usvg_text_layout::fontdb::Database::new();
        let input_colors= String::from("0d6efd,6c757d,198754,0dcaf0,ffc107,dc3545,f8f9fa,212529,ffffff,000000");
        db.load_system_fonts();

        let mut this = Self {
            fontdb: db,
            colors: ColorType::None,
            size: (width, height),
            count: 0,
        };

        let colors = if input_colors.contains(':') {
            //? object
            let obj = input_colors.split(',').map(|s| {
                let s = s.split(':').collect::<Vec<&str>>();

                if s.len() < 2 {
                    log::error!("Invalid color object, try checking help");
                    return None;
                }

                Some((s[0].to_string(), s[1].to_string()))
            }).collect::<Vec<Option<(String, String)>>>();

            let mut colors = Vec::new();

            for c in obj {
                if let Some(c) = c {
                    colors.push(c);
                }
            }

            ColorType::Object(colors)

        } else {
            //? list
            // let colors = args.colors.split(",").map(|s| {
            //     s.to_string()
            // })
            // .collect::<Vec<String>>();

            let mut colors = Vec::new();

            for color in input_colors.split(',') {
                colors.push(color.to_string())
            }
            ColorType::Array(colors)
        };

        this.colors = colors;
        Ok(this)
    }


    pub fn render(&mut self, fi: &Path) -> Result<()> {
        match fi.extension() {
            Some(e) if e.to_str() == Some("svg") => {},
            Some(_) |
            None => {
                log::warn!("Filer {:?} is not of type SVG", fi);
                // util::logger::warning(format!("File '{}' is not of SVG type", fi.clone().to_str().unwrap()));
                bail!("Failed to render");
            }
        };

        match self.colors.clone() {
            ColorType::Array(c) => {
                for color in c {
                    log::info!("Rendering the color {color:?}");
                    let fo = PathBuf::from("image.png");
                    self.render_one(fi, &fo, &color)?;

                }
            },
            ColorType::Object(c) => {
                for o in c {
                    log::info!("Rendering the color {:?}", o);
                    let fo = PathBuf::from("image.png");
                    self.render_one(fi, &fo, &o.1)?;

                }
            },
            ColorType::None => unreachable!(),
        }

        Ok(())
    }

    fn render_one(&mut self, fi: &Path, fo: &Path, color: &String) -> Result<()>{

        if fo.exists() {
            log::warn!("File {fo:?} exists, skipping");
            return Ok(());
        }

        let svg = self.set_color(&self.get_svg_data(fi)?, &color);

        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.resources_dir = std::fs::canonicalize(fi.clone())
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));

        let mut tree = match usvg::Tree::from_data(svg.as_bytes(), &opt) {
            Ok(v) => v,
            Err(_) => {
                log::error!("Failed to parse {fi:?}");
                bail!("");
            }
        };

        tree.convert_text(&self.fontdb);

        let mut pixmap = tiny_skia::Pixmap::new(self.size.0, self.size.1).unwrap();

        log::info!("Rendering {fo:?}");

        //? maybe handle this and possibly throw error if its none
        let _ = resvg::render(
            &tree,
            usvg::FitTo::Size(self.size.0, self.size.1),
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        );


        pixmap.save_png(fo)?;
        self.count += 1;
        Ok(())
    }


    fn set_color(&self, svg: &String, color: &String) -> String {
        svg.replace("fill=\"currentColor\"", &format!("fill=\"#{}\"", color))
    }

    fn get_svg_data(&self, fi: &Path) -> Result<String>{
        match std::fs::read_to_string(fi) {
            Ok(d) => Ok(d),
            Err(_) => {
                log::error!("File {fi:?} does not exist");
                bail!("File {fi:?} does not exist");
            }
        }
    }
}