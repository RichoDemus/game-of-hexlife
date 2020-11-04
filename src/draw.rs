use quicksilver::Graphics;
use crate::core::Core;
use quicksilver::graphics::{Image, Color};
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::golem::glow::COLOR;

pub fn draw(gfx: &mut Graphics, core: &Core, image: &Image) {



    for coordinate in &core.grid {
        let raw_x = coordinate.x;
        let raw_y = coordinate.y;
        let x = (raw_x *35  + 18*raw_y) as f32;
        let y = (raw_y *30) as f32;
        let region = Rectangle::new(Vector::new(100.0 + x, 100.0 + y), image.size()/10.);
        // if raw_x == 0 && raw_y == 0 {
        //     gfx.draw_image_tinted(&image, region, Color::RED);
        // } else {
            gfx.draw_image(&image, region);
        // }
    }

}
