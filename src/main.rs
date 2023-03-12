use std::collections::HashMap;
use std::time::{self, Instant};

use rand::Rng;
use raylib::{ffi::IsKeyDown, prelude::*};

fn get_texture(src: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    let img = Image::load_image(src).expect("Could not load image");

    rl.load_texture_from_image(&thread, &img)
        .expect("could not load texture from image")
}

#[derive(PartialEq)]
enum State {
    Game,
    GameOver,
    MainMenu,
}

pub struct Time {
    pub time_to_pass: f32,
    pub start: Instant,
}

impl Default for Time {
    fn default() -> Self {
        let start = time::Instant::now();
        Time {
            time_to_pass: 0.0,
            start,
        }
    }
}

impl Time {
    pub fn update(&mut self) -> bool {
        if self.start.elapsed().as_secs_f32() > self.time_to_pass {
            self.start = time::Instant::now();
            return true;
        }
        false
    }
}

struct GameKey {
    pos: Vector2,
    character: char,
    tint: Color,
    hashchars: HashMap<char, Color>,
    alive: bool,
    _active: bool,
}

impl Default for GameKey {
    fn default() -> Self {
        let hashchars = HashMap::from([
            ('A', Color::from((255, 0, 0, 255))),
            ('B', Color::from((255, 128, 128, 255))),
            ('C', Color::from((255, 158, 255, 255))),
            ('D', Color::from((9, 25, 255, 255))),
            ('E', Color::from((178, 255, 0, 255))),
            ('F', Color::from((0, 156, 255, 255))),
            ('G', Color::from((5, 25, 87, 255))),
            ('H', Color::from((21, 0, 164, 255))),
            ('I', Color::from((234, 12, 5, 255))),
            ('J', Color::from((15, 205, 89, 255))),
            ('K', Color::from((255, 0, 255, 255))),
            ('L', Color::from((9, 255, 255, 255))),
            ('M', Color::from((25, 255, 255, 255))),
            ('N', Color::from((70, 255, 255, 255))),
            ('O', Color::from((255, 129, 255, 255))),
            ('P', Color::from((255, 30, 255, 255))),
            ('Q', Color::from((255, 5, 255, 255))),
            ('R', Color::from((255, 190, 255, 255))),
            ('S', Color::from((255, 255, 24, 255))),
            ('T', Color::from((255, 255, 123, 255))),
            ('U', Color::from((4, 25, 55, 255))),
            ('V', Color::from((63, 25, 51, 255))),
            ('W', Color::from((200, 100, 91, 255))),
            ('X', Color::from((5, 155, 5, 255))),
            ('Y', Color::from((155, 200, 255, 255))),
            ('Z', Color::from((39, 5, 255, 255))),
        ]);
        GameKey {
            pos: Vector2::zero(),
            character: 'A',
            tint: Color::WHITE,
            hashchars: hashchars,
            alive: true,
            _active: false,
        }
    }
}

impl GameKey {
    fn set_color(&mut self) {
        self.tint = self.hashchars.get(&self.character).unwrap().to_owned();
    }
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
    let key_img = get_texture("assets/key.png", &mut rl, &thread);

    let mut keys = vec![GameKey {
        pos: Vector2::new(800.0, -70.0),
        character: 'A',
        ..Default::default()
    }];

    for key in &mut keys {
        key.set_color();
    }

    let bottle_pos = Vector2::new(75.0, 125.0);
    let base_y = 450;

    let mut water_consumed = (bottle.height() / 2) as f32;
    let mut key_gen_time = Time {
        time_to_pass: 3.0,
        ..Default::default()
    };
    let mut gen_time_increase = Time {
        time_to_pass: 2.0,
        ..Default::default()
    };
    let mut dt = 0.0;
    let mut consumption_rate = 30.0;
    let mut key_drop_factor = 1.0;

    let mut score = 0;
    let mut state = State::Game;

    while !rl.window_should_close() {
        let start = time::Instant::now();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if state == State::Game {
            // -- UPDATE
            if key_gen_time.update() {
                let n_keys = rand::thread_rng().gen_range(1..4);

                for i in 0..n_keys {
                    let i = i as f32;
                    let rand_x = rand::thread_rng().gen_range(600..win_width - 70) as f32;
                    let rand_char_ascii = rand::thread_rng().gen_range(65..90);
                    let rand_char = char::from_u32(rand_char_ascii).unwrap();
                    let mut temp_key = GameKey {
                        pos: Vector2::new(rand_x, -70.0 - (i * 70.0)),
                        character: rand_char,
                        ..Default::default()
                    };
                    temp_key.set_color();
                    keys.push(temp_key);
                }
            }

            for key in &mut keys {
                key.pos.y += 0.03 * key_drop_factor;

                if key.pos.y > win_height as f32 {
                    key.alive = false;
                }

                key._active = key.pos.y + key_img.height() as f32 > base_y as f32
                    && key.pos.y < base_y as f32;

                unsafe {
                    if key._active && IsKeyDown(key.character as i32) {
                        water_consumed += 20.0;
                        key.alive = false;
                        score += 100;
                    }
                }
            }
            keys.retain(|x| x.alive);

            if gen_time_increase.update() && key_drop_factor < 6.0 {
                consumption_rate += 5.0;
                key_drop_factor += 0.5;
            }

            if key_drop_factor < 6.0 {
                key_gen_time.time_to_pass -= 0.0001 * dt;
            }
            water_consumed -= consumption_rate * dt;

            // -- FRAMERATE INDEPENDANCE
            let raw_dt = start.elapsed().as_secs_f32();
            dt = raw_dt * 60.0;

            // -- GAME LOSS
            if water_consumed <= 0.0 {
                state = State::GameOver;
            }

            // -- DRAW
            d.draw_texture(&bg, 0, 0, Color::WHITE);

            d.draw_texture_v(&bottle, bottle_pos, Color::WHITE);

            d.draw_texture_rec(
                &bottle_filled,
                Rectangle::new(
                    0.0,
                    bottle_filled.height() as f32 - water_consumed,
                    bottle.width() as f32,
                    water_consumed,
                ),
                Vector2::new(
                    bottle_pos.x,
                    bottle_pos.y + (bottle_filled.height() as f32 - water_consumed),
                ),
                Color::WHITE,
            );

            // -- DRAW - KEYS
            for key in &keys {
                d.draw_texture_v(&key_img, key.pos, key.tint);
                d.draw_text(
                    key.character.to_string().as_str(),
                    key.pos.x as i32 + 15,
                    key.pos.y as i32 + 15,
                    50,
                    Color::BLACK,
                );
            }

            // -- UI LINES
            d.draw_rectangle(600, base_y, win_width, 10, Color::BLACK);
            d.draw_rectangle(600, 0, 10, win_height, Color::BLACK);

            // -- INFORMATION UI
            d.draw_text(format!("SCORE: {}", score).as_str(), 0, 0, 54, Color::BLACK);
        } else if state == State::GameOver {
            d.draw_text("GAME OVER\n BIATCH", 150, win_height / 2, 120, Color::WHITE);
        }
    }
}
