use nannou::prelude::*;

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 20;
const PTS_PER_W: f32 = 40.0;
const PTS_PER_H: f32 = 40.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: [bool; GRID_WIDTH * GRID_HEIGHT],
}

fn model(app: &App) -> Model {
    let mut grid = [false; GRID_WIDTH * GRID_HEIGHT];
    grid[10] = true;
    grid[210] = true;
    // grid[18] = true;
    // grid[381] = true;
    // grid[383] = true;
    // grid[398] = true;

    let _window = app
        .new_window()
        .size(
            PTS_PER_W as u32 * GRID_WIDTH as u32,
            PTS_PER_H as u32 * GRID_HEIGHT as u32,
        )
        .view(view)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::loop_ntimes(50));
    Model { grid }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let mut next_grid = model.grid.clone();

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let index = x_y_to_index(x, y);
            // if model.grid[index] {
            //     if (y + 1) < GRID_HEIGHT {
            //         let index_p1 = x_y_to_index(x, y + 1);
            //         println!("index = {index}, index_p1 = {index_p1}, x = {x}, y = {y}");
            //         next_grid[index] = false;
            //         next_grid[index_p1] = true;
            //     }
            // }
            if model.grid[index] {
                if (x + 1) < GRID_WIDTH {
                    let index_p1 = x_y_to_index(x + 1, y);
                    println!("index = {index}, index_p1 = {index_p1}, x = {x}, y = {y}");
                    next_grid[index] = false;
                    next_grid[index_p1] = true;
                }
            }
        }
    }
    model.grid = next_grid;
}

fn view(app: &App, model: &Model, frame: Frame) {

    let window = app.main_window();
    let draw = app.draw();
    draw.background().color(BLACK);
    let win_half_w = 0.5 * window.rect().w();
    let win_half_h = 0.5 * window.rect().h();

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let index = x_y_to_index(x, y);
            if model.grid[index] {
                let xpt = (PTS_PER_W * x as f32) - win_half_w + 0.5 * PTS_PER_W;
                let ypt = (PTS_PER_H * y as f32) - win_half_h + 0.5 * PTS_PER_H;
                draw.rect()
                    .x_y(xpt, ypt)
                    .w_h(PTS_PER_W, PTS_PER_H)
                    .color(WHITE);
                // println!("x = {x}, xpt = {xpt}\ny = {y}, ypt = {ypt}\n");
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn x_y_to_index(x: usize, y: usize) -> usize {
    let index = y * GRID_WIDTH + x;
    return index;
}
