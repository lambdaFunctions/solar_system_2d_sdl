use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub struct Body {
    pub color: Color,
    pub position: Vec<f32>,
    pub velocity: Vec<f32>,
    pub radius: f32,
    pub mass: f32,
}

impl Body {
    pub fn new(
        color: Color, position: Vec<f32>, velocity: Vec<f32>, radius: f32, mass: f32
    ) -> Body {

        Body {
            color: color,
            position: position,
            velocity: velocity,
            radius: radius,
            mass: mass,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let center: (f32, f32) = (self.position[0], self.position[1]);

        let (cx, cy) = center;
        canvas.set_draw_color(self.color);
    
        for degree in 0..360 {
            let rad = (degree as f64).to_radians();
            let x = cx + (rad.cos() * self.radius as f64) as f32;
            let y = cy + (rad.sin() * self.radius as f64) as f32;

            let _ = canvas.draw_point(Point::new(x as i32, y as i32));
        }

        self.fill_color(canvas, center, self.radius, self.color);
    }

    fn fill_color(
        &self,
        canvas: &mut Canvas<Window>,
        center: (f32, f32),
        radius: f32,
        color: Color,
    ) {
        let (cx, cy) = center;
        canvas.set_draw_color(color);
    
        for y in -radius as i32..=radius as i32{
            let y_pwr: f32 = y.pow(2) as f32;
            let x_span = (radius.powf(2.0) - y_pwr).sqrt();

            for x in -x_span as i32..=x_span as i32 {
                let _ = canvas.draw_point(Point::new((cx + x as f32) as i32, (cy + y as f32) as i32));
            }
        }
    }

    pub fn change_position(&mut self, x: f32, y: f32) {
        self.position[0] += x;
        self.position[1] += y;
    }

    pub fn change_velocity(&mut self, x: f32, y: f32) {
        self.velocity[0] += x;
        self.velocity[1] += y;
    }
}
