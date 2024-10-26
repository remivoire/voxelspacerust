// interpolation.rs

pub const MAP_N: usize = 1024;

// Bilinear interpolation for height map
pub fn get_height_interpolated(x: f32, y: f32, heightmap_data: &[u8]) -> f32 {
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1) % MAP_N;
    let y1 = (y0 + 1) % MAP_N;

    let h00 = heightmap_data[MAP_N * y0 + x0] as f32;
    let h10 = heightmap_data[MAP_N * y0 + x1] as f32;
    let h01 = heightmap_data[MAP_N * y1 + x0] as f32;
    let h11 = heightmap_data[MAP_N * y1 + x1] as f32;

    let tx = x - x.floor();
    let ty = y - y.floor();

    let h0 = h00 * (1.0 - tx) + h10 * tx;
    let h1 = h01 * (1.0 - tx) + h11 * tx;

    h0 * (1.0 - ty) + h1 * ty
}

// Bilinear interpolation for color map
pub fn get_color_interpolated(x: f32, y: f32, colormap_data: &[u8]) -> [u8; 3] {
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1) % MAP_N;
    let y1 = (y0 + 1) % MAP_N;

    let c00 = &colormap_data[(MAP_N * y0 + x0) * 3..(MAP_N * y0 + x0) * 3 + 3];
    let c10 = &colormap_data[(MAP_N * y0 + x1) * 3..(MAP_N * y0 + x1) * 3 + 3];
    let c01 = &colormap_data[(MAP_N * y1 + x0) * 3..(MAP_N * y1 + x0) * 3 + 3];
    let c11 = &colormap_data[(MAP_N * y1 + x1) * 3..(MAP_N * y1 + x1) * 3 + 3];

    let tx = x - x.floor();
    let ty = y - y.floor();

    let c0 = [
        (c00[0] as f32 * (1.0 - tx) + c10[0] as f32 * tx) as u8,
        (c00[1] as f32 * (1.0 - tx) + c10[1] as f32 * tx) as u8,
        (c00[2] as f32 * (1.0 - tx) + c10[2] as f32 * tx) as u8,
    ];

    let c1 = [
        (c01[0] as f32 * (1.0 - tx) + c11[0] as f32 * tx) as u8,
        (c01[1] as f32 * (1.0 - tx) + c11[1] as f32 * tx) as u8,
        (c01[2] as f32 * (1.0 - tx) + c11[2] as f32 * tx) as u8,
    ];

    [
        (c0[0] as f32 * (1.0 - ty) + c1[0] as f32 * ty) as u8,
        (c0[1] as f32 * (1.0 - ty) + c1[1] as f32 * ty) as u8,
        (c0[2] as f32 * (1.0 - ty) + c1[2] as f32 * ty) as u8,
    ]
}