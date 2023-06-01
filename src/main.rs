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

struct SlideObj {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    scale: f32,
    tex_idx: usize,
}

// initializes the contents of the next slide and deletes te contents of the previous one (moves to direction defined with to_right)
/*fn create_next_slide(
    slides_content: &mut Vec<Vec<SlideObj>>,
    slides_data: &Vec<Vec<SlideObj>>,
    to_right: bool,
    current_slide: usize,
) {
    let x_offset = if to_right { 2.0 } else { -2.0 };
    let next_slide = if to_right {
        current_slide + 1
    } else {
        current_slide - 1
    };

    // copy data items into content (only if slide data exists)
    if next_slide < slides_data.len() {
        let mut new_vec: Vec<SlideObj> = Vec::new();
        for data in slides_data[next_slide].iter() {
            new_vec.push(SlideObj {
                x: data.x + x_offset,
                y: data.y,
                w: data.w,
                h: data.h,
                scale: data.scale,
                tex_idx: data.tex_idx,
            });
        }

        slides_content.push(new_vec);
    }

    // delete previous data (if it exists)
    if !slides_content.is_empty() {
        let to_delete = if to_right {
            0
        } else {
        };
    }
}*/

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    // setup
    frug_instance.set_background_color(frug::create_color(0.205, 0.39, 1.0, 1.0));

    // window data
    let window_w = 1000.0;
    let window_h = 800.0;
    frug_instance.set_window_size(window_w, window_h);

    // gradient background
    let day = true;
    let top_bkg_color = if day {
        [0.205, 0.59, 1.0]
    } else {
        [0.1, 0.0, 0.05]
    };
    let bottom_bkg_color = if day {
        [0.75, 0.75, 1.0]
    } else {
        [0.05, 0.05, 0.1]
    };
    let background = [
        frug::Vertex {
            // top left
            position: [-1.0, 1.0, 0.0],
            color: top_bkg_color,
            ..Default::default()
        },
        frug::Vertex {
            // bottom left
            position: [-1.0, -1.0, 0.0],
            color: bottom_bkg_color,
            ..Default::default()
        },
        frug::Vertex {
            // bottom right
            position: [1.0, -1.0, 0.0],
            color: bottom_bkg_color,
            ..Default::default()
        },
        frug::Vertex {
            // top right
            position: [1.0, 1.0, 0.0],
            color: top_bkg_color,
            ..Default::default()
        },
    ];
    let background_indices = [0, 1, 3, 1, 2, 3];

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
    let mut slide = 0;
    let mut slide_in_transition: bool = false;
    let slide_speed = 0.01;
    let mut slide_transition_speed = 0.0;

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
    let frogo_walk_spd = 10;
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
    let grass_spd = 1.0;
    let mut grass_tiles: Vec<[f32; 3]> = Vec::new();
    //      init grass tiles
    for i in 0..grass_repeats {
        grass_tiles.push([
            -1.0 + grass_w * i as f32, // x
            -1.0 + grass_h,            // y
            (i % 2) as f32,            // flip x
        ]);
    }

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

    // mountain
    let mount_tex_idx = frug_instance.load_texture(include_bytes!("img/mountain.png"));
    let mount2_tex_idx = frug_instance.load_texture(include_bytes!("img/mountain_dark.png"));
    let mount_w = 640.0 / window_w;
    let mount_h = 400.0 / window_h;
    let mut mountains_back: Vec<[f32; 2]> = Vec::new();
    let mount_back_scale = 1.1;
    let mount_back_spd = 0.1;
    let mut mountains_front: Vec<[f32; 2]> = Vec::new();
    let mount_front_scale = 1.4;
    let mount_front_spd = 0.15;
    let gap_rng = (0.2, 0.4);
    //      init back mountains
    mountains_back.push([
        -1.4 + rand::thread_rng().gen_range(gap_rng.0..gap_rng.1), // x
        -1.0 + grass_h + mount_h * mount_back_scale,               // y
    ]);
    loop {
        // create mountains until full screen has mountains
        let last_x = mountains_back[mountains_back.len() - 1][0] + mount_w * mount_back_scale;
        let gap = rand::thread_rng().gen_range(gap_rng.0..gap_rng.1);
        mountains_back.push([
            last_x + gap,                                // x
            -1.0 + grass_h + mount_h * mount_back_scale, // y
        ]);

        // exit if this should be the last mountain to create
        if last_x + gap + (mount_w * mount_back_scale) > 1.0 {
            break;
        }
    }
    //      init front mountains
    mountains_front.push([
        -1.4 + rand::thread_rng().gen_range(gap_rng.0..gap_rng.1), // x
        -1.0 + grass_h + mount_h * mount_front_scale,              // y
    ]);
    loop {
        // create mountains until full screen has mountains
        let last_x = mountains_front[mountains_front.len() - 1][0] + mount_w * mount_front_scale;
        let gap = rand::thread_rng().gen_range(gap_rng.0..gap_rng.1);
        mountains_front.push([
            last_x + gap,                                 // x
            -1.0 + grass_h + mount_h * mount_front_scale, // y
        ]);

        // exit if this should be the last mountain to create
        if last_x + gap + (mount_w * mount_front_scale) > 1.0 {
            break;
        }
    }

    // trees
    let pine_tex = frug_instance.load_texture(include_bytes!("img/pine.png"));
    let pine_front_tex = frug_instance.load_texture(include_bytes!("img/pine_front.png"));
    let pine_w = 140.0 / window_w;
    let pine_h = 270.0 / window_h;
    let pine_scale = (0.9, 1.2);
    let pine_gap = (0.005, 0.05);
    //      back forest
    let back_forest_spd = 0.5;
    let mut back_forest: Vec<[f32; 3]> = Vec::new();
    {
        let scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
        back_forest.push([
            -1.3 + rand::thread_rng().gen_range(pine_gap.0..pine_gap.1), // x
            -1.0 + grass_h + pine_h * scale,                             // y
            scale,                                                       // scale
        ]);
    }
    loop {
        // create pines until full screen has pines
        let last_pine = back_forest[back_forest.len() - 1];
        let last_x = last_pine[0] + pine_w * last_pine[2];
        let gap = rand::thread_rng().gen_range(pine_gap.0..pine_gap.1);
        let scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
        back_forest.push([
            last_x + gap,                    // x
            -1.0 + grass_h + pine_h * scale, // y
            scale,
        ]);

        // exit if this should be the last mountain to create
        if last_x + gap + (pine_w * scale) > 1.0 {
            break;
        }
    }
    //      front forest
    let front_forest_spd = 0.7;
    let mut front_forest: Vec<[f32; 3]> = Vec::new();
    {
        let scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
        front_forest.push([
            -1.1 + rand::thread_rng().gen_range(pine_gap.0..pine_gap.1), // x
            -1.0 + grass_h + pine_h * scale,                             // y
            scale,                                                       // scale
        ]);
    }
    loop {
        // create pines until full screen has pines
        let last_pine = front_forest[front_forest.len() - 1];
        let last_x = last_pine[0] + pine_w * last_pine[2];
        let gap = rand::thread_rng().gen_range(pine_gap.0..pine_gap.1);
        let scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
        front_forest.push([
            last_x + gap,                    // x
            -1.0 + grass_h + pine_h * scale, // y
            scale,
        ]);

        // exit if this should be the last mountain to create
        if last_x + gap + (pine_w * scale) > 1.0 {
            break;
        }
    }

    // indices to delete in vectors
    let mut indices_to_delete: Vec<usize> = Vec::new();

    // slides content
    let mut slides_data = [
        vec![],
        vec![
            SlideObj {
                // title
                x: -(1255.0 / window_w * 1.5) / 2.0,
                y: 0.85,
                w: 1255.0 / window_w,
                h: 138.0 / window_h,
                scale: 1.5,
                tex_idx: frug_instance.load_texture(include_bytes!("img/slide_titles/1.png")),
            },
            SlideObj {
                // gamepad
                x: -0.5 - (320.0 / window_w * 1.15) / 2.0,
                y: 0.3,
                w: 320.0 / window_w,
                h: 180.0 / window_h,
                scale: 1.15,
                tex_idx: frug_instance.load_texture(include_bytes!("img/gamepad.png")),
            },
            SlideObj {
                // rust crab
                x: (256.0 / window_w * 1.75) / -2.0,
                y: 0.4,
                w: 256.0 / window_w,
                h: 256.0 / window_h,
                scale: 1.75,
                tex_idx: frug_instance.load_texture(include_bytes!("img/rust_crab.png")),
            },
            SlideObj {
                // docs
                x: 0.5 - (340.0 / window_w * 0.9) / 2.0,
                y: 0.4,
                w: 340.0 / window_w,
                h: 400.0 / window_h,
                scale: 0.9,
                tex_idx: frug_instance.load_texture(include_bytes!("img/docs.png")),
            },
        ],
        vec![
            SlideObj {
                // title
                x: -(1445.0 / window_w * 1.3) / 2.0,
                y: 0.85,
                w: 1445.0 / window_w,
                h: 138.0 / window_h,
                scale: 1.3,
                tex_idx: frug_instance.load_texture(include_bytes!("img/slide_titles/2.png")),
            },
            SlideObj {
                // chart - popularity
                x: -0.5 - (300.0 / window_w * 1.0) / 2.0,
                y: 0.5,
                w: 300.0 / window_w,
                h: 400.0 / window_h,
                scale: 1.0,
                tex_idx: frug_instance.load_texture(include_bytes!("img/chart.png")),
            },
            SlideObj {
                // trophy - sdl (industry standard)
                x: -(500.0 / window_w * 0.7) / 2.0,
                y: 0.55,
                w: 500.0 / window_w,
                h: 640.0 / window_h,
                scale: 0.7,
                tex_idx: frug_instance.load_texture(include_bytes!("img/sdl.png")),
            },
            SlideObj {
                // bevy - rust state of the art
                x: 0.5 - (600.0 / window_w * 0.9) / 2.0,
                y: 0.55,
                w: 600.0 / window_w,
                h: 440.0 / window_h,
                scale: 0.9,
                tex_idx: frug_instance.load_texture(include_bytes!("img/bevy.png")),
            },
        ],
        vec![
            SlideObj {
                // title
                x: -(1445.0 / window_w * 1.0) / 2.0,
                y: 0.85,
                w: 1410.0 / window_w,
                h: 200.0 / window_h,
                scale: 1.0,
                tex_idx: frug_instance.load_texture(include_bytes!("img/slide_titles/3.png")),
            },
            SlideObj {
                // bad documentation
                x: -0.333 - (170.0 / window_w * 1.5) / 2.0,
                y: 0.35,
                w: 170.0 / window_w,
                h: 200.0 / window_h,
                scale: 1.5,
                tex_idx: frug_instance.load_texture(include_bytes!("img/bad_docs.png")),
            },
            SlideObj {
                // abstraction extremes
                x: 0.333 - (738.0 / window_w * 0.6) / 2.0,
                y: 0.45,
                w: 738.0 / window_w,
                h: 793.0 / window_h,
                scale: 0.6,
                tex_idx: frug_instance.load_texture(include_bytes!("img/abstraction_usable.png")),
            },
        ],
        vec![
            SlideObj {
                // title
                x: -(1389.0 / window_w * 1.2) / 2.0,
                y: 0.85,
                w: 1389.0 / window_w,
                h: 224.0 / window_h,
                scale: 1.2,
                tex_idx: frug_instance.load_texture(include_bytes!("img/slide_titles/4.png")),
            },
            SlideObj {
                // gpu
                x: -0.575 - (400.0 / window_w * 0.9) / 2.0,
                y: 0.33,
                w: 400.0 / window_w,
                h: 330.0 / window_h,
                scale: 0.9,
                tex_idx: frug_instance.load_texture(include_bytes!("img/gpu.png")),
            },
            SlideObj {
                // +
                x: -0.25 - (180.0 / window_w * 1.2) / 2.0,
                y: 0.25,
                w: 180.0 / window_w,
                h: 180.0 / window_h,
                scale: 1.2,
                tex_idx: frug_instance.load_texture(include_bytes!("img/add.png")),
            },
            SlideObj {
                // good docs
                x: 0.0 - (170.0 / window_w * 1.4) / 2.0,
                y: 0.3,
                w: 170.0 / window_w,
                h: 200.0 / window_h,
                scale: 1.4,
                tex_idx: frug_instance.load_texture(include_bytes!("img/good_docs.png")),
            },
            SlideObj {
                // =
                x: 0.27 - (180.0 / window_w * 1.2) / 2.0,
                y: 0.25,
                w: 180.0 / window_w,
                h: 180.0 / window_h,
                scale: 1.2,
                tex_idx: frug_instance.load_texture(include_bytes!("img/equal.png")),
            },
            SlideObj {
                // frug
                x: 0.56 - (192.0 / window_w * 1.9) / 2.0,
                y: 0.37,
                w: 192.0 / window_w,
                h: 192.0 / window_h,
                scale: 1.9,
                tex_idx: frug_instance.load_texture(include_bytes!("img/frog_idle/1.png")),
            },
        ],
        vec![
            SlideObj {
                // title
                x: -(1577.0 / window_w * 1.0) / 2.0,
                y: 0.85,
                w: 1577.0 / window_w,
                h: 224.0 / window_h,
                scale: 1.0,
                tex_idx: frug_instance.load_texture(include_bytes!("img/slide_titles/5.png")),
            },
            SlideObj {
                // docs screenshot
                x: -0.35 - (1610.0 / window_w * 0.5) / 2.0,
                y: 0.45,
                w: 1610.0 / window_w,
                h: 1332.0 / window_h,
                scale: 0.5,
                tex_idx: frug_instance.load_texture(include_bytes!("img/frug_docs_ss.png")),
            },
            SlideObj {
                // docs qr
                x: 0.45 - (1023.0 / window_w * 0.6) / 2.0,
                y: 0.45,
                w: 1023.0 / window_w,
                h: 1023.0 / window_h,
                scale: 0.6,
                tex_idx: frug_instance.load_texture(include_bytes!("img/frug_qr.png")),
            },
        ],
    ];
    /*let mut slides_content: Vec<Vec<SlideObj>> = Vec::new();
    slides_content.push(vec![]);*/
    let slide_movement_duration = 2.0;
    let mut slide_movement_left = 0.0;
    //      update x values of slides' data with offset
    for i in 0..slides_data.len() {
        for item in slides_data[i].iter_mut() {
            item.x += slide_movement_duration * i as f32;
        }
    }

    // ============= UPDATE FUNCTION =============
    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        // ****     LOGIC   ****

        // update transitions
        if transition == Transition::Outro {
            transition_height -= transition_speed;
            if transition_height <= 0.0 {
                transition = Transition::None;
                if slide == 0 {
                    slide = 1;
                }
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
        // set frogo walk animation or idle based on transition speed
        if frogo_is_idle && slide_transition_speed != 0.0 {
            frogo_is_idle = false;
            frogo_tex_loc = 0;
            frogo_tex_idx = frogo_walk[frogo_tex_loc];
            frogo_tex_timer = frogo_walk_spd;
        } else if !frogo_is_idle && slide_transition_speed == 0.0 {
            frogo_is_idle = true;
            frogo_tex_loc = 0;
            frogo_tex_idx = frogo_idle[frogo_tex_loc];
            frogo_tex_timer = frogo_idle_spd;
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
        for i in (0..clouds_to_delete.len()).rev() {
            clouds_data.remove(clouds_to_delete[i]);
        }
        clouds_to_delete.clear();

        // update grass data
        //      move grass & check if should delete them
        for i in 0..grass_tiles.len() {
            grass_tiles[i][0] -= slide_transition_speed * grass_spd;

            // check for deletion
            if slide_transition_speed > 0.0 {
                // to delete left
                if grass_tiles[i][0] + grass_w * grass_scale < -1.0 {
                    indices_to_delete.push(i);
                }
            } else if slide_transition_speed < 0.0 {
                // to delete right
                if grass_tiles[i][0] + grass_w * grass_scale > 1.0 {
                    indices_to_delete.push(i);
                }
            }
        }
        //      create new grass tiles if moving
        if slide_in_transition {
            if slide_transition_speed > 0.0 {
                // create on right if last tile not equal or beyond border
                if grass_tiles[grass_tiles.len() - 1][0] + grass_w < 1.0 {
                    grass_tiles.push([
                        grass_tiles[grass_tiles.len() - 1][0] + grass_w, // x
                        grass_tiles[grass_tiles.len() - 1][1],           // y
                        ((grass_tiles[grass_tiles.len() - 1][2] as i32 + 1) % 2) as f32, // flip x
                    ]);
                }
            }
        }
        //      delete grass tiles
        for i in indices_to_delete.iter().rev() {
            grass_tiles.remove(*i);
        }
        indices_to_delete.clear();

        // update front forest data
        //      move trees & check if should delete them
        for i in 0..front_forest.len() {
            front_forest[i][0] -= slide_transition_speed * front_forest_spd;

            // check for deletion
            if slide_transition_speed > 0.0 {
                // to delete left
                if front_forest[i][0] + pine_w * front_forest[i][2] < -1.0 {
                    indices_to_delete.push(i);
                }
            } else if slide_transition_speed < 0.0 {
                // to delete right
                if front_forest[i][0] + pine_w * front_forest[i][2] > 1.0 {
                    indices_to_delete.push(i);
                }
            }
        }
        //      create new trees if moving
        if slide_in_transition {
            if slide_transition_speed > 0.0 {
                // create on right if last tile not equal or beyond border
                let last_pine_right = front_forest[front_forest.len() - 1][0]
                    + pine_w * front_forest[front_forest.len() - 1][2];

                if last_pine_right < 1.0 {
                    let new_scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
                    let new_x =
                        last_pine_right + rand::thread_rng().gen_range(pine_gap.0..pine_gap.1);
                    let new_y = -1.0 + grass_h + pine_h * new_scale;

                    front_forest.push([
                        new_x,     // x
                        new_y,     // y
                        new_scale, // scale
                    ]);
                }
            }
        }
        //      delete trees
        for i in indices_to_delete.iter().rev() {
            front_forest.remove(*i);
        }
        indices_to_delete.clear();

        // update back forest data
        //      move trees & check if should delete them
        for i in 0..back_forest.len() {
            back_forest[i][0] -= slide_transition_speed * back_forest_spd;

            // check for deletion
            if slide_transition_speed > 0.0 {
                // to delete left
                if back_forest[i][0] + pine_w * back_forest[i][2] < -1.0 {
                    indices_to_delete.push(i);
                }
            } else if slide_transition_speed < 0.0 {
                // to delete right
                if back_forest[i][0] + pine_w * back_forest[i][2] > 1.0 {
                    indices_to_delete.push(i);
                }
            }
        }
        //      create new trees if moving
        if slide_in_transition {
            if slide_transition_speed > 0.0 {
                // create on right if last tile not equal or beyond border
                let last_pine_right = back_forest[back_forest.len() - 1][0]
                    + pine_w * back_forest[back_forest.len() - 1][2];

                if last_pine_right < 1.0 {
                    let new_scale = rand::thread_rng().gen_range(pine_scale.0..pine_scale.1);
                    let new_x =
                        last_pine_right + rand::thread_rng().gen_range(pine_gap.0..pine_gap.1);
                    let new_y = -1.0 + grass_h + pine_h * new_scale;

                    back_forest.push([
                        new_x,     // x
                        new_y,     // y
                        new_scale, // scale
                    ]);
                }
            }
        }
        //      delete trees
        for i in indices_to_delete.iter().rev() {
            back_forest.remove(*i);
        }
        indices_to_delete.clear();

        // update front mountains data
        //      move mountains & check if should delete them
        for i in 0..mountains_front.len() {
            mountains_front[i][0] -= slide_transition_speed * mount_front_spd;

            // check for deletion
            if slide_transition_speed > 0.0 {
                // to delete left
                if mountains_front[i][0] + mount_w * mount_front_scale < -1.0 {
                    indices_to_delete.push(i);
                }
            } else if slide_transition_speed < 0.0 {
                // to delete right
                if mountains_front[i][0] > 1.0 {
                    indices_to_delete.push(i);
                }
            }
        }
        //      create new trees if moving
        if slide_in_transition {
            if slide_transition_speed > 0.0 {
                // create on right if last tile not equal or beyond border
                let last_mount_right =
                    mountains_front[mountains_front.len() - 1][0] + mount_w * mount_front_scale;

                if last_mount_right < 1.0 {
                    let new_x =
                        last_mount_right + rand::thread_rng().gen_range(gap_rng.0..gap_rng.1);

                    mountains_front.push([
                        new_x,                                        // x
                        -1.0 + grass_h + mount_h * mount_front_scale, // y
                    ]);
                }
            }
        }
        //      delete trees
        for i in indices_to_delete.iter().rev() {
            mountains_front.remove(*i);
        }
        indices_to_delete.clear();

        // update back mountains data
        //      move mountains & check if should delete them
        for i in 0..mountains_back.len() {
            mountains_back[i][0] -= slide_transition_speed * mount_back_spd;

            // check for deletion
            if slide_transition_speed > 0.0 {
                // to delete left
                if mountains_back[i][0] + mount_w * mount_back_scale < -1.0 {
                    indices_to_delete.push(i);
                }
            } else if slide_transition_speed < 0.0 {
                // to delete right
                if mountains_back[i][0] > 1.0 {
                    indices_to_delete.push(i);
                }
            }
        }
        //      create new trees if moving
        if slide_in_transition {
            if slide_transition_speed > 0.0 {
                // create on right if last tile not equal or beyond border
                let last_mount_right =
                    mountains_back[mountains_back.len() - 1][0] + mount_w * mount_back_scale;

                if last_mount_right < 1.0 {
                    let new_x =
                        last_mount_right + rand::thread_rng().gen_range(gap_rng.0..gap_rng.1);

                    mountains_back.push([
                        new_x,                                       // x
                        -1.0 + grass_h + mount_h * mount_back_scale, // y
                    ]);
                }
            }
        }
        //      delete trees
        for i in indices_to_delete.iter().rev() {
            mountains_back.remove(*i);
        }
        indices_to_delete.clear();

        // updates slides data position
        for slide in slides_data.iter_mut() {
            for item in slide.iter_mut() {
                item.x -= slide_transition_speed;
            }
        }

        // stop slide transition
        if slide_in_transition {
            slide_movement_left -= slide_speed;
            if slide_movement_left <= 0.0 {
                slide_in_transition = false;
                slide_transition_speed = 0.0;
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
            } else if !slide_in_transition {
                // move to next slide only if slide is static
                slide_transition_speed = slide_speed;
                slide_in_transition = true;
                slide_movement_left = slide_movement_duration;
                println!("moving slide");
            }
        }

        // ****     RENDER  ****
        instance.clear();

        // render if not in full transition
        if transition != Transition::Full {
            // render background
            instance.add_colored_vertices(&background, &background_indices);

            // render mountains
            //      back mountains
            for mount in mountains_back.iter() {
                instance.add_tex_rect(
                    mount[0],
                    mount[1],
                    mount_w * mount_back_scale,
                    mount_h * mount_back_scale,
                    mount2_tex_idx,
                    false,
                    false,
                );
            }
            //      front mountains
            for mount in mountains_front.iter() {
                instance.add_tex_rect(
                    mount[0],
                    mount[1],
                    mount_w * mount_front_scale,
                    mount_h * mount_front_scale,
                    mount_tex_idx,
                    false,
                    false,
                );
            }

            // forests
            for pine in back_forest.iter() {
                instance.add_tex_rect(
                    pine[0],
                    pine[1],
                    pine_w * pine[2],
                    pine_h * pine[2],
                    pine_tex,
                    false,
                    false,
                );
            }
            for pine in front_forest.iter() {
                instance.add_tex_rect(
                    pine[0],
                    pine[1],
                    pine_w * pine[2],
                    pine_h * pine[2],
                    pine_front_tex,
                    false,
                    false,
                );
            }

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

            // render slide content (if within screen)
            for slide in slides_data.iter() {
                for item in slide.iter() {
                    if item.x + item.w * item.scale >= -1.0 && item.x < 1.0 {
                        if item.y - item.h * item.scale <= 1.0 && item.y > -1.0 {
                            instance.add_tex_rect(
                                item.x,
                                item.y,
                                item.w * item.scale,
                                item.h * item.scale,
                                item.tex_idx,
                                false,
                                false,
                            )
                        }
                    }
                }
            }

            // render grass
            for tile in grass_tiles.iter() {
                instance.add_tex_rect(
                    tile[0],
                    tile[1],
                    grass_w,
                    grass_h,
                    grass_tex_idx,
                    tile[2] == 1.0,
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
