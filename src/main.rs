use nannou::math::{Angle, Rad};
use nannou::prelude::*;

const HEIGHT: f32 = 800.0;
const WIDTH: f32 = 800.0;
const MAX_X: f32 = WIDTH / 2.0;
const MIN_X: f32 = (WIDTH / 2.0) * -1.0;
const MAX_Y: f32 = HEIGHT / 2.0;
const MIN_Y: f32 = (HEIGHT / 2.0) * -1.0;

type Quad = Vec<Vector2>;

#[derive(Clone, Debug)]
struct Square {
    points: Quad,
}

impl Square {
    fn new(points: Quad) -> Square {
        Square { points }
    }

    fn top_left(&self) -> Vector2 {
        self.points[0]
    }

    fn bottom_right(&self) -> Vector2 {
        self.points[2]
    }

    fn update(&mut self) {
        let mut new_points = Vec::new();
        for p in self.points.iter() {
            let mut new_p = p.clone();

            // scale the point
            new_p *= 0.995;

            // rotate about center
            // x` = x cos 𝜃 - y sin 𝜃
            // y` = y cos 𝜃 + x sin 𝜃
            let angle = Rad(PI / 64.0);
            let cos = Rad::cos(angle);
            let sin = Rad::sin(angle);
            let x = new_p.x;
            let y = new_p.y;
            new_p.x = (x * cos) - (y * sin);
            new_p.y = (y * cos) + (x * sin);

            new_points.push(new_p);
        }
        self.points = new_points;
    }

    fn draw(&self, draw: &Draw) {
        draw.polygon()
            .color(rgba(0.0, 0.0, 0.0, 0.0))
            .stroke_weight(2.0)
            .stroke(WHITE)
            .points(self.points.clone());
    }
}

fn base_sq() -> Square {
    let size = WIDTH / 3.0;
    Square::new(vec![
        vec2(-size, size),
        vec2(size, size),
        vec2(size, -size),
        vec2(-size, -size),
    ])
}

struct Model {
    _window: window::Id,
    squares: Vec<Square>,
}

fn main() {
    nannou::app(model).event(event).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let mut squares = Vec::new();
    squares.push(base_sq());

    Model { _window, squares }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() < 400 {
        return;
    }

    for square in model.squares.iter_mut() {
        square.update();
    }

    if (app.elapsed_frames() % 40) == 0 {
        model.squares.push(base_sq());
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    if app.elapsed_frames() < 400 {
        draw.to_frame(app, &frame).unwrap();
        return;
    }

    draw.rect()
        .w_h(WIDTH, HEIGHT)
        .color(srgba(0.0, 0.0, 0.0, 0.15));
    for square in model.squares.iter().rev() {
        square.draw(&draw);
    }

    let win = base_sq();
    win.draw(&draw);

    let tl = win.top_left();
    let br = win.bottom_right();

    // TOP
    draw.polygon().color(BLACK).points(vec![
        vec2(MIN_X, MAX_Y),
        vec2(MAX_X, MAX_Y),
        vec2(MAX_X, tl.y),
        vec2(MIN_X, tl.y),
    ]);
    // BOTTOM
    draw.polygon().color(BLACK).points(vec![
        vec2(MIN_X, br.y),
        vec2(MAX_X, br.y),
        vec2(MAX_X, MIN_Y),
        vec2(MIN_X, MIN_Y),
    ]);
    // LEFT
    draw.polygon().color(BLACK).points(vec![
        vec2(MIN_X, MAX_Y),
        vec2(tl.x, tl.y),
        vec2(tl.x, MIN_Y),
        vec2(MIN_X, MIN_Y),
    ]);
    // RIGHT
    draw.polygon().color(BLACK).points(vec![
        vec2(br.x, MAX_Y),
        vec2(MAX_X, MAX_Y),
        vec2(MAX_X, MIN_Y),
        vec2(br.x, MIN_Y),
    ]);

    draw.to_frame(app, &frame).unwrap();
}
