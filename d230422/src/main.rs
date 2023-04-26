use nannou::noise::*;
use nannou::prelude::*;
use nannou::rand;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    circle: Circle,
    perlin: Perlin,
}

#[derive(Clone, Copy, Debug)]
struct Circle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

impl Circle {
    fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        if (self.y - self.radius * 0.5) <= -250.0 {
            self.vy *= -1.0;
            self.y += self.vy * dt;
        }
        if (self.y - self.radius * 0.5) >= 250.0 {
            self.vy *= -1.0;
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

fn model(app: &App) -> Model {
    app.new_window().size(500, 500).view(view).build().unwrap();
    let perlin = Perlin::new();
    perlin.set_seed(0);
    Model {
        circle: Circle {
            x: rand::random_range(-250.0, 250.0),
            y: rand::random_range(-250.0, 250.0),
            vx: rand::random_range(-50.0, 50.0),
            vy: rand::random_range(-50.0, 50.0),
            radius: 100.0,
        },
        perlin,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dt = 1.0 / 60.0;
    model.circle.update(dt);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKBLUE);
    let n = 50;
    let d = 500.0 / n as f32;
    let t = app.elapsed_frames() as f32 / 1000.0;
    for i in 0..n {
        for j in 0..n {
            let sx = map_range(i, 0, n, -250.0, 250.0);
            let sy = map_range(j, 0, n, -250.0, 250.0);
            let (ex, ey) = if (i + j) % 2 == 0 {
                (sx + d, sy + d)
            } else {
                (sx + d, sy - d)
            };
            let p = model.perlin.get([
                (sx / 250.0) as f64 + t as f64,
                (sy / 250.0) as f64,
                t as f64,
            ]) as f32;
            let dist2 = (model.circle.x - sx).powi(2) + (model.circle.y - sy).powi(2);
            let dist = dist2.sqrt();
            if p > 0.05 {
                draw.line()
                    .start(pt2(sx, sy))
                    .end(pt2(ex, ey))
                    .weight(3.0)
                    .color(WHITE);
            } else if dist < model.circle.radius {
                draw.line()
                    .start(pt2(sx, sy))
                    .end(pt2(ex, ey))
                    .weight(3.0)
                    .color(RED);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
