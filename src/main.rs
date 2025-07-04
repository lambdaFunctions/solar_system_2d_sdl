use std::vec;
// use std::time::Duration;
use std::{time::Duration, io::{stdout, Write}};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

mod view;
use view::background;
mod physics;
use physics::bodies::Body;
use physics::calculations::{compute_gravity, compute_acceleration};


static DT: f32 = 1.0; // 86400


fn print_debug(
    earth_gravity_force: (f32, f32), earth_acceleration: (f32, f32), earth: &Body
) {
    let mut stdout = stdout();

    println!("\rEarth gravity force: {} | {}", earth_gravity_force.0, earth_gravity_force.1);
    println!("\rEarth acceleration: {} | {}", earth_acceleration.0, earth_acceleration.1);
    println!("\rEarth velocity: {} | {}", earth.velocity[0], earth.velocity[1]);
    println!("\rEarth position X: {}", earth.position[0]);
    println!("\rEarth position Y: {}", earth.position[1]);

    // Move the cursor up 
    print!("\x1B[5A");

    // Overwrite each line
    print!("\rEarth gravity force: {} | {}", earth_gravity_force.0, earth_gravity_force.1);
    print!("\rEarth acceleration: {} | {}", earth_acceleration.0, earth_acceleration.1);
    print!("\rEarth velocity: {} | {}", earth.velocity[0], earth.velocity[1]);
    print!("\rEarth position X: {}", earth.position[0]);
    print!("\rEarth position Y: {}", earth.position[1]);

    // Flush to update terminal
    stdout.flush().unwrap();
}


fn main() {
    let screen_width: u32 = 1000;
    let screen_height: u32 = 800;

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video();
    let window = video_subsystem.expect("REASON").window(
        "Solar System", screen_width, screen_height
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas()
        .build()
        .unwrap();

    let background_view: background::Background = background::Background {
        screen_area: Rect::new(0, 0, screen_width, screen_height),
        clear_color: Color::RGB(0, 0, 0),
    };

    let mut running: bool = true;
    let mut event_queue: sdl2::EventPump = sdl_context.event_pump().unwrap();    

    let mut earth: Body = Body::new(
        // Color::RGB(255, 255, 255), 
        Color::RGB(0, 191, 255), 
        vec![250.0, 300.0],
        vec![0.0_f32, 2.5_f32],
        10.0,
        10.0 // 5.972 * 10.0_f32.powf(24.0)
    );

    let mut sun: Body = Body::new(
        Color::RGB(255, 255, 0),
        vec![500.0, 400.0],
        vec![0.0_f32, 0.0_f32],
        30.0,
        10000.0 // 1.989 * 10.0_f32.powf(30.0)
    );

    while running {
        background_view.render(&mut canvas);

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                _ => {}
            }
        }

        earth.render(&mut canvas);
        sun.render(&mut canvas);

        let earth_gravity_force: (f32, f32) = compute_gravity(&earth, &sun);
        let earth_acceleration: (f32, f32) = compute_acceleration(
            earth_gravity_force.0, earth_gravity_force.1, earth.mass
        );

        let sun_gravity_force: (f32, f32) = (-earth_gravity_force.0, -earth_gravity_force.1);
        let sun_acceleration: (f32, f32) = compute_acceleration(
            sun_gravity_force.0, sun_gravity_force.1, sun.mass
        );

        earth.change_velocity(earth_acceleration.0 * DT, earth_acceleration.1 * DT);
        earth.change_position(earth.velocity[0], earth.velocity[1]);

        sun.change_velocity(sun_acceleration.0 * DT, sun_acceleration.1 * DT);
        sun.change_position(sun.velocity[0], sun.velocity[1]);

        print_debug(earth_gravity_force, earth_acceleration, &earth);

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(56));
   }
}

