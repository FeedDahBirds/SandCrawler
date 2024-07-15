use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
}

fn model(_app: &App) -> Model {
    Model { x : -300.0 }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if (_model.x > 300.0) {
        _model.x = -300.0
    } else if (_model.x <= -300.0) {
        _model.x += 5.0;
    } else {
        _model.x += 5.0;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background().color(PLUM);
    draw.ellipse().x_y_z(_model.x,0.0,0.0).color(STEELBLUE);
    draw.to_frame(_app, &frame).unwrap();
}