use nannou::{App, Draw};
use crate::*;

pub(crate) fn render_point_vectors(_app: &App, model: &Model, draw: &Draw) {
    draw_all_points(draw, model);
    model.points.iter().for_each(|point| {
        let mov_vec = point.pos - point.last_pos;
        draw.arrow()
            .points(point.pos, point.pos + mov_vec * 50.0)
            .weight(2.0)
            .color(point.colour);
    });
}

pub(crate) fn render_point_vectors_coloured(app: &App, model: &Model, draw: &Draw) {
    let boundary = app.window_rect();

    let radius: usize = 10;

    for py in ((boundary.y.start as i32 - radius as i32 * 2)..(boundary.y.end as i32 + radius as i32 * 2)).step_by(radius * 2) {
        for px in ((boundary.x.start as i32 - radius as i32 * 2)..(boundary.x.end as i32 + radius as i32 * 2)).step_by(radius * 2) {
            let start = Vec2::new(px as f32, py as f32);
            let value = tileable_perlin(&model.settings, app, start);
            let angle = value * TAU;
            let colour = colour_from_zero_one(value);


            let magnitude = radius as f32 * model.settings.perlin_push.value_f32();
            let end = start + magnitude * Vec2::new(angle.cos(), angle.sin());

            draw.arrow()
                .start(start)
                .end(end)
                .color(colour)
                .weight(model.settings.perlin_push.value_f32());
        }
    }

    draw_all_points(draw, model);
    model.points.iter().for_each(|point| {
        let mov_vec = point.pos - point.last_pos;
        draw.arrow()
            .points(point.pos, point.pos + mov_vec * 50.0)
            .weight(2.0)
            .color(colour_from_zero_one(mov_vec.angle() / TAU));
    });
}


pub(crate) fn render_speed_sizing(app: &App, model: &Model, draw: &Draw) {
    for point in model.points.iter() {
        let pos = point.pos;
        let movement = point.pos - point.last_pos;
        let movement_length = movement.length();
        let max_length = app.window_rect().wh().max_element();
        let mut dist = if movement_length > max_length / 2.0 {
            0.0
        } else {
            movement_length * 5.0 * model.settings.show_points.value_f32()
        };

        if dist < 2.0 {
            dist = 2.0
        }

        draw_double_circle(draw, pos, point.colour, dist);
    }
}