use crate::Camera;
use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub fn print_debug_info(
    camera: &Camera,
    pressed_keys: &HashSet<VirtualKeyCode>,
    delta_time: f32,
    frame_rate: f32,
) {
    // Clear the current line in the terminal and print the debug info on the same line
    print!("\r"); // Move the cursor to the beginning of the line
    print!(
        "Camera: x={:.2}, y={:.2}, height={:.2}, horizon={:.2}, angle={:.2}, zfar={:.2} | \
         Delta Time: {:.3} | FPS: {:.2} | Pressed Keys: {:?}",
        camera.x, camera.y, camera.height, camera.horizon, camera.angle, camera.zfar,
        delta_time, frame_rate, pressed_keys
    );
    use std::io::{self, Write};
    io::stdout().flush().unwrap(); // Ensure the line is printed immediately
}