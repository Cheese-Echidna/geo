use crate::*;
use crate::delaunay::delaunay_triangulation;

pub(crate) fn render_mst(app: &App, model: &Model, draw: &Draw) {
    let edges = crate::delaunay::delaunay_triangulation(app, model, false).iter().flat_map(|x| {
        [(x.0, x.1), (x.1, x.2), (x.2, x.0)]
    }).collect::<Vec<(Point, Point)>>();

    let mst = kruskals_mst(edges);

    for edge in mst.into_iter() {
        draw.line()
            .weight(5.0)
            .points(edge.start.pos, edge.end.pos)
            .color(average_lin_srgb(&vec![edge.start.colour, edge.end.colour]));
    }

    draw_all_points(draw, model);
}

pub(crate) fn render_bfs(app: &App, model: &Model, draw: &Draw) {
    let edges = delaunay_triangulation(app, model, false).iter().flat_map(|x| {
        [(x.0, x.1), (x.1, x.2), (x.2, x.0)]
    }).collect::<Vec<(Point, Point)>>();

    let (edges, distances) = nearest::bfs(edges);
    for (start, end) in edges {
        draw.line()
            .points(start.pos, end.pos)
            .weight(4.0)
            .color(average_lin_srgb(&vec![start.colour, end.colour]));
    }

    if model.settings.show_points.bool {
        for (point, distance) in distances {
            draw_double_circle(draw, point.pos, point.colour, 12.0 / (distance as f32 + 1.0).powf(0.5) * model.settings.show_points.value_f32());
        }
    }
}