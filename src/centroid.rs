use crate::*;

pub(crate) fn centroid(polygon: &Vec<Vec2>) -> Option<Vec2> {
    let len = polygon.len();
    let a = 0.5 * polygon.iter().enumerate().map(|(i, a)| {
        let b = &polygon[(i + 1) % len];
        a.x * b.y - b.x * a.y
    }).sum::<f32>();

    let c_x = polygon.iter().enumerate().map(|(i, a)| {
        let b = &polygon[(i + 1) % len];
        (a.x + b.x) * (a.x * b.y - b.x * a.y)
    }).sum::<f32>() / (6.0 * a);

    let c_y = polygon.iter().enumerate().map(|(i, a)| {
        let b = &polygon[(i + 1) % len];
        (a.y + b.y) * (a.x * b.y - b.x * a.y)
    }).sum::<f32>() / (6.0 * a);

    let p = Vec2::new(c_x, c_y);

    match vec2_is_wrong(&p) {
        true => {None}
        false => {Some(p)}
    }
}
