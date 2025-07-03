use super::bodies::Body;


// pub const G_FORCE: f32 = 6.6743e-11;
pub const G_FORCE: f32 = 0.5; 

pub fn compute_gravity(body_1: &Body, body_2: &Body) -> (f32, f32) {
    let dx: f32 = body_2.position[0] - body_1.position[0];
    let dy: f32 = body_2.position[1] - body_1.position[1];
    let distance_sqr: f32 = dx.powf(2.0) + dy.powf(2.0);
    let distance: f32 = distance_sqr.sqrt();

    let force: f32 = G_FORCE * body_2.mass * body_1.mass / distance_sqr;
    let fx: f32 = force * dx / distance;
    let fy: f32 = force * dy / distance;

    return (fx, fy)
}

pub fn compute_acceleration(force_x: f32, force_y: f32, mass: f32) -> (f32, f32) {
    let ax: f32 = force_x / mass;
    let ay: f32 = force_y / mass;

    return (ax, ay)
}
