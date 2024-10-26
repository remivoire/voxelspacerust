# voxelspacerust
this is a rust implementation of the infamous voxel space algorithm used by novalogic for their early comanche games and delta force 1 & 2

## features

- **load your own maps**: you can pick custom colormap and heightmap images with a file picker
- **sky gradient**: "simulates" a sky that goes from light blue to darker blue near the horizon
- **real-time controls**: move around and change angles with your keyboard, giving you an interactive 3d experience
- **optimized performance**: uses frame-rate independent motion and other tweaks to make it smoother even on big maps

## requirements

- **rust** (latest stable version)
- **cargo** (comes with rust)

## dependencies

this project uses a few crates:
- **pixels** for pixel rendering
- **winit** for managing windows and events
- **image** for handling image files
- **rfd** for the file picker (cross-platform)

add these dependencies to your `cargo.toml`:

```toml
[dependencies]
pixels = "0.10"
winit = "0.27"
image = "0.23"
rfd = "0.11"
```
yes, i know they're not the latest

## installation

1. **clone the repo**:
   ```bash
   git clone https://github.com/remivoire/voxelspacerust.git
   cd voxel-landscape-renderer
   ```

2. **add default assets** (optional): 
   - if you don’t want to pick your own images each time, place default images in an `assets` folder in the root of the project
   - use these filenames:
     - colormap: `assets/map0.color.gif`
     - heightmap: `assets/map0.height.gif`

3. **run the app**:
   ```bash
   cargo run --release
   ```
   > pro tip: running in release mode is faster, so it’s recommended

## usage

### loading custom maps

when you launch the app, a file dialog will pop up for you to choose:
- **colormap**: an rgb image file (like `.png`, `.jpg`, or `.gif`) to give colors to the landscape
- **heightmap**: a grayscale image file (also `.png`, `.jpg`, or `.gif`) to provide height info for the landscape

if you cancel either dialog, it’ll load the default assets from the `assets` folder (if available)

### controls

use the following keys to move around and interact with the landscape:

| key              | action                         |
|------------------|--------------------------------|
| `up arrow`       | move forward                   |
| `down arrow`     | move backward                  |
| `left arrow`     | rotate left                    |
| `right arrow`    | rotate right                   |
| `w`              | look up                        |
| `s`              | look down                      |
| `e`              | move camera higher             |
| `d`              | move camera lower              |
| `esc`            | exit the app                   |

### sky gradient

a smooth sky gradient effect is applied automatically. it transitions from a light blue at the top to a darker blue near the horizon

## code structure

- **`main` function**: sets up the window, file loading, camera, and starts the event loop
- **`handle_window_event`**: processes keyboard input and window events
- **`update`**: updates camera position and angles using delta time for smooth movement
- **`draw`**: handles rendering the landscape and sky gradient with some optimized ray marching for performance

## optimization tips

1. **reduce depth**: `zfar` depth is reduced to make rendering faster
2. **bigger ray marching steps**: instead of processing every depth layer it steps by 2 layers to cut down computations
3. **frame-rate independent motion**: by using delta time, movement stays smooth regardless of the frame rate
