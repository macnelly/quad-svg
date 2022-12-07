use macroquad::{prelude::*, texture::draw_texture, window::{clear_background, next_frame, screen_width, screen_height}};

struct Position{
   x: f32,
   y: f32,
   forward: bool,
   speed: f32,
}
#[macroquad::main("ferris in svg")]
async fn main() {
    let svg_data = "<svg
    width=\"194\"
    height=\"194\"
    viewBox=\"0 0 51.329165 51.329165\"
    version=\"1.1\"
    id=\"svg12924\"
    xmlns=\"http://www.w3.org/2000/svg\"
    xmlns:svg=\"http://www.w3.org/2000/svg\">
   <defs
      id=\"defs12921\" />
     <g
        id=\"g53240\"
        transform=\"translate(-1.2067852,-5.5310822)\">
       <path
          style=\"opacity:1;fill:#ffff00;fill-opacity:1;stroke:#000000;stroke-width:3.77953;stroke-opacity:1\"
          id=\"path45291\"
          d=\"M 195.22434,96.206726 132.14254,76.775922 80.373871,117.72558 79.360315,51.72678 24.417474,15.145991 86.872861,-6.21271 l 17.812129,-63.557832 39.6131,52.798401 65.95134,-2.700112 -37.97314,53.989908 z\"
          transform=\"matrix(0.26458333,0,0,0.26458333,-4.1088137,24.896505)\" />
       <text
          xml:space=\"preserve\"
          style=\"font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:14.8167px;line-height:1.25;font-family:Caladea;-inkscape-font-specification:'Caladea, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;fill-opacity:1;stroke:none;stroke-width:0.264583\"
          x=\"13.973488\"
          y=\"36.092587\"
          id=\"text52495\"><tspan
            id=\"tspan52493\"
            style=\"font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:14.8167px;font-family:Caladea;-inkscape-font-specification:'Caladea, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;stroke-width:0.264583\"
            x=\"13.973488\"
            y=\"36.092587\">Text</tspan></text>
     </g>
 </svg>";
    let texture = quad_svg::svg_to_texture(&svg_data);
    let mut positions: Vec<Position> = vec![];
    for _i in 0..100{
      positions.push(Position { 
         x: rand::gen_range(0.0, screen_width()-texture.width()),
         y: rand::gen_range(0.0, screen_height()-texture.height()),
         forward: rand::gen_range(0,2) == 1,
         speed: rand::gen_range(0.5, 5.0),
      })
    }
    loop {
        clear_background(LIGHTGRAY);
        for i in 0..positions.len() {
            draw_texture(
               texture,
               positions[i].x,
               positions[i].y,
               GRAY,
            );
            if positions[i].forward {
               positions[i].y += positions[i].speed
            }
            else {
               positions[i].y -= positions[i].speed
            }
            if positions[i].y + texture.height() > screen_height() {
               positions[i].y = screen_height() - texture.height();
               positions[i].forward = false;
            }
            if positions[i].y < 0.0 {
               positions[i].y = 0.0;
               positions[i].forward = true;
            }
        }
        next_frame().await
}

}