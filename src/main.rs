use macroquad::prelude::*;
use macroquad::audio::*;

fn config() -> Conf {
    Conf {
        window_title: "Whack a Doge".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

enum Scene {
    PLAY,
}

fn point_rect_collision(mouse_x: &f32, mouse_y: &f32, rect: &Rect) -> bool {
    if mouse_x >= &rect.x && mouse_x <= &(&rect.x + &rect.w) && mouse_y >= &rect.y && mouse_y <= &(&rect.y + &rect.h) {    
        return true;
    }
    return false;
}

struct Doge {
	img: Texture2D,
	pos: Vec2,
	rect: Rect,
}

impl Doge {
	fn draw(&mut self) {

		self.rect.x = self.pos.x;
		self.rect.y = self.pos.y;
		self.rect.w = 70.;
		self.rect.h = 70.;


		draw_texture_ex(self.img, self.pos.x, self.pos.y, WHITE, DrawTextureParams{
            		dest_size: Some(Vec2::new(70., 70.)),
            		..Default::default()
            	});

		// draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 3., YELLOW);
	}
}

#[macroquad::main(config)]
async fn main() {
    // load assets
    let doge_img = load_texture("texture/doge.png").await.unwrap();
    let palu1 = load_texture("texture/palu1.png").await.unwrap();
    let palu2 = load_texture("texture/palu2.png").await.unwrap();
    let tanah_img = load_texture("texture/tanah.png").await.unwrap();

    // load sfx
    let bonk_sfx = load_sound("texture/bonk_sfx.ogg").await.unwrap();

    // load font
    let f = load_ttf_font("texture/nasalization-rg.ttf").await.unwrap();

    let mut doge = Doge {
    	img: doge_img,
    	pos: Default::default(),
    	rect: Default::default(),
    };

    let mut current_palu = palu1;
    let row : i32 = 3;
    let col : i32 = 3;
    let mut tanah_list = vec![];
    let mut rng = rand::gen_range(0, 8);
    let mut last_update = get_time();
    let mut last_show = get_time();
    let mut last_count = get_time();
    let mut countdown = 30;
    let mut start_countdown = 5;
    let mut show_count = 3;
    let mut score = 0;

    for i in 0..row {
        for j in 0..col {
            tanah_list.push(Rect::new((i as f32) * 200. + 140., (j as f32) * 200. + 100., 100., 50.));
        }
    }

    doge.pos = Vec2::new(tanah_list[rng].x  + 20., tanah_list[rng].y);

    let mut current_scene = Scene::PLAY;
    show_mouse(false);

    // let bg = Color::from_rgba(17, 166, 41, Default::default());
    
    loop {
        clear_background(GREEN);

        let (mouseX, mouseY) = mouse_position();

        match current_scene {
            Scene::PLAY => {

            	if get_time() - last_update > 1. && start_countdown > 0 {
            		last_update = get_time();
            		start_countdown -= 1;
            	}

            	// show up
            	// tanah_list[rng].y - 40.

            	doge.pos.y -= 2.;
            	if doge.pos.y <= tanah_list[rng].y - 40. {
            		doge.pos.y = tanah_list[rng].y - 40.;
            	}

            	if start_countdown == 0 {
	            		if get_time() - last_show > 0.6 && show_count > 0 {
		                	last_show = get_time();
		                	show_count -= 1;
	                	}

	                	if get_time() - last_count > 1. && countdown > 0 {
		                	last_count = get_time();
		                	countdown -= 1;
	                	}
            		doge.draw();
            	}
    			

                for tanah in tanah_list.iter_mut() {
                    draw_rectangle(tanah.x, tanah.y + 40., tanah.w, tanah.h + 50., GREEN);
                    draw_texture_ex(tanah_img, tanah.x, tanah.y, WHITE, DrawTextureParams{
                    	dest_size: Some(Vec2::new(100., 50.)),
                    	..Default::default()
                    });
                }


                if show_count == 0 && countdown > 0 {
                	rng = rand::gen_range(0, 8);
                    doge.pos = Vec2::new(tanah_list[rng].x  + 20., tanah_list[rng].y + 20.);
                    show_count = 3;
                }

                draw_text_ex(format!("Score: {}", score).as_str(), 10., 35., TextParams{
                	font: f,
                	font_size: 30,
                	..Default::default()
                });

                draw_text_ex(format!("{}", countdown).as_str(), screen_width()/2., 35., TextParams{
                	font: f,
                	font_size: 30,
                	..Default::default()
                });

                if start_countdown > 0 {
                	draw_text_ex(format!("{}", start_countdown).as_str(), screen_width()/2., screen_height()/2., TextParams{
                	font: f,
                	font_size: 50,
                	..Default::default()
                });
                }

                if is_mouse_button_pressed(MouseButton::Left) && point_rect_collision(&mouseX, &mouseY, &doge.rect) && countdown > 0 {
                	score += 1;
                	play_sound_once(bonk_sfx);
                	rng = rand::gen_range(0, 8);
                    doge.pos = Vec2::new(tanah_list[rng].x  + 20., tanah_list[rng].y + 20.);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    current_palu = palu2;
                } else if is_mouse_button_released(MouseButton::Left) {
                    current_palu = palu1;
                }


                // game over condition
                if countdown == 0 {
                	draw_text_ex("Tekan \"R\" untuk restart", 180., screen_height()/2., TextParams{
                		font: f,
                		font_size: 35,
                		..Default::default()
                	});


                	if is_key_pressed(KeyCode::R) {
                		countdown = 30;
                		score = 0;
                		rng = rand::gen_range(0, 8);
                    	doge.pos = Vec2::new(tanah_list[rng].x  + 20., tanah_list[rng].y + 20.);
                	}
                }

                draw_texture(current_palu, mouseX - palu1.width()/2., mouseY - palu1.height()/2., WHITE);
            },
        }

        next_frame().await
    }
}