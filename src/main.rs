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
    frug_instance.set_background_color(frug::create_color(0.2, 0.2, 1.0, 1.0));

    // transition data
    let mut transition = Transition::Full;
    let transition_color = [0.0, 0.0, 0.0];
    let transition_speed: f32 = 0.02;
    let mut transition_height = 0.0;

    // slides data
    let slide = 0;

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

        // ****     INPUT   ****
        if input.key_pressed(frug::VirtualKeyCode::Right) {
            // advance
            if slide == 0 {
                // from title screen to beggining of presentation
                if transition == Transition::Full {
                    transition = Transition::Outro;
                    transition_height = 1.0;
                    println!("trans outro");
                }
            }
        }

        // ****     RENDER  ****
        instance.clear();

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
