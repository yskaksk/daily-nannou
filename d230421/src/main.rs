use nannou::color::{self, encoding::Srgb};
use nannou::prelude::*;
use nannou::rand::{self, rand::seq::IteratorRandom};

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

#[derive(Debug, Copy, Clone)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: color::rgb::Rgb<Srgb, u8>,
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let mut circles: Vec<Circle> = Vec::new();
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
        loop {
            let x = random_range(-250.0, 250.0);
            let y = random_range(-250.0, 250.0);
            let color = colors.iter().choose(&mut rng).unwrap();
            if circles.len() == 0 {
                circles.push(Circle {
                    x,
                    y,
                    radius: 50.0,
                    color: *color,
                });
                continue;
            }
            let mut min_d = 1000.0;
            for c in circles.iter() {
                let d = ((c.x - x).powi(2) + (c.y - y).powi(2)).sqrt() - c.radius;
                if d < min_d {
                    min_d = d;
                }
                if min_d < 0.0 {
                    break;
                }
            }
            if min_d < 0.0 {
                continue;
            }
            let radius = min_d.min(100.0);
            circles.push(Circle {
                x,
                y,
                radius,
                color: *color,
            });
            if circles.len() >= 300 {
                break;
            }
        }
        for circle in circles {
            vague_circle(
                &draw,
                circle.radius * 2.0,
                circle.x,
                circle.y,
                100,
                circle.color,
            );
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
