use crate::sketch::*;
use crate::sketch::delaunay::{delaunay_triangulation, voronoi_diagram};

pub(crate) fn render_delaunay(app: &App, model: &Model, draw: &Draw) {
    for (p0, p1, p2) in delaunay_triangulation(app, model, true) {
        let colours = vec![p0.colour, p1.colour, p2.colour];
        let colour = average_lin_srgb(&colours);

        draw.polygon()
            .points([Vec2::from(p0), Vec2::from(p1), Vec2::from(p2), Vec2::from(p0)])
            .color(colour);
    }

    draw_all_points(draw, model);
}

pub(crate) fn render_voronoi(app: &App, model: &Model, draw: &Draw) {
    let voronoi = voronoi_diagram(app, model);

    for (points, colour) in voronoi.clone().into_iter() {
        draw.polygon()
            .points(points)
            .color(colour);
    };
    let cell_settings = &model.settings.settings_per_render_mode[1].as_ref().unwrap()[0];
    if cell_settings.bool {
        // border lines
        for polyline in voronoi.into_iter() {
            let mut points = polyline.0;
            if points.len() == 0 {
                continue;
            }
            points.push(points[0]);
            draw.polyline()
                .weight(cell_settings.value_f32())
                .points(points)
                .color(BLACK);
        }
    }

    draw_all_points(draw, model);
}

pub(crate) fn render_bubbles(app: &App, model: &Model, draw: &Draw) {
    let voronoi = voronoi_diagram(app, model);
    voronoi.into_iter().enumerate().for_each(|(_i, (points, c))|{
        match centroid(&points) {
            None => {}
            Some(centroid) => {
                let radius = points
                    .windows(2)
                    .map(|edge| distance_to_line(centroid, edge[0], edge[1]))
                    .fold(f32::MAX, f32::min);

                draw.ellipse()
                    .xy(centroid)
                    .color(c)
                    .radius(radius);

            }
        }
    });
    draw_all_points(draw, model);
}

