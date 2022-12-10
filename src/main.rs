use nannou::prelude::*;
use rayon::prelude::*;
use std::time::Instant;
// use std::iter::*;

const GRID_WIDTH: usize = 500;
const GRID_HEIGHT: usize = 500;
const PTS_PER_W: f32 = 1.0;
const PTS_PER_H: f32 = 1.0;
const FRAME_COUNT_MAX: f64 = 60.0;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn randomize(&mut self) {
        self.alive = random();
    }

    fn set_alive(&mut self) {
        self.alive = true;
    }

    // fn set_dead(&mut self) {
    //     self.alive = false;
    // }
}

struct Model {
    grid: Vec<Cell>,
    time_counter: Instant,
    frame_counter: f64,
    frame_time: f64,
    print_frame_time: bool,
}

fn model(app: &App) -> Model {
    let mut grid = vec![Cell { alive: false }; GRID_WIDTH * GRID_HEIGHT];

    grid.par_iter_mut().for_each(|cell| cell.randomize());

    // for cell in grid.iter_mut() {
    //     cell.randomize();
    // }

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
    // app.set_loop_mode(LoopMode::rate_fps(10.0));
    Model {
        grid,
        time_counter: Instant::now(),
        frame_counter: 0.0,
        frame_time: 0.0,
        print_frame_time: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.frame_counter >= FRAME_COUNT_MAX {
        model.frame_time = model.time_counter.elapsed().as_secs_f64() / FRAME_COUNT_MAX;
        model.frame_counter = 0.0;
        model.print_frame_time = true;
        model.time_counter = Instant::now();
    } else {
        model.print_frame_time = false;
        model.frame_counter += 1.0;
    }

    // let mut next_grid = model.grid.clone();
    let mut next_grid = vec![Cell { alive: false }; GRID_WIDTH * GRID_HEIGHT];

    next_grid
        .par_iter_mut()
        .zip(0..GRID_WIDTH * GRID_HEIGHT)
        .for_each(|(cell, index)| {
            let (x, y) = index_to_xy(index);
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
                                model.grid[x_y_to_index(newx as usize, newy as usize)].alive as i32;
                        }
                    }
                    // println!("newx = {newx}, newy = {newy}");
                }
            }
            if model.grid[index].alive {
                if neighbour_count == 2 || neighbour_count == 3 {
                    cell.set_alive();
                }
            } else if neighbour_count == 3 {
                cell.set_alive();
            }
        });

    model.grid = next_grid;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let draw = app.draw();
    draw.background().color(BLACK);
    let win_half_w = 0.5 * window.rect().w();
    let win_half_h = 0.5 * window.rect().h();

    if model.print_frame_time {
        println!("{}", 1.0 / model.frame_time);
    }

    model
        .grid
        .iter()
        .zip(0..GRID_WIDTH * GRID_HEIGHT)
        .for_each(|(cell, index)| {
            if cell.alive {
                let (x, y) = index_to_xy(index);
                let xpt = (PTS_PER_W * x as f32) - win_half_w + 0.5 * PTS_PER_W;
                let ypt = (PTS_PER_H * y as f32) - win_half_h + 0.5 * PTS_PER_H;
                draw.rect()
                    .x_y(xpt, ypt)
                    .w_h(PTS_PER_W, PTS_PER_H)
                    .color(WHITE);
                // println!("x = {x}, xpt = {xpt}\ny = {y}, ypt = {ypt}\n");
            }
        });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn x_y_to_index(x: usize, y: usize) -> usize {
    let index = y * GRID_WIDTH + x;
    return index;
}

fn index_to_xy(i: usize) -> (usize, usize) {
    let x = i % GRID_WIDTH;
    let y = i / GRID_WIDTH;
    (x, y)
}
