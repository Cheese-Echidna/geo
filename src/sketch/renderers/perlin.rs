use crate::sketch::*;

pub fn render_perlin(app: &App, model: &Model, draw: &Draw) {
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
}