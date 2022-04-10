use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    bg_color: rgb::Srgb<u8>,
    x: f32,
    y: f32,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        bg_color: YELLOW,
        x: 0.0,
        y: 0.0,
        radius: 100.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius < 500.0 {
        model.radius += 1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse()
        .color(model.bg_color)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
