use crate::{GameLayer, PARTICLE_RADIUS, PARTICLE_SPAWN_SPACING, Particle, SPAWN_GRID_WIDTH};
use avian2d::prelude::*;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = q_windows.single().expect("window not found");

    crate::hud::spawn_hud(&mut commands, &window);

    // Particles
    let particle_mesh = meshes.add(Circle::new(PARTICLE_RADIUS));
    let particle_material = materials.add(Color::srgb(0.2, 0.8, 1.0));

    let particle_layers = CollisionLayers::new(GameLayer::Particle, [GameLayer::Wall]);
    let x0 =
        -(SPAWN_GRID_WIDTH as f32) * PARTICLE_SPAWN_SPACING / 2.0 + PARTICLE_SPAWN_SPACING / 2.0;
    let y0 =
        -(SPAWN_GRID_WIDTH as f32) * PARTICLE_SPAWN_SPACING / 2.0 + PARTICLE_SPAWN_SPACING / 2.0;
    for i in 0..SPAWN_GRID_WIDTH {
        for j in 0..SPAWN_GRID_WIDTH {
            let x = x0 + i as f32 * PARTICLE_SPAWN_SPACING;
            let y = y0 + j as f32 * PARTICLE_SPAWN_SPACING;
            commands.spawn((
                Mesh2d(particle_mesh.clone().into()),
                MeshMaterial2d(particle_material.clone()),
                Transform::from_xyz(x, y, 1.0),
                RigidBody::Dynamic,
                Collider::circle(PARTICLE_RADIUS),
                particle_layers,
                LinearVelocity::ZERO,
                Particle {
                    mesh_id: particle_mesh.id(),
                    material_id: particle_material.id(),
                },
                Name::new(format!("Particle_{i}_{j}")),
            ));
        }
    }
}
