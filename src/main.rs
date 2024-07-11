extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time;
use std::time::Duration;
use std::collections::HashMap;

use rand::Rng;

use particle::Particle;

pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sdl_experiments", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();

    // track individual frametime
    static FPS_CAP: u16 = 200;
    static MIN_FRAME_TIME: f64 = 1f64/FPS_CAP as f64; 
    let mut frame_start_time = time::SystemTime::now();
    // track fps intermittently
    static FPS_TRACKER_INTERVAL: f64 = 3f64;
    let mut intermittent_tracker = frame_start_time.clone();
    let mut n_frames: u32 = 0;

    // vector of particles on the screen
    let mut particle_map: HashMap<(i32,i32), Particle> = std::collections::HashMap::new();
    
    //remove this later~~~~~~~~~~~~~
    for i in 200..=400{    
        for j in 500..=700{
            particle_map.insert((i,j), Particle());
        }
    }
    'game_loop: loop {

        // cap fps by enforcing min frame time
        match MIN_FRAME_TIME - frame_start_time.elapsed().expect("Sys clock error.").as_secs_f64() {
            kill_time if kill_time > 0f64 => std::thread::sleep(Duration::from_secs_f64(kill_time)),
            _ => (),
        }

        // reset frame timer
        frame_start_time = time::SystemTime::now();

        // track fps intermittently
        match intermittent_tracker.elapsed().expect("Sys clock error.").as_secs_f64() {
            elapsed_time if elapsed_time < FPS_TRACKER_INTERVAL => (),
            elapsed_time => {
                println!("FPS: {}", n_frames as f64/elapsed_time);
                intermittent_tracker = frame_start_time.clone();
                n_frames = 0;
            }
        }
        n_frames += 1;

        // clear last frame
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // ~~~~~ game logic to draw new screen ~~~~~
        canvas.set_draw_color(Color::CYAN);

        // TODO: figure out how to skip this without using #[unsafe]...
        // clone the hashmap
        let mut hm_clone: HashMap<(i32,i32), Particle> = std::collections::HashMap::new();
        for (k, v) in &particle_map{
            hm_clone.insert(k.clone(), v.clone());
        }

        // iterate through the clone and update the actual
        for (point, particle) in hm_clone{

            //  check edge of screen collision
            if point.1+1<1080{
                // check straight down
                if !particle_map.contains_key(&(point.0, point.1+1)){
                    
                    particle_map.remove(&point);
                    particle_map.insert((point.0, point.1+1), particle);

                }else{
                    // gen rand direction to check first (L/R)
                    let rand_dir =
                        if rand::thread_rng().gen_bool(0.5) {
                            1
                        }else{
                            -1
                        };


                    // check down&(L/R)
                    if point.0+rand_dir<1920 && point.0+rand_dir>=0 && !particle_map.contains_key(&(point.0+rand_dir, point.1+1)){
                    
                        particle_map.remove(&point);
                        particle_map.insert((point.0+rand_dir, point.1+1), particle);

                    // check down &(R/L)
                    }else if point.0-rand_dir<1920 && point.0-rand_dir>=0 && !particle_map.contains_key(&(point.0-rand_dir, point.1+1)){
                        
                        particle_map.remove(&point);
                        particle_map.insert((point.0-rand_dir, point.1+1), particle);

                    }
                }
            }

            //canvas.draw_point(point).expect("Couldn't draw point.");

        }
        
        // check game events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                /*
                Event::MouseButtonDown {
                    timestamp, window_id, which, mouse_btn, clicks, x, y
                } => (),
                Event::MouseButtonUp {
                    timestamp, window_id, which, mouse_btn, clicks, x, y
                } => (),
                Event::MouseMotion {
                    timestamp, window_id, which, mousestate, x, y, xrel, yrel 
                } => {particle_map.insert((x, y), Particle());},
                */
                _ => (),
            }       
        }
        
        if event_pump.mouse_state().left(){
            particle_map.insert((event_pump.mouse_state().x(), event_pump.mouse_state().y()), Particle());
        }  

        // draw new frame
        canvas.present();
    }
    

}

mod particle{

    pub struct Particle();

    impl Particle{
        
        pub fn clone(&self) -> Particle{
            Particle()
        }

    }

}