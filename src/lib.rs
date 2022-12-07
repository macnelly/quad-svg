use macroquad::{texture::Texture2D, prelude::ImageFormat};
use resvg::{usvg_text_layout::{fontdb, TreeTextToPath}};

/*
    TODO: include font file in package to not rely on system fonts
*/

/*
    rasterize svg to png image
*/
pub fn svg_to_png(svg_str:&str)->Vec<u8>{
    let opt = resvg::usvg::Options::default();
    let mut tree = resvg::usvg::Tree::from_str(svg_str, &opt).unwrap();
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    tree.convert_text(&fontdb, opt.keep_named_groups);
    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    
    resvg::render(
        &tree,
        resvg::usvg::FitTo::Original,
        resvg::tiny_skia::Transform::default(),
        pixmap.as_mut()
    ).unwrap();
    pixmap.encode_png().unwrap()
}

/*
    rasterize svg and create Texture2D
*/
pub fn svg_to_texture(svg_str:&str)->Texture2D{
    let png_data = svg_to_png(&svg_str);
    Texture2D::from_file_with_format(&png_data, Some(ImageFormat::Png))
}
