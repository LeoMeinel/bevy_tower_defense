/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */
use bevy::prelude::*;

// This is messy but it should be noted that it allows for easy global customizations!
pub const WINDOW_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_TITLE: &str = "Bevy Tower Defense";
pub const WINDOW_RESIZABLE: bool = true;
pub const CAMERA0: ((f32, f32, f32), (Vec3, Vec3)) = ((-2.0, 2.5, 5.0), (Vec3::ZERO, Vec3::Y));
pub const PLANE0: (f32, (f32, f32, f32)) = (5.0, (0.3, 0.5, 0.3));
pub const CUBE0: (f32, (f32, f32, f32), (f32, f32, f32)) =
    (1.0, (0.67, 0.84, 0.92), (0.0, 0.5, 0.0));

fn main() {
    App::new()
        .insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: WINDOW_TITLE.to_string(),
            resizable: WINDOW_RESIZABLE,
            ..Default::default()
        })
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera.after(spawn_basic_scene))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(CAMERA0.0 .0, CAMERA0.0 .1, CAMERA0.0 .2)
            .looking_at(CAMERA0.1 .0, CAMERA0.1 .1),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE0.0 })),
        material: materials.add(Color::rgb(PLANE0.1 .0, PLANE0.1 .1, PLANE0.1 .2).into()),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE0.0 })),
        material: materials.add(Color::rgb(CUBE0.1 .0, CUBE0.1 .1, CUBE0.1 .2).into()),
        transform: Transform::from_xyz(CUBE0.2 .0, CUBE0.2 .1, CUBE0.2 .2),
        ..default()
    });
}
