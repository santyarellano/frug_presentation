use frug;

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
    frug_instance.set_background_color(frug::create_color(0.3, 0.3, 1.0, 1.0));

    // window data
    let window_w = 1000.0;
    let window_h = 800.0;
    frug_instance.set_window_size(window_w, window_h);

    // transition data
    let mut transition = Transition::Full;
    let transition_color = [0.0, 0.0, 0.0];
    let transition_speed: f32 = 0.02;
    let mut transition_height = 0.0;

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
            // render frogo
            instance.add_tex_rect(-0.7, -0.2, frogo_w, frogo_h, frogo_tex_idx, false, false);
        }

        // render transition
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

        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
