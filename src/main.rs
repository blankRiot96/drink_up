use raylib::prelude::*;

fn get_texture(src: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    let img = Image::load_image(src).expect("Could not load image");

    rl.load_texture_from_image(&thread, &img)
        .expect("could not load texture from image")
}
fn main() {
    let win_width = 1100;
    let win_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(win_width, win_height)
        .title("Drink Up!")
        .build();

    let bg = get_texture("assets/background.png", &mut rl, &thread);
    let bottle = get_texture("assets/bottle.png", &mut rl, &thread);
    let bottle_filled = get_texture("assets/bottle_fill.png", &mut rl, &thread);

    let bottle_pos = Vector2::new(75.0, 125.0);
    let water_consumed = 100.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_texture(&bg, 0, 0, Color::WHITE);

        d.draw_texture_v(&bottle, bottle_pos, Color::WHITE);
        d.draw_texture_pro(
            &bottle_filled,
            Rectangle::new(
                0.0,
                0.0,
                bottle_filled.width as f32,
                bottle_filled.height as f32 - water_consumed,
            ),
            Rectangle::new(
                bottle_pos.x,
                bottle_pos.y,
                bottle.width as f32,
                bottle.height as f32,
            ),
            bottle_pos,
            180.0,
            Color::WHITE,
        )
    }
}
