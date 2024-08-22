use egui::Color32;

pub struct Particle {
    // position of the particle x and y will store in the vector
    pos: [i32; 2],
    color: Color32,
    velocity: f64,
}

impl Particle {
    pub fn new(pos: [i32; 2], color: Color32) -> Self {
        Self {
            pos,
            color,
            velocity: 1.0,
        }
    }
}
