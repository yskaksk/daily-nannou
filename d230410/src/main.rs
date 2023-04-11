use nannou::color;
use nannou::color::encoding::Srgb;
use nannou::prelude::*;
use nannou::rand;
use nannou::rand::rand::seq::IteratorRandom;

fn main() {
    nannou::app(model).update(update).size(500, 500).run();
}

struct Model {
    circles: Vec<Circle>,
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: color::rgb::Rgb<Srgb, u8>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { circles: vec![] }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for circle in model.circles.iter() {
        vague_circle(&draw, circle.radius, circle.x, circle.y, 30, circle.color);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, _: Update) {
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
    let color = colors.iter().choose(&mut rng).unwrap();
    let circle = Circle {
        x: random_range(-250.0, 250.0),
        y: random_range(-250.0, 250.0),
        radius: random_range(0.0, 100.0),
        color: *color,
    };
    model.circles.push(circle);
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
