use image::Rgb;
use rand::Rng;

pub struct CircleState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: i32,
    pub color: Rgb<u8>,
    pub width: u32,
    pub height: u32,
}

impl CircleState {
    pub fn new(width: u32, height: u32) -> Self {
        let radius = (height / 25) as i32;
        let vx = width as f32 / 100.0;
        let vy = height as f32 / 100.0;

        Self {
            x: 200.0,
            y: 200.0,
            vx,
            vy,
            radius,
            color: random_color(),
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x <= self.radius as f32 || self.x >= self.width as f32 - self.radius as f32 {
            self.vx *= -1.0;
            self.color = random_color();
        }

        if self.y <= self.radius as f32 || self.y >= self.height as f32 - self.radius as f32 {
            self.vy *= -1.0;
            self.color = random_color();
        }
    }
}

fn random_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    Rgb([
        rng.gen_range(80..255),
        rng.gen_range(80..255),
        rng.gen_range(80..255),
    ])
}
