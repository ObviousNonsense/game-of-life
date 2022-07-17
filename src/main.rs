use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    width: usize,
    height: usize,
    // pts_per_w: f32,
    // pts_per_h: f32,
    grid: [bool; 20 * 20],
}

fn model(app: &App) -> Model {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;
    let mut grid = [false; WIDTH * HEIGHT];
    grid[1] = true;
    grid[18] = true;
    grid[381] = true;
    grid[383] = true;
    grid[398] = true;

    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    Model {
        width: WIDTH,
        height: HEIGHT,
        grid,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let draw = app.draw();
    draw.background().color(BLACK);
    let win_half_w = 0.5 * window.rect().w();
    let win_half_h = 0.5 * window.rect().h();
    let pts_per_w = window.rect().w() / (model.width as f32);
    let pts_per_h = window.rect().h() / (model.height as f32);

    for x in 0..20 {
        for y in 0..20 {
            let index = y * 20 + x;
            if model.grid[index] {
                let xpt = (pts_per_w * x as f32) - win_half_w + 0.5 * pts_per_w;
                let ypt = (pts_per_h * y as f32) - win_half_h + 0.5 * pts_per_h;
                draw.rect()
                    .x_y(xpt, ypt)
                    .w_h(pts_per_w, pts_per_h)
                    .color(WHITE);
                // println!("x = {x}, xpt = {xpt}\ny = {y}, ypt = {ypt}\n");
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
