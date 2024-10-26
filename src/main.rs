use pixels::{Error, Pixels, SurfaceTexture};
use std::collections::HashSet;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use image::{GenericImageView, DynamicImage};
use rfd::FileDialog;

mod debug;
// mod interpolation;
// Import the debug module

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 576;
const MAP_N: usize = 1024;
const SCALE_FACTOR: f32 = 50.0;

struct Camera {
    x: f32,        // x position on the map
    y: f32,        // y position on the map
    height: f32,   // height of the camera
    horizon: f32,  // offset of the horizon position (looking up-down)
    zfar: f32,     // distance of the camera looking forward
    angle: f32,    // camera angle (radians, clockwise)
}

fn main() -> Result<(), Error> {
    // Initialize the event loop and window
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Voxel Landscape")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    // Open file pickers to select heightmap and colormap files
    let colormap_path = FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg", "gif"])
        .set_title("Select Colormap Image")
        .pick_file();

    let heightmap_path = FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg", "gif"])
        .set_title("Select Heightmap Image")
        .pick_file();

    // Load selected images, or fallback to default assets if not selected
    let colormap_img: DynamicImage = match colormap_path {
        Some(path) => image::open(path).expect("Failed to load colormap image"),
        None => image::open("assets/map0.color.gif").expect("Failed to load default colormap image"),
    };

    let heightmap_img: DynamicImage = match heightmap_path {
        Some(path) => image::open(path).expect("Failed to load heightmap image"),
        None => image::open("assets/map0.height.gif").expect("Failed to load default heightmap image"),
    };

    // Ensure images are the correct size
    assert_eq!(colormap_img.width(), MAP_N as u32);
    assert_eq!(colormap_img.height(), MAP_N as u32);
    assert_eq!(heightmap_img.width(), MAP_N as u32);
    assert_eq!(heightmap_img.height(), MAP_N as u32);

    // Convert images to raw pixel data
    let colormap = colormap_img.to_rgb8();
    let heightmap = heightmap_img.to_luma8();
    let colormap_data = colormap.into_raw();
    let heightmap_data = heightmap.into_raw();

    // Initialize the camera and pressed keys set
    let mut camera = Camera {
        x: 512.0,
        y: 512.0,
        height: 70.0,
        horizon: 60.0,
        zfar: 600.0,
        angle: 1.5 * PI,
    };
    let mut pressed_keys = HashSet::new();

    // Track the time to calculate `delta_time`
    let mut last_frame_time = Instant::now();
    let mut debug_timer = Instant::now();

    // Create the pixel buffer
    let surface_texture = SurfaceTexture::new(SCREEN_WIDTH, SCREEN_HEIGHT, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    // Start the event loop
    event_loop.run(move |event, _, control_flow| {
        // Set a refresh interval for smoother, consistent updates (target ~60 FPS)
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));
        window.request_redraw();

        match event {
            Event::WindowEvent { event, .. } => {
                if !handle_window_event(event, control_flow, &mut pressed_keys) {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Calculate delta time for smooth frame-independent motion
                let now = Instant::now();
                let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                last_frame_time = now;

                update(&mut camera, &pressed_keys, delta_time);
                draw(
                    pixels.frame_mut(),
                    &camera,
                    &colormap_data,
                    &heightmap_data,
                );

                // Calculate frame rate
                let frame_rate = 1.0 / delta_time;

                // Print debug info periodically to avoid slowing down the event loop
                if debug_timer.elapsed().as_secs_f32() >= 1.0 {
                    debug::print_debug_info(&camera, &pressed_keys, delta_time, frame_rate);
                    debug_timer = Instant::now();
                }

                if pixels.render().map_err(|e| eprintln!("pixels.render() failed: {:?}", e)).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
    });
}

// Handle window events and update pressed keys
fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    pressed_keys: &mut HashSet<VirtualKeyCode>,
) -> bool {
    match event {
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
            true
        }
        WindowEvent::KeyboardInput { input, .. } => {
            if let KeyboardInput {
                virtual_keycode: Some(keycode),
                state,
                ..
            } = input
            {
                match state {
                    ElementState::Pressed => {
                        pressed_keys.insert(keycode);
                    }
                    ElementState::Released => {
                        pressed_keys.remove(&keycode);
                    }
                }
                if keycode == VirtualKeyCode::Escape {
                    *control_flow = ControlFlow::Exit;
                }
            }
            true
        }
        _ => false,
    }
}

// Update camera based on pressed keys and delta time
fn update(camera: &mut Camera, pressed_keys: &HashSet<VirtualKeyCode>, delta_time: f32) {
    let movement_speed = 100.0 * delta_time;
    let rotation_speed = 1.5 * delta_time;

    if pressed_keys.contains(&VirtualKeyCode::Up) {
        camera.x += camera.angle.cos() * movement_speed;
        camera.y += camera.angle.sin() * movement_speed;
    }
    if pressed_keys.contains(&VirtualKeyCode::Down) {
        camera.x -= camera.angle.cos() * movement_speed;
        camera.y -= camera.angle.sin() * movement_speed;
    }
    if pressed_keys.contains(&VirtualKeyCode::Left) {
        camera.angle -= rotation_speed;
    }
    if pressed_keys.contains(&VirtualKeyCode::Right) {
        camera.angle += rotation_speed;
    }
    if pressed_keys.contains(&VirtualKeyCode::E) {
        camera.height += 50.0 * delta_time;
    }
    if pressed_keys.contains(&VirtualKeyCode::D) {
        camera.height -= 50.0 * delta_time;
    }
    if pressed_keys.contains(&VirtualKeyCode::S) {
        camera.horizon += 70.0 * delta_time;
    }
    if pressed_keys.contains(&VirtualKeyCode::W) {
        camera.horizon -= 70.0 * delta_time;
    }
}

// Draw the scene to the frame buffer (sky gradient and terrain rendering)
fn draw(
    frame: &mut [u8],
    camera: &Camera,
    colormap_data: &[u8],
    heightmap_data: &[u8],
) {
    let width = SCREEN_WIDTH as usize;
    let height = SCREEN_HEIGHT as usize;
    let zfar = ((camera.zfar / 2.0).round() as usize).min(300);

    // Sky gradient
    let horizon_line = camera.horizon.round() as usize;
    for y in 0..horizon_line.min(height) {
        let t = y as f32 / horizon_line as f32;
        let r = (135.0 * (1.0 - t) + 70.0 * t) as u8;
        let g = (206.0 * (1.0 - t) + 130.0 * t) as u8;
        let b = (235.0 * (1.0 - t) + 180.0 * t) as u8;

        for x in 0..width {
            let index = (y * width + x) * 4;
            frame[index..index + 4].copy_from_slice(&[r, g, b, 255]);
        }
    }

    // Terrain rendering
    let sin_angle = camera.angle.sin();
    let cos_angle = camera.angle.cos();
    let plx = cos_angle * camera.zfar + sin_angle * camera.zfar;
    let ply = sin_angle * camera.zfar - cos_angle * camera.zfar;
    let prx = cos_angle * camera.zfar - sin_angle * camera.zfar;
    let pry = sin_angle * camera.zfar + cos_angle * camera.zfar;

    for i in 0..width {
        let delta_x = (plx + (prx - plx) / SCREEN_WIDTH as f32 * i as f32) / camera.zfar;
        let delta_y = (ply + (pry - ply) / SCREEN_WIDTH as f32 * i as f32) / camera.zfar;
        let mut rx = camera.x;
        let mut ry = camera.y;
        let mut tallest_height = height as i32;

        for z in (1..zfar).step_by(2) {
            rx += delta_x * 4.0;
            ry += delta_y * 4.0;
            let map_x = ((rx as usize) & (MAP_N - 1)) as usize;
            let map_y = ((ry as usize) & (MAP_N - 1)) as usize;
            let map_offset = MAP_N * map_y + map_x;
            let heightmap_value = heightmap_data[map_offset] as f32;
            let proj_height = ((camera.height - heightmap_value) / z as f32 * SCALE_FACTOR + camera.horizon).round() as i32;

            if proj_height < tallest_height {
                let y_start = proj_height.max(0) as usize;
                let y_end = tallest_height as usize;
                for y in y_start..y_end {
                    if y >= height {
                        continue;
                    }
                    let index = (y * width + i) * 4;
                    let color_offset = map_offset * 3;
                    frame[index..index + 4].copy_from_slice(&[
                        colormap_data[color_offset],
                        colormap_data[color_offset + 1],
                        colormap_data[color_offset + 2],
                        255,
                    ]);
                }
                tallest_height = proj_height;
                if tallest_height <= 0 {
                    break;
                }
            }
        }
    }
}