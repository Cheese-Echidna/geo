use crate::sketch::*;

pub (crate) fn draw_double_circle(draw: &Draw, location: Vec2, colour: LinSrgb<f32>, size: f32) {
    draw.ellipse()
        .radius(size * 1.33333)
        .color(WHITE)
        .x_y(location.x, location.y);

    draw.ellipse()
        .radius(size)
        .color(colour)
        .x_y(location.x, location.y);
}

pub (crate) fn colour_from_zero_one(x: f32) -> LinSrgb<f32> {
    let okhsv: palette::Okhsv = palette::Okhsv::new(x * 360.0, 0.8, 1.0).clamp();
    let linsrgb = palette::LinSrgb::from_color_unclamped(okhsv);
    lin_srgb(linsrgb.red, linsrgb.green, linsrgb.blue)

    // let x = hsv(x, 1.0, 1.0).into_lin_srgba();
    // lin_srgb(x.red, x.green, x.blue)
}

pub (crate) fn okhsv_to_linsrgb(h: f32, s: f32, v: f32) -> LinSrgb<f32> {
    let okhsv: palette::Okhsv = palette::Okhsv::new(h * 360.0, s, v).clamp();
    let linsrgb = palette::LinSrgb::from_color_unclamped(okhsv);
    lin_srgb(linsrgb.red, linsrgb.green, linsrgb.blue)
}

pub (crate) fn average_lin_srgb(v: &Vec<LinSrgb<f32>>) -> LinSrgb<f32> {
    let colour = v.iter()
        .fold((0.0_f32, 0.0_f32, 0.0_f32), |col, x| {
            (col.0 + x.red, col.1 + x.green, col.2 + x.blue)
        });
    let black = lin_srgb(0.0, 0.0, 0.0);
    let len = v.iter().filter(|&&x| { !(x == black) }).count() as f32;
    lin_srgb(colour.0 / len, colour.1 / len, colour.2 / len)
}

pub (crate) fn transmute_f32_to_u32(x: f32) -> u32 {
    unsafe { std::mem::transmute::<f32, u32>(x) }
}

pub (crate) fn tileable_perlin(settings: &Settings, app: &App, pos: Vec2) -> f32 {
    let seed = settings.perlin_seed.value_u32();
    let size = app.window_rect().wh();
    let scale = settings.settings_per_render_mode[0].as_ref().unwrap()[0].value_f32();

    let x_prop = pos.x / size.x + 0.5;
    let y_prop = pos.y / size.y + 0.5;

    let angle_x = x_prop * TAU;
    let angle_y = y_prop * TAU;

    let a = cos(angle_x) * scale;
    let b = sin(angle_x) * scale;
    let c = cos(angle_y) * scale;
    let d = sin(angle_y) * scale;

    let noise = OpenSimplex::new(seed);

    let v = (noise.get([a as f64, b as f64, c as f64, d as f64])) as f32 + 0.5;
    v
}

pub(crate) fn draw_all_points(draw: &Draw, model: &Model) {
    model.points.iter().for_each(|x| {
        draw_point(draw, x, model);
    })
}

pub(crate) fn draw_point(draw: &Draw, point: &Point, model: &Model) {
    let mode = model.settings.render_mode;
    if model.settings.show_points.bool || mode == 0 || mode == 6 || mode == 9 {
        draw_double_circle(draw, point.pos, point.colour, model.settings.show_points.value_f32());
    }
}