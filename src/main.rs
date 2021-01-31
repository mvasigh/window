use nannou::math::{prelude::*, Angle, Rad};
use nannou::prelude::*;
use ui::color::GRAY;

const HEIGHT: f32 = 400.0;
const WIDTH: f32 = 400.0;
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

    fn update(&mut self) {
        let mut new_points = Vec::new();
        for p in self.points.iter() {
            let mut new_p = p.clone();

            // scale the point
            new_p *= 0.85;

            // rotate about center
            // x` = x cos 𝜃 - y sin 𝜃
            // y` = y cos 𝜃 + x sin 𝜃
            let angle = Rad(PI / 8.0);
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
        draw.polygon().color(BLACK).stroke_weight(2.0).stroke(WHITE).points(self.points.clone());
    }
}

struct Model {
    squares: Vec<Square>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let mut squares = Vec::new();

    let size = WIDTH / 1.5;

    squares.push(Square::new(vec![
        vec2(-size, size),
        vec2(size, size),
        vec2(size, -size),
        vec2(-size, -size),
    ]));

    Model { squares }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for square in model.squares.iter() {
        square.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
