use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let rect = app.window_rect();
        let n = 5;
        let d = rect.w() / n as f32;
        for i in 0..n {
            for j in 0..n {
                let x = rect.left() + d * (i as f32 + 0.5);
                let y = rect.bottom() + d * (j as f32 + 0.5);
                draw_unit(x, y, d * 0.45, &draw);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_unit(x: f32, y: f32, r: f32, draw: &Draw) {
    draw_circle(x, y, r, DODGERBLUE, draw);
    draw_circle_with_fold(x, y, r * 0.95, DODGERBLUE, draw);
    draw_circle(x + r * 0.95, y, r * 0.5, DODGERBLUE, draw);
    draw.line()
        .start(pt2(x + r * 0.95, y))
        .end(pt2(x - r * 0.1, y + r * 0.6))
        .weight(2.0)
        .color(DODGERBLUE);
    draw_circle(x - r * 0.1, y + r * 0.6, r * 0.1, DODGERBLUE, draw);
    draw.line()
        .start(pt2(x - r * 0.1, y + r * 0.6))
        .end(pt2(x - r * 0.5, y - r * 0.4))
        .weight(2.0)
        .color(DODGERBLUE);
    draw_circle(x - r * 0.5, y - r * 0.4, r * 0.25, DODGERBLUE, draw);
    draw.line()
        .start(pt2(x - r * 0.5, y - r * 0.4))
        .end(pt2(x - r * 0.75, y + r * 0.45))
        .weight(2.0)
        .color(DODGERBLUE);
    draw_circle(x - r * 0.75, y + r * 0.45, r * 0.25, DODGERBLUE, draw);
    draw.line()
        .start(pt2(x - r * 0.75, y + r * 0.45))
        .end(pt2(x + r * 0.75, y - r * 0.45))
        .weight(2.0)
        .color(DODGERBLUE);
    draw_circle(x + r * 0.75, y - r * 0.45, r * 0.05, DODGERBLUE, draw);
}

fn draw_circle(x: f32, y: f32, radius: f32, color: Rgb<Srgb, u8>, draw: &Draw) {
    let points = (0..=360).map(|i| {
        let rad = deg_to_rad(i as f32);
        let xx = x + rad.cos() * radius;
        let yy = y + rad.sin() * radius;
        (pt2(xx, yy), color)
    });
    draw.polyline().weight(2.0).points_colored(points);
}

fn draw_circle_with_fold(x: f32, y: f32, radius: f32, color: Rgb<Srgb, u8>, draw: &Draw) {
    let points = (0..=360).map(|i| {
        if i < 270 {
            let rad = deg_to_rad(i as f32);
            let xx = x + rad.cos() * radius;
            let yy = y + rad.sin() * radius;
            (pt2(xx, yy), color)
        } else {
            let rad = deg_to_rad((450 - i) as f32); // 180 - (i - 270)
            let xx = x + radius + rad.cos() * radius;
            let yy = y - radius + rad.sin() * radius;
            (pt2(xx, yy), color)
        }
    });
    draw.polyline().weight(2.0).points_colored(points);
}
