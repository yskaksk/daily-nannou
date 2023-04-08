use nannou::color;
use nannou::color::encoding::Srgb;
use nannou::prelude::*;
use nannou::rand;
use nannou::rand::rand::seq::IteratorRandom;

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    let n = 50;

    let n_col = 5;
    let radius = 500.0 / n_col as f32 * 0.9;
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
    for i in 0..n_col {
        for j in 0..n_col {
            let x = 500.0 / (2.0 * n_col as f32) + map_range(i, 0, n_col, -250.0, 250.0);
            let y = 500.0 / (2.0 * n_col as f32) + map_range(j, 0, n_col, -250.0, 250.0);
            let color = colors.iter().choose(&mut rng).unwrap();
            vague_circle(&draw, radius, x, y, n, *color);
        }
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
