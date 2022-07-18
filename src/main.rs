use nannou::prelude::*;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;
const PTS_PER_W: f32 = 4.0;
const PTS_PER_H: f32 = 4.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: [bool; GRID_WIDTH * GRID_HEIGHT],
}

fn model(app: &App) -> Model {
    let mut grid = [false; GRID_WIDTH * GRID_HEIGHT];

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let index = x_y_to_index(x, y);
            grid[index] = random();
        }
    }

    let _window = app
        .new_window()
        .size(
            PTS_PER_W as u32 * GRID_WIDTH as u32,
            PTS_PER_H as u32 * GRID_HEIGHT as u32,
        )
        .view(view)
        .build()
        .unwrap();

    // app.set_loop_mode(LoopMode::loop_ntimes(10));
    Model { grid }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // let mut next_grid = model.grid.clone();
    let mut next_grid = [false; GRID_WIDTH * GRID_HEIGHT];

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let index = x_y_to_index(x, y);

            // count neighbours
            // println!("----------------");
            // println!("x = {x}, y = {y}");
            // println!("----------------");
            let mut neighbour_count = 0;
            let neighbours: [i32; 3] = [-1, 0, 1];
            for dx in neighbours {
                for dy in neighbours {
                    let newx: i32 = x as i32 + dx;
                    let newy: i32 = y as i32 + dy;
                    if (newx < GRID_WIDTH as i32)
                        && (newx >= 0)
                        && (newy < GRID_HEIGHT as i32)
                        && (newy >= 0)
                    {
                        if !(dx == 0 && dy == 0) {
                            neighbour_count +=
                                model.grid[x_y_to_index(newx as usize, newy as usize)] as i32;
                        }
                    }
                    // println!("newx = {newx}, newy = {newy}");
                }
            }

            if model.grid[index] {
                if neighbour_count == 2 || neighbour_count == 3 {
                    next_grid[index] = true;
                }
            } else if neighbour_count == 3 {
                next_grid[index] = true;
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
