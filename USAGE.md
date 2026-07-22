# rt Usage Guide

## What this ray tracer does

This project renders a 3D scene by shooting rays from a camera through each pixel of the image. Each ray checks what it hits first in the world, and that hit information is used to decide the pixel color.

The current version supports:
- a sphere
- a flat plane
- a cube
- a cylinder
- simple directional lighting
- simple shadows
- moving the camera viewpoint

## How the project is organized

- `src/main.rs` builds the scene, sets the camera, and prints the final PPM image.
- `src/world.rs` stores all objects and finds the closest hit.
- `src/hit.rs` stores the result of one ray-object intersection.
- `src/sphere.rs`, `src/plane.rs`, `src/cube.rs`, and `src/cylinder.rs` define the objects.
- `src/camera.rs` creates rays for each pixel.
- `src/vec3.rs` provides vector math.

## Running the renderer

To generate a PPM image, run:

```bash
cargo run --release > output.ppm