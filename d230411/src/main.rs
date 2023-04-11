use nannou::color::{self, encoding::Srgb};
use nannou::prelude::*;
use nannou::rand::{self, rand::seq::IteratorRandom};

fn main() {
    nannou::app(model).update(update).size(500, 500).run();
}

struct Model {
    circles: Vec<Circle>,
}

#[derive(Debug, Copy, Clone)]
struct Circle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    color: color::rgb::Rgb<Srgb, u8>,
}

impl Circle {
    fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.vy -= 0.7;
        if (self.y - self.radius * 0.5) <= -250.0 {
            self.vy = -0.7 * self.vy;
            self.y += self.vy * dt;
        }
        if (self.x + self.radius * 0.5) >= 250.0 {
            self.vx *= -1.0;
            self.x += self.vx * dt;
        }
        if (self.x - self.radius * 0.5) <= -250.0 {
            self.vx *= -1.0;
            self.x += self.vx * dt;
        }
    }
}

fn update(_: &App, model: &mut Model, frame: Update) {
    for circle in model.circles.iter_mut() {
        circle.update(frame.since_last.as_secs_f32());
    }
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    let colors = [
        BLUE,
        BROWN,
        CHOCOLATE,
        CRIMSON,
        DARKSLATEGRAY,
        HOTPINK,
        OLIVE,
        SKYBLUE,
    ];
    let mut rng = rand::thread_rng();
    let circles = Vec::from_iter((0..40).map(|_| {
        let x = rand::random_range(-250.0, 250.0);
        let y = rand::random_range(-250.0, 250.0);
        let vx = rand::random_range(-50.0, 50.0);
        let vy = rand::random_range(-50.0, 10.0);
        let radius = rand::random_range(30.0, 80.0);
        let color = colors.iter().choose(&mut rng).unwrap();
        Circle {
            x,
            y,
            vx,
            vy,
            radius,
            color: *color,
        }
    }));
    Model { circles }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for circle in model.circles.iter() {
        vague_circle(&draw, circle.radius, circle.x, circle.y, 30, circle.color);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn vague_circle(
    draw: &Draw,
    radius: f32,
    x: f32,
    y: f32,
    n: i32,
    color: color::rgb::Rgb<Srgb, u8>,
) {
    for i in 0..=n {
        let size = map_range(i, 0, n, radius, radius * 0.1);
        let alpha = color::Alpha {
            color,
            alpha: map_range(i, 0, n, 0.01, 0.8),
        };
        draw.ellipse().x_y(x, y).w_h(size, size).color(alpha);
    }
}
