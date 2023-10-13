
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use crate::helper::color_info::{get_rgb_color, TextureColor};


pub fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    return if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(
                &mut square_texture,
                |texture| {
                    texture.set_draw_color(get_rgb_color(color));
                    texture.clear();
                }
            )
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

