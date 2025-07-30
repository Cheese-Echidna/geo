#[macro_use]
extern crate alloc;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::*;
use std::hash::Hasher;

use nannou::color::*;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou_egui::{self, egui, Egui};
use nannou_egui::egui::Ui;
use noise::{NoiseFn, OpenSimplex};
use palette;
use palette::Clamp;
use palette::convert::FromColorUnclamped;

use crate::centroid::centroid;
use crate::delaunay::{voronoi_diagram, Point64};
use crate::kruskals::kruskals_mst;
use crate::point::*;
use crate::settings::*;
use crate::utils::*;

mod delaunay;
mod kruskals;
mod point;
mod centroid;
mod nearest;
mod settings;
mod renderers;
mod utils;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const POINTS_SQUARE_WIDTH_PX: usize = 800;
const POINTS_SQUARE_WIDTH_POINTS: usize = 25;
const BACKGROUND: Srgb<u8> = BLACK;

fn main() {
    nannou::app(model)
        .update(update)
        .fullscreen()
        .run();
}

struct Model {
    _window: WindowId,
    bg: Srgb<u8>,
    points: Vec<Point>,
    settings: Settings,
    gui: Egui,
}


fn draw(app: &App, model: &Model, frame: Frame) {
    frame.clear(model.bg);
    let draw = app.draw();
    let render_mode_name = match model.settings.render_mode {
        0 => { renderers::perlin::render_perlin(app, model, &draw); "Perlin Vector Field" }
        1 => { renderers::voronoi::render_voronoi(app, model, &draw); "Voronoi Diagram" }
        2 => { renderers::voronoi::render_delaunay(app, model, &draw); "Delaunay Triangulation" }
        3 => { renderers::graph::render_mst(app, model, &draw); "Minimum Spanning Team" }
        4 => { renderers::graph::render_bfs(app, model, &draw); "Breadth First Search" }
        5 => { renderers::points::render_speed_sizing(app, model, &draw); "Speed Sizing" }
        6 => { renderers::points::render_point_vectors(app, model, &draw); "Movement Vectors" }
        7 => { renderers::voronoi::render_bubbles(app, model, &draw); "Bubbles" }
        9 => { renderers::points::render_point_vectors_coloured(app, model, &draw); "Perlin Vector Field with Arrows" }
        _ => {
            frame.clear(RED);
            return;
        }
    };

    draw_progress_bar(app, &draw, model);
    draw_title(app, &draw, render_mode_name);

    draw.to_frame(app, &frame).unwrap();
    model.gui.draw_to_frame(&frame).unwrap();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .title("nannou geometric")
        .view(draw)
        .event(event)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        _window: window_id,
        bg: BACKGROUND,
        points: Point::new_points(&app),
        settings: Settings::default(),
        gui: egui,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if model.settings.simulation_speed.bool {
        // sets last point for velocity calculation
        model.points.iter_mut().for_each(|x| {
            x.last_pos = x.pos;
        });

        // first we are going to calculate the voronoi diagram
        // then the centroid
        // then push the points toward the centroid
        if model.settings.centroid_push.bool {
            let voronoi = voronoi_diagram(app, model);
            for (i, (points, _)) in voronoi.iter().enumerate() {
                match centroid(&points) {
                    None => {}
                    Some(centroid) => {
                        let point = model.points[i].pos;
                        let mut push_vector = centroid - point;

                        let push_magnitude = 1.0 / 20.0 * model.settings.centroid_push.value_f32() * model.settings.simulation_speed.value_f32();

                        push_vector *= push_magnitude;
                        model.points[i].pos += push_vector
                    }
                }
            }
        }
        // println!("Errors after centroid push: {}", model.count_non_finite_points());

        if model.settings.timer_pull.bool {
            let max_time = model.settings.timer_pull.value_f32();

            let x = ((app.time % max_time) / max_time + 0.5) % 1.0;
            let mu = 0.5;
            let std = 3.0 / max_time;
            let y = E.powf(-0.5 * ((x - mu) / std).powi(2));

            let strength = y * model.settings.simulation_speed.value_f32();

            for point in model.points.iter_mut() {
                let mut points = vec![];

                let width_height = app.window_rect().wh();
                let width = Vec2::new(width_height.x, 0.0);
                let height = Vec2::new(0.0, width_height.y);

                let starting_location = point.starting_location;

                points.push(starting_location);
                points.push(starting_location + width + height);
                points.push(starting_location + width - height);
                points.push(starting_location + width);
                points.push(starting_location + height);
                points.push(starting_location - width + height);
                points.push(starting_location - width - height);
                points.push(starting_location - width);
                points.push(starting_location - height);

                points.sort_by(|x, y| point.pos.distance(*x).partial_cmp(&point.pos.distance(*y)).unwrap_or(Ordering::Equal));

                let mut vec = points[0] - point.pos;

                vec *= 1.0 / 100.;
                vec *= strength;
                point.pos += vec;
            }
        }
        // println!("Errors after timer pull: {}", model.count_non_finite_points());

        // the following checks to see if it is outside the view window
        for point in model.points.iter_mut() {
            let window_rect = app.window_rect();

            if !window_rect.contains(point.pos) {
                let (x_start, x_end) = (window_rect.x.start, window_rect.x.end);
                let (y_start, y_end) = (window_rect.y.start, window_rect.y.end);
                let (width, height) = (window_rect.w(), window_rect.h());

                let (x, y) = (point.pos.x, point.pos.y);

                if x < x_start {
                    point.pos.x += width;
                } else if x > x_end {
                    point.pos.x -= width;
                }

                if y < y_start {
                    point.pos.y += height;
                } else if y > y_end {
                    point.pos.y -= height;
                }
            }
        }
        // println!("Errors after vw check: {}", model.count_non_finite_points());

        if model.settings.perlin_push.bool {
            for point in model.points.iter_mut() {
                let value = tileable_perlin(&model.settings, &app, point.pos);
                let angle = value * TAU;

                let push = Vec2::new(angle.cos(), angle.sin()) * 1.0 / 10.0;
                point.pos += push * model.settings.perlin_push.value_f32() * model.settings.simulation_speed.value_f32();
            }
        }
        // println!("Errors after perlin push: {}", model.count_non_finite_points());

        if model.settings.mouse_push.bool {
            for point in model.points.iter_mut() {
                let mouse = Vec2::new(app.mouse.x, app.mouse.y);
                if app.window_rect().contains(mouse) {
                    let vec = point.pos - mouse;
                    let dir = vec.normalize();
                    let distance = vec.length();

                    // let new_length = 1.0 / distance.powf(0.7);
                    let new_length = E.powf(-1.0 / 100.0 * distance);

                    let new_vec = dir * new_length;

                    point.pos += new_vec * model.settings.simulation_speed.value_f32() * model.settings.mouse_push.value_f32();
                }
            }
        }
        // println!("Errors after mouse push: {}", model.count_non_finite_points());
    }
    gui(app, model, update);
}

fn gui(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.gui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();


    egui::Window::new("Settings").show(&ctx, |ui| {
        let clicked = ui.button("Reset to spiral").clicked();
        if clicked {
            model.points = Point::new_points_circle(app);
        }

        let clicked = ui.button("Reset to grid").clicked();
        if clicked {
            model.points = Point::new_points_square(app);
        }

        let clicked = ui.button("Reset to nautilus").clicked();
        if clicked {
            model.points = Point::new_points_multi_colour_spiral(app);
        }

        let clicked = ui.button("Reset to puppy").clicked();
        if clicked {
            model.points = Point::new_points_from_image(app);
        }

        model.settings.simulation_speed.show(ui);
        model.settings.show_points.show(ui);
        model.settings.perlin_seed.show(ui);
        model.settings.perlin_push.show(ui);
        model.settings.mouse_push.show(ui);
        model.settings.centroid_push.show(ui);
        model.settings.timer_pull.show(ui);

        match &mut model.settings.settings_per_render_mode[model.settings.render_mode as usize] {
            None => {}
            Some(x) => {
                ui.add(egui::Label::new(format!("Render mode {} settings:", model.settings.render_mode)));
                x.iter_mut().for_each(|y| y.show(ui));
            }
        }
    });
}

// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(x) => {
            match x {
                VirtualKeyCode::Key1 => { model.settings.render_mode = 1 }
                VirtualKeyCode::Key2 => { model.settings.render_mode = 2 }
                VirtualKeyCode::Key3 => { model.settings.render_mode = 3 }
                VirtualKeyCode::Key4 => { model.settings.render_mode = 4 }
                VirtualKeyCode::Key5 => { model.settings.render_mode = 5 }
                VirtualKeyCode::Key6 => { model.settings.render_mode = 6 }
                VirtualKeyCode::Key7 => { model.settings.render_mode = 7 }
                VirtualKeyCode::Key8 => { model.settings.render_mode = 8 }
                VirtualKeyCode::Key9 => { model.settings.render_mode = 9 }
                VirtualKeyCode::Key0 => { model.settings.render_mode = 0 }
                _ => {}
            }
        }
        KeyReleased(_) => {}
        ReceivedCharacter(_) => {}
        MouseMoved(_) => {}
        MousePressed(_) => {}
        MouseReleased(_) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_, _) => {}
        Resized(_) => {}
        HoveredFile(_) => {}
        DroppedFile(_) => {}
        HoveredFileCancelled => {}
        Touch(_) => {}
        TouchPressure(_) => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
        Moved(_) => {}
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.gui.handle_raw_event(event);
}

fn draw_progress_bar(app: &App, draw: &Draw, model: &Model) {
    if model.settings.timer_pull.bool {
        let max_time = model.settings.timer_pull.value_f32();
        let start = app.window_rect().bottom_left();
        let width = Vec2::new(app.window_rect().w(), 0.0);
        let line = width * ((app.time % max_time) / max_time);
        draw.line()
            .start(start)
            .end(start + width)
            .weight(12.0)
            .color(BLACK);

        draw.line()
            .start(start)
            .end(start + line)
            .weight(12.0)
            .color(SKYBLUE);
    }
}

fn draw_title(app: &App, draw: &Draw, title: &str) {
    let win = app.window_rect();

    let estimated_text_width = title.len() as f32 * 10.0;
    let text_position = pt2(win.right() - estimated_text_width - 10.0, win.top() - 20.0);

    draw.text(title)
        .xy(text_position)
        .left_justify()
        .font_size(24)
        .color(WHITE);
}