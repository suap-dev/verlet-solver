#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use engine::{Entity, World};
use glium::glutin::{
    event,
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::{vec2, Mat2};
use std::f32::consts::TAU;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = World::new(&event_loop);

    let mut triangle = Entity::empty();
    triangle.add_vertex(vec2(0.0, 0.5));
    triangle.add_vertex(vec2(-0.5, -0.5));
    triangle.add_vertex(vec2(0.5, -0.5));

    let mut circle = Entity::empty_with_color([1.0, 0.0, 0.0, 1.0]);
    let mut v1 = vec2(0.4, 0.0);
    let steps = 32;
    let angle = TAU / steps as f32;
    let rot = Mat2::new(angle.cos(), -angle.sin(), angle.sin(), angle.cos());
    for _step in 0..steps {
        circle.add_vertex(v1);
        v1 = rot * v1;
    }

    world.add(triangle);
    world.add(circle);

    event_loop.run(move |event, _, control_flow| {
        world.render();
        if let event::Event::WindowEvent { event, .. } = event {
            if let event::WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
