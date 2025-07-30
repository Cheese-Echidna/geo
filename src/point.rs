use crate::*;
use image;
use image::{GenericImageView};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Point {
    pub pos: Vec2,
    pub colour: LinSrgb<f32>,
    pub id: usize,
    pub starting_location: Vec2,
    pub last_pos: Vec2,
}


impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        transmute_f32_to_u32(self.pos.x).hash(state);
        transmute_f32_to_u32(self.pos.y).hash(state);
        transmute_f32_to_u32(self.colour.red).hash(state);
        transmute_f32_to_u32(self.colour.green).hash(state);
        transmute_f32_to_u32(self.colour.blue).hash(state);
    }
}

impl Point {
    pub(crate) fn new(pos: Vec2, colour: LinSrgb<f32>) -> Point {
        Point {
            pos: pos,
            colour: colour,
            id: random(),
            starting_location: pos,
            last_pos: pos,
        }
    }


    pub(crate) fn new_points_circle(_app: &App) -> Vec<Point> {

        let mut points = Vec::new();
        let angles = [0.25 * PI, 0.75 * PI, 1.25 * PI, 1.75 * PI];
        let max = POINTS_SQUARE_WIDTH_POINTS.pow(2);
        for i in 0..max {
            let prop = i as f32 / max as f32;
            let angle = angles[i % 4] + prop * TAU;
            let dist = prop * POINTS_SQUARE_WIDTH_PX as f32 / 2.0;
            let (x, y) = (dist * angle.cos(), dist * angle.sin());

            let point = Point::new(Vec2::new(x, y), colour_from_zero_one(prop));
            points.push(point)
        }

        points
    }

    pub(crate) fn new_points(app: &App) -> Vec<Point> {
        Point::new_points_square(app)
    }


    pub(crate) fn new_points_from_image(app: &App) -> Vec<Point> {
        let mut points = vec![];
        let max = 50;
        let wh = app.window_rect().wh();
        let image = image::io::Reader::open(r"puppy_blur.jpg").unwrap().decode().unwrap();
        let image_wh = Vec2::new(image.width() as f32, image.height() as f32);

        for y in 0..max {
            let prop_y = (y as f32 + 0.5) / max as f32;
            for x in 0..max {
                let prop_x = (x as f32 + 0.5) / max as f32;
                let pos = Vec2::new(( 0.5 - prop_x) * wh.x, (0.5 - prop_y) * wh.y);
                let image_data = image.get_pixel((prop_x * image_wh.x).round() as u32, (prop_y * image_wh.y).round() as u32);
                let colour:LinSrgb<f32> = convert(image_data);

                let point = Point::new(pos, colour);
                points.push(point);
            }
        }

        points
    }

    pub(crate) fn new_points_square(app: &App) -> Vec<Point> {
        let mut points = vec![];

        let rect = app.window_rect();
        let n_x = 30.;
        let spacing_x = rect.x.len() / n_x;
        let n_y = (rect.y.len() / spacing_x).floor();
        let spacing_y = rect.y.len() / n_y;
        let d = Vec2::new(spacing_x, spacing_y);
        let start = rect.bottom_left() + d / 2.0;
        let colour_fn = |x, y| colour_from_zero_one(((x as f32 / n_x) + y as f32) / n_y);
        for i in 0..(n_x as i32) {
            for j in 0..(n_y as i32) {
                points.push(Point::new(start + d * IVec2::new(i, j).as_f32(), colour_fn(i, j)))
            }
        }

        points
    }

    pub(crate) fn new_points_multi_colour_spiral(app: &App) -> Vec<Point> {
        let max_angle = TAU;
        let n_colours:usize = 5;
        let mut points = vec![];

        let max_radius = app.window_rect().wh().max_element() / 2.0;

        let a = max_radius / max_angle;
        for spiral_num in 0..n_colours {
            let offset_proportion = spiral_num as f32 / n_colours as f32;
            let offset_angle = TAU * offset_proportion;
            let num_points = POINTS_SQUARE_WIDTH_POINTS.pow(2) / n_colours;
            for point_num in 1..=num_points {
                let individual_spiral_proportion = (point_num as f32 - 1.0) / num_points as f32;
                let t = point_num as f32 / num_points as f32 * max_angle;
                let pos = Vec2::new(a * t * (t + offset_angle).cos(), a * t * (t + offset_angle).sin());

                // let hue = (offset_proportion * 6.0).round() / 6.0;
                let hue = offset_proportion;

                let saturation = 1.0 - individual_spiral_proportion * 0.9;
                let colour = okhsv_to_linsrgb(hue, saturation, 1.0);

                if app.window_rect().contains(pos) {
                    let point = Point::new(pos, colour);
                    points.push(point)
                }
            }
        }

        points
    }


    // pub(crate) fn new_points_heart() -> Vec<Point> {
    //     let func = |a:f32, t:f32| {
    //         let (sin, cos) = t.sin_cos();
    //         Vec2::new(a*sin.powi(3), a/SQRT_2 * (-cos.powi(3) - cos.powi(2) + 2.0 * cos + 0.5))
    //     };
    //
    //     let mut points = vec![];
    //
    //     for y in 0..POINTS_SQUARE_WIDTH_POINTS {
    //         for x in 0..POINTS_SQUARE_WIDTH_POINTS {
    //             let t = x as f32 / POINTS_SQUARE_WIDTH_POINTS as f32 * TAU;
    //             let a1 = y as f32 / POINTS_SQUARE_WIDTH_POINTS as f32;
    //             let a = a1 * POINTS_SQUARE_WIDTH_PX as f32 / 2.0;
    //
    //             let pos = func(a,t);
    //             let colour = colour_from_zero_one(a1);
    //             points.push(Point::new(pos, colour))
    //         }
    //     }
    //
    //     points
    //
    // }
}

fn convert(x: image::Rgba<u8>) -> LinSrgb {
    let c = nannou::color::Rgba::new(x.0[0], x.0[1], x.0[2], 255);
    let c2 = c.into_lin_srgba();
    c2.into()
}

pub(crate) fn cos(p0: f32) -> f32 {
    p0.cos()
}

pub(crate) fn sin(p0: f32) -> f32 {
    p0.sin()
}

impl From<Point> for Vec2 {
    fn from(value: Point) -> Self {
        Vec2::new(value.pos.x, value.pos.y)
    }
}

impl From<Point> for Point64 {
    fn from(value: Point) -> Self {
        Point64 { x: value.pos.x as f64, y: value.pos.y as f64 }
    }
}

impl From<[f32; 2]> for Point64 {
    fn from(value: [f32; 2]) -> Self {
        Self { x: value[0] as f64, y: value[1] as f64 }
    }
}

pub(crate) fn vec2_is_wrong(pos: &Vec2) -> bool {
    !(pos.x.is_finite() && pos.y.is_finite())
}

pub(crate) fn distance_to_line(center: Vec2, p1: Vec2, p2: Vec2) -> f32 {
    let num = ((p2.y - p1.y) * center.x - (p2.x - p1.x) * center.y + p2.x * p1.y - p2.y * p1.x).abs();
    let denom = ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).sqrt();
    num / denom
}