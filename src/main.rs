use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.main_window();
    window.set_inner_size_points(400.0, 400.0);
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().xy(window.rect().bottom_left()).color(STEELBLUE).radius(200.0);
    draw.ellipse().xy(window.rect().top_left()).color(STEELBLUE).radius(200.0);
    draw.ellipse().xy(window.rect().bottom_right()).color(STEELBLUE).radius(200.0);
    draw.ellipse().xy(window.rect().top_right()).color(STEELBLUE).radius(200.0);
    draw.to_frame(app, &frame).unwrap();
}
