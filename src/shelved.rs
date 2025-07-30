use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, TAU};

fn render_3d(app: &App, model: &Model, draw: &Draw) {
    // let t = sin(app.time % 30.0 / 30.0 * TAU) * 0.5 + 0.5;
    // let v = 1.0 - t;

    let size = app.window_rect().wh();
    let radius = 350.0;

    let max = 150;
    let window_rect = app.window_rect();
    for y in 0..max {
        for x in 0..max {
            let prop = Vec2::new(x as f32 / max as f32, y as f32 / max as f32);

            let pos = Vec2::new((prop.x - 0.5) * window_rect.w(),(prop.y - 0.5) * window_rect.h());
            let value = tileable_perlin(&model.settings, app, pos);

            let colour = colour_from_zero_one(value);

            let magnitude = 4.0 * model.settings.perlin_push.value_f32();

            let theta = prop.x * TAU;
            let phi = prop.y * TAU;
            let r = 350.0;

            let x = r * cos(theta) * sin(phi);
            let y = r * sin(theta) * sin(phi);
            let z = r * cos(phi);

            let start_sp = Vec3::new(x, y, z);

            if start_sp.z >= 0.0 {
                draw.ellipse()
                    .radius(magnitude)
                    .xyz(start_sp)
                    .color(colour);
            }


        };
    }

    if model.settings.show_points.bool {
        model.points.iter().for_each(|point| {
            let pos = spherical_coords(point.pos, size, radius);
            let radius = model.settings.show_points.value_f32();
            draw.ellipse()
                .radius(radius)
                .color(point.colour)
                .xyz(pos);


            let pos2 = Vec3::new(pos.x, pos.y, pos.z - radius * 0.25);
            draw.ellipse()
                .radius(radius * 4.0 / 3.0)
                .xyz(pos2)
                .color(WHITE);
        });
    }


    draw.ellipse().radius(radius).color(LIGHTBLUE).xyz(Vec3::new(0.0,0.0,0.0));
}
fn render_2d(app: &App, model: &Model, draw: &Draw) {
    let size = app.window_rect().wh();
    let max_radius = 350.0;
    let t = model.settings.settings_per_render_mode[7].as_ref().unwrap()[0].value_f32().clamp(0.0,1.0);

    // if model.settings.show_points.bool {
    model.points.iter().for_each(|point| {
        let og_vec3 = Vec3::new(point.pos.x, point.pos.y, max_radius + 1.0);
        let pos = vec3_lerp(t, og_vec3, spherical_coords(point.pos, size, max_radius));
        draw.ellipse()
            .xyz(pos)
            .color(point.colour)
            .radius(model.settings.show_points.value_f32());
    });

    draw.ellipse()
        .color(LIGHTBLUE)
        .radius(max_radius);

}
pub(crate) fn dfs(edges: Vec<(Point, Point)>) -> (Vec<(Point, Point)>, Vec<(Point, usize)>) {
    // Convert edge list to adjacency list
    let mut graph = HashMap::new();
    for (a, b) in &edges {
        graph.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        graph.entry(b.clone()).or_insert_with(Vec::new).push(a.clone());
    }

    let mut stack = Vec::new();
    let mut visited = HashMap::new();
    let mut tree_edges = Vec::new();
    let mut distances = Vec::new();

    // Assume the first point in the edge list is the starting node
    if let Some((start, _)) = edges.first() {
        stack.push(start.clone());
        visited.insert(start.clone(), true);
        distances.push((start.clone(), 0));
    }

    while let Some(current) = stack.pop() {
        let current_distance = distances.iter().find(|(p, _)| p == &current).unwrap().1;

        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    stack.push(neighbor.clone());
                    visited.insert(neighbor.clone(), true);
                    tree_edges.push((current.clone(), neighbor.clone()));
                    distances.push((neighbor.clone(), current_distance + 1));
                }
            }
        }
    }

    (tree_edges, distances)
}
pub(crate) fn nearest_neighbour(vec: &Vec<Point>) -> Vec<(usize, usize)> {
    let mut input = vec.iter().map(|x| x.pos).collect::<Vec<Vec2>>();
    let mut walk = vec![];

    let mut i: usize = 0;
    loop {
        let current_elem = input[i];
        input.remove(i);
        let next = match find_nearest(current_elem, &input) {
            None => { break; }
            Some(x) => { x }
        };
        walk.push((i, next));
        i = next;
    }
    walk.push((i, 0));
    walk
}
fn find_nearest(current: Vec2, others: &Vec<Vec2>) -> Option<usize> {
    let smallest = others.iter().min_by(|&&x, &&y| {
        current.distance(x).partial_cmp(&current.distance(y)).unwrap()
    })?;
    others.iter().position(|x| x == smallest)
}
impl Model {
    fn count_non_finite_points(&self) -> usize {
        self.points.iter().filter(|x| { x.is_wrong() }).count()
    }
}

fn spherical_coords(pos:Vec2, size:Vec2, r: f32) -> Vec3 {
    let theta = pos.x / size.x * TAU;
    let phi = pos.y / size.y * TAU;

    let x = r * cos(theta) * sin(phi);
    let y = r * sin(theta) * sin(phi);
    let z = r * cos(phi);

    Vec3::new(x, y, z)
}

fn circular_coords(pos:Vec2, size:Vec2, max_r: f32) -> Vec2 {
    let ang_x = (pos.x / size.x) * TAU + FRAC_PI_2;
    let prop_y = (pos.y / size.y) + 0.5;
    let r = max_r * prop_y;

    let x = r * cos(ang_x);
    let y = r * sin(ang_x);

    Vec2::new(x, y)
}
fn random_lin_srgb() -> LinSrgb<f32> {
    lin_srgb(random_range(0.0, 1.0), random_range(0.0, 1.0), random_range(0.0, 1.0))
}
fn opposite_colour(c: LinSrgb<f32>) -> LinSrgb<f32> {
    let components = (1.0 - c.red, 1.0 - c.green, 1.0 - c.blue);
    LinSrgb::from_components(components)
}
fn colour_lerp(v: f32, a: LinSrgb<f32>, b: LinSrgb<f32>) -> LinSrgb<f32> {
    let va = 1.0 - v;
    let vb = v;

    lin_srgb(a.red * va + b.red * vb, a.green * va + b.green * vb, a.blue * va + b.blue * vb)
}

fn vec2_lerp(t:f32, a:Vec2, b:Vec2) -> Vec2 {
    a * t + b * (1.0 - t)
}

fn vec3_lerp(t:f32, a:Vec3, b:Vec3) -> Vec3 {
    a * t + b * (1.0 - t)
}
pub(crate) fn is_wrong(&self) -> bool {
    !(self.pos.x.is_finite() && self.pos.y.is_finite())
}


fn render_mesh(app: &App, model: &Model, draw: &Draw) {
    let steps = 100;

    let x_steps = steps;
    let y_steps = ((app.window_rect().h() / app.window_rect().w()) * steps as f32).round() as i32;

    for y_step in 0..y_steps{
        let y = app.window_rect().y.lerp(y_step as f32 / y_steps as f32);
        for x_step in 0..x_steps {
            let x = app.window_rect().x.lerp(x_step as f32 / x_steps as f32);
            let mut pos = Vec3::new(x,y, 0.0);
            let perlin_value = tileable_perlin(&model.settings, app, pos.xy());
            pos.z = perlin_value * 200.0;
            draw.ellipse()
                .xyz(pos)
                .color(RED)
                .radius(3.0);

        }
    }
}
pub(crate) fn render_double_duty(app: &App, model: &Model, draw: &Draw) {
    let voronoi = voronoi_diagram(app, model);
    let triangulation = delaunay_triangulation(app, model, true);
    for (polyline, colour) in voronoi.into_iter() {
        let mut points = polyline;
        if points.len() == 0 {
            continue;
        }
        points.push(points[0]);
        draw.polyline()
            .weight(3.0)
            .points(points.clone())
            .color(BLACK);
        draw.polyline()
            .weight(2.0)
            .points(points)
            .color(colour);

    }
    for (p0, p1, p2) in triangulation {
        for (start, end) in [(p0, p1), (p1, p2), (p2, p0)] {
            draw.line()
                .points(start.pos, end.pos)
                .stroke_weight(3.0)
                .color(BLACK);
            draw.line()
                .points(start.pos, end.pos)
                .stroke_weight(2.0)
                .color(average_lin_srgb(&vec![start.colour, end.colour]));

        }
    }
    model.points.iter().for_each(|point|{
        draw.ellipse()
            .color(point.colour)
            .xy(point.pos)
            .radius(2.0);
    })

}

pub(crate) fn render_raw_points(_app: &App, model: &Model, draw: &Draw) {
    draw_all_points(draw, model);
}