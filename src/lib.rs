use macroquad::{prelude::ImageFormat, texture::Texture2D};
use resvg::usvg::{TreeParsing, TreeTextToPath};

pub use resvg::tiny_skia::Transform;

/*
    TODO: include font file in package to not rely on system fonts
*/

#[derive(Debug)]
pub enum SVGParseError {
    SVGSourceParse,
    NoFonts,
    InvalidTransform,
    PNGEncoding,
}

impl std::error::Error for SVGParseError {}
impl std::fmt::Display for SVGParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SVGParseError::SVGSourceParse => write!(f, "Invalid SVG source"),
            SVGParseError::NoFonts => write!(f, "No system fonts found"),
            SVGParseError::InvalidTransform => write!(f, "Invalid texture transform"),
            SVGParseError::PNGEncoding => write!(f, "Failed to encode PNG"),
        }
    }
}

/*
    rasterize svg and create Texture2D
*/
pub fn svg_to_texture(svg_source: &str, transform: &Transform) -> Result<Texture2D, SVGParseError> {
    let png_data = svg_to_png(svg_source, transform)?;
    Ok(Texture2D::from_file_with_format(
        &png_data,
        Some(ImageFormat::Png),
    ))
}

/*
    rasterize svg to png image
*/
fn svg_to_png(svg_source: &str, transform: &Transform) -> Result<Vec<u8>, SVGParseError> {
    let mut tree = resvg::usvg::Tree::from_str(svg_source, &resvg::usvg::Options::default())
        .map_err(|_| SVGParseError::SVGSourceParse)?;

    if tree.has_text_nodes() {
        let mut fontdb = resvg::usvg::fontdb::Database::new();
    
        if fontdb.is_empty() {
            return Err(SVGParseError::NoFonts);
        }
    
        fontdb.load_system_fonts();
        tree.convert_text(&fontdb);
    }

    let mut pixmap = resvg::tiny_skia::Pixmap::new(
        (transform.sx * tree.size.width() as f32) as u32,
        (transform.sy * tree.size.height() as f32) as u32,
    )
    .ok_or(SVGParseError::InvalidTransform)?;

    let tree = resvg::Tree::from_usvg(&tree);
    tree.render(*transform, &mut pixmap.as_mut());

    pixmap.encode_png().map_err(|_| SVGParseError::PNGEncoding)
}
