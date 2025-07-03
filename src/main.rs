// #![windows_subsystem = "windows"]

use egui_macroquad::egui;
use macroquad::prelude::*;

mod app_settings;

use app_settings::*;

const TILE_SIZE: f32 = 50.0;
const GRID_W: usize = 20;
const GRID_H: usize = 20;

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    // create grid
    let mut grid = vec![false; GRID_W * GRID_H];
    let mut focused_tile: Option<usize>;

    loop {
        camera_fixer(&mut camera, &mut zoomer);

        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if world_mpos.x < TILE_SIZE * GRID_W as f32
            && world_mpos.x >= 0.0
            && world_mpos.y < TILE_SIZE * GRID_H as f32
            && world_mpos.y >= 0.0
        {
            let x = (world_mpos.x / TILE_SIZE) as usize;
            let y = (world_mpos.y / TILE_SIZE) as usize * GRID_W;

            focused_tile = Some(x + y)
        } else {
            focused_tile = None;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if focused_tile.is_some() {
                grid[focused_tile.unwrap()] = true;
            }
        }

        // ! draw
        clear_background(BLACK);
        set_camera(&camera);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });

        if let Some(index) = focused_tile {
            let x = (index % GRID_W) as f32 * TILE_SIZE;
            let y = (index / GRID_W) as f32 * TILE_SIZE;

            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, PURPLE);
        }

        for (index, tile) in grid.iter().enumerate() {
            let x = (index % GRID_W) as f32 * TILE_SIZE;
            let y = (index / GRID_W) as f32 * TILE_SIZE;

            if *tile == true {
                draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, YELLOW);
            }
            draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLUE);
        }

        egui_macroquad::draw();

        next_frame().await
    }
}
