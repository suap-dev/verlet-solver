#![allow(dead_code)]

// NOTES (for future refference not really applied):
// 1) screen_coordinates: u32 -- simply current state of framebuffers coordinates system - u32
// 2) gl_coords: f32  and  gl_length: f32  -- actual screen coordinates and length according to OpenGL
// 3) logical_coordinates: f32/f64  and  logical_length: f32/f64 -- my internal system of coordinates according to world size (and unit?)
// 4) world_size: [u32; 2] -- logical size of the world
// 5) unit: f32/f64 -- do we even need this? it can be used to translate the screen coordinate to logical? nevermind.

use std::f32::consts::TAU;

use nalgebra_glm::{mat2, vec2, Vec2};

// shape will speak logical units
pub struct Shape {
    pub vertices: Vec<Vec2>,
    pub color: [f32; 4],
    pivot_point: Vec2,
    rotation: f32,
}
pub const VERTICES_OF_A_CIRCLE: u16 = 6;
pub const CIRCLE_INDICES: [u16; 12] = [
    0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4,
    5,
    // 0, 5, 6,
    // 0, 6, 7,
    // 0, 7, 8,
    // 0, 8, 9,
    // 0, 9, 10,
    // 0, 10, 11,
    // 0, 11, 12,
    // 0, 12, 13,
    // 0, 13, 14,
    // 0, 14, 15,
    // 0, 15, 16,
    // 0, 16, 17
];
impl Shape {
    pub fn polygon(vertices: Vec<Vec2>, color: [f32; 4]) -> Self {
        Self {
            vertices,
            color,
            pivot_point: vec2(0.0, 0.0),
            rotation: 0.0,
        }
    }

    pub fn circle(radius: f32, color: [f32; 4]) -> Self {
        let interior_angle = TAU / VERTICES_OF_A_CIRCLE as f32;

        let mut vertices: Vec<Vec2> = Vec::new();

        let mut temp_vertex_position = vec2(0.0, radius);
        let rotation_matrix = mat2(
            interior_angle.cos(),
            -interior_angle.sin(),
            interior_angle.sin(),
            interior_angle.cos(),
        );

        vertices.push(temp_vertex_position); // 0-th vertex

        for _ in 1..VERTICES_OF_A_CIRCLE {
            temp_vertex_position = rotation_matrix * temp_vertex_position;
            vertices.push(temp_vertex_position);
        }

        Self {
            vertices,
            color,
            pivot_point: vec2(0.0, 0.0),
            rotation: 0.0,
        }
    }

    pub fn rectangle(width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            vertices: vec![
                vec2(height / 2.0, -width / 2.0),
                vec2(-height / 2.0, -width / 2.0),
                vec2(-height / 2.0, width / 2.0),
                vec2(height / 2.0, width / 2.0),
            ],
            color,
            pivot_point: vec2(0.0, 0.0),
            rotation: 0.0,
        }
    }
}
