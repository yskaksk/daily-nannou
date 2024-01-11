use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let grid_size = 1.0;
        let rect = app.window_rect();
        let perlin = Perlin::new();
        perlin.set_seed(1);
        let noise_scale = 0.002;
        for i in 0..(rect.w() as i32) {
            for j in 0..(rect.h() as i32) {
                let x = rect.left() + i as f32 * grid_size;
                let y = rect.top() - j as f32 * grid_size;
                let noise = perlin.get([x as f64 * noise_scale, y as f64 * noise_scale]);
                let g = map_range(noise, -1.0, 1.0, 0.5, 1.0);
                draw.rect()
                    .x_y(x, y)
                    .w_h(1.0, 1.0)
                    .rgb(g, g, g);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
