use frug;
use math;
use rand::Rng;

#[derive(PartialEq)]
enum Transition {
    Intro,
    Outro,
    Full,
    None,
}

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    // setup
    frug_instance.set_background_color(frug::create_color(0.205, 0.39, 1.0, 1.0));

    // window data
    let window_w = 1000.0;
    let window_h = 800.0;
    frug_instance.set_window_size(window_w, window_h);

    // Text images
    let frug_title_idx = frug_instance.load_texture(include_bytes!("img/frug_title.png"));
    let frug_title_scale = 1.0;
    let frug_title_w = 1155.0 / window_w * frug_title_scale;
    let frug_title_h = 376.0 / window_w * frug_title_scale;

    // transition data
    let tborder_tex_idx =
        frug_instance.load_texture(include_bytes!("img/frug_transition_border.png"));
    let tborder_scale = 1.5;
    let tborder_w = 160.0 / window_w * tborder_scale;
    let tborder_h = 70.0 / window_h * tborder_scale;
    let tborder_repeats = math::round::ceil((2.0 / tborder_w) as f64, 0) as i32;
    let mut transition = Transition::Full;
    let transition_color = [0.131, 0.027, 0.033];
    let transition_speed: f32 = 0.02;
    let mut transition_height = 1.0;

    // slides data
    let slide = 0;

    // load frog textures
    let frogo_idle = vec![
        frug_instance.load_texture(include_bytes!("img/frog_idle/1.png")),
        frug_instance.load_texture(include_bytes!("img/frog_idle/2.png")),
        frug_instance.load_texture(include_bytes!("img/frog_idle/3.png")),
        frug_instance.load_texture(include_bytes!("img/frog_idle/4.png")),
        frug_instance.load_texture(include_bytes!("img/frog_idle/5.png")),
        frug_instance.load_texture(include_bytes!("img/frog_idle/6.png")),
    ];
    let frogo_walk = vec![
        frug_instance.load_texture(include_bytes!("img/frog_walk/1.png")),
        frug_instance.load_texture(include_bytes!("img/frog_walk/2.png")),
        frug_instance.load_texture(include_bytes!("img/frog_walk/3.png")),
        frug_instance.load_texture(include_bytes!("img/frog_walk/4.png")),
    ];
    let frogo_scale = 1.5;
    let frogo_size = 192.0 * frogo_scale;
    let frogo_w = frogo_size / window_w;
    let frogo_h = frogo_size / window_h;
    let frogo_idle_spd = 5;
    let frogo_walk_spd = 8;
    let mut frogo_is_idle = true;
    let mut frogo_tex_timer = frogo_idle_spd;
    let mut frogo_tex_loc = 0;
    let mut frogo_tex_idx = frogo_idle[frogo_tex_loc];

    // grass
    let grass_tex_idx = frug_instance.load_texture(include_bytes!("img/grass.png"));
    let grass_scale = 1.5;
    let grass_w = 160.0 / window_w * grass_scale;
    let grass_h = 240.0 / window_h * grass_scale;
    let grass_repeats = math::round::ceil((2.0 / grass_w) as f64, 0) as i32;

    // clouds
    let cloud_tex_idx = frug_instance.load_texture(include_bytes!("img/cloud1.png"));
    let cloud_w = 260.0 / window_w;
    let cloud_h = 130.0 / window_h;
    let clouds_y = (0.7, 1.05);
    let mut clouds_data: Vec<[f32; 4]> = Vec::new();
    let clouds_time = (150, 250);
    let mut clouds_timer = rand::thread_rng().gen_range(clouds_time.0..clouds_time.1);
    let clouds_scale = (0.85, 1.25);
    let clouds_speed = (0.001, 0.003);
    let mut clouds_to_delete: Vec<usize> = Vec::new();

    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        // ****     LOGIC   ****

        // update transitions
        if transition == Transition::Outro {
            transition_height -= transition_speed;
            if transition_height <= 0.0 {
                transition = Transition::None;
            }
        } else if transition == Transition::Intro {
            transition_height += transition_speed;
            if transition_height >= 1.0 {
                transition = Transition::Full;
            }
        }

        // update frogo animation
        frogo_tex_timer -= 1;
        if frogo_tex_timer <= 0 {
            // set to next frame
            if frogo_is_idle {
                frogo_tex_timer = frogo_idle_spd;
                frogo_tex_loc += 1;
                if frogo_tex_loc >= 6 {
                    frogo_tex_loc = 0;
                }

                frogo_tex_idx = frogo_idle[frogo_tex_loc];
            } else {
                frogo_tex_timer = frogo_walk_spd;
                frogo_tex_loc += 1;
                if frogo_tex_loc >= 4 {
                    frogo_tex_loc = 0;
                }

                frogo_tex_idx = frogo_walk[frogo_tex_loc];
            }
        }

        // update clouds data
        //      create clouds
        clouds_timer -= 1;
        if clouds_timer <= 0 {
            clouds_timer = rand::thread_rng().gen_range(clouds_time.0..clouds_time.1);

            clouds_data.push([
                1.0,                                                          // x
                rand::thread_rng().gen_range(clouds_y.0..clouds_y.1),         // y
                rand::thread_rng().gen_range(clouds_scale.0..clouds_scale.1), // scale
                rand::thread_rng().gen_range(clouds_speed.0..clouds_speed.1), // speed
            ]);
        }
        //      move clouds & check if should delete them
        for i in 0..clouds_data.len() {
            clouds_data[i][0] -= clouds_data[i][3];

            if clouds_data[i][0] + (clouds_data[i][2] * cloud_w) < -1.0 {
                clouds_to_delete.push(i);
            }
        }
        //      delete clouds
        /*if clouds_to_delete.len() > 0 {
            for i in clouds_to_delete.iter().rev() {
                clouds_data.remove(*i);
                println!("removed cloud");
            }
        }*/
        for i in (0..clouds_to_delete.len()).rev() {
            println!("{}", clouds_to_delete[i]);
            clouds_data.remove(clouds_to_delete[i]);
            println!("removed cloud");
        }
        clouds_to_delete.clear();

        // ****     INPUT   ****
        if input.key_pressed(frug::VirtualKeyCode::Right) {
            // advance
            if slide == 0 {
                // from title screen to beggining of presentation
                if transition == Transition::Full {
                    transition = Transition::Outro;
                    transition_height = 1.0;
                }
            }
        }

        // ****     RENDER  ****
        instance.clear();

        // render if not in full transition
        if transition != Transition::Full {
            // render clouds
            for cloud in clouds_data.iter() {
                instance.add_tex_rect(
                    cloud[0],
                    cloud[1],
                    cloud_w * cloud[2],
                    cloud_h * cloud[2],
                    cloud_tex_idx,
                    false,
                    false,
                );
            }

            // render grass
            for i in 0..grass_repeats {
                instance.add_tex_rect(
                    -1.0 + grass_w * i as f32,
                    -1.0 + grass_h,
                    grass_w,
                    grass_h,
                    grass_tex_idx,
                    i % 2 == 0,
                    false,
                );
            }

            // render frogo
            instance.add_tex_rect(
                -0.85,
                -1.04 + grass_h + frogo_h,
                frogo_w,
                frogo_h,
                frogo_tex_idx,
                false,
                false,
            );
        }

        // render transition squares
        match transition {
            Transition::Full => {
                instance.add_colored_rect(-1.0, 1.0, 2.0, 2.0, transition_color);
            }
            Transition::None => {}
            _ => {
                // * This works for either intro or outro *
                // top rectangle
                instance.add_colored_rect(-1.0, 1.0, 2.0, transition_height, transition_color);

                // bottom rectangle
                instance.add_colored_rect(
                    -1.0,
                    -1.0 + transition_height,
                    2.0,
                    transition_height,
                    transition_color,
                );
            }
        }

        // render transition borders & text
        if transition != Transition::None {
            // borders
            for i in 0..tborder_repeats {
                // upper border
                instance.add_tex_rect(
                    -1.0 + tborder_w * (i) as f32,
                    1.0 - transition_height + tborder_h,
                    tborder_w,
                    tborder_h,
                    tborder_tex_idx,
                    false,
                    false,
                );

                // lower border
                instance.add_tex_rect(
                    -1.0 + tborder_w * (i) as f32,
                    -1.0 + transition_height,
                    tborder_w,
                    tborder_h,
                    tborder_tex_idx,
                    false,
                    true,
                );
            }

            // frug title
            instance.add_tex_rect(
                0.0 - frug_title_w / 2.0,
                1.0 - transition_height + 0.65,
                frug_title_w,
                frug_title_h,
                frug_title_idx,
                false,
                false,
            )
        }

        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
