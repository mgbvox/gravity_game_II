use crate::{GameLayer, PARTICLE_RADIUS, PARTICLE_SPAWN_SPACING, Particle, SPAWN_GRID_WIDTH};
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::{LinearVelocity, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::core::Name;
use bevy::prelude::{
    Camera2d, Circle, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Query, ResMut,
    Transform, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = q_windows.single();

    crate::hud::spawn_hud(&mut commands, &window);

    // ParticlesÏ
    let particle_mech = meshes.add(Circle::new(PARTICLE_RADIUS));
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
                Mesh2d(particle_mech.clone().into()),
                MeshMaterial2d(particle_material.clone()),
                Transform::from_xyz(x, y, 1.0),
                RigidBody::Dynamic,
                Collider::circle(PARTICLE_RADIUS),
                particle_layers,
                LinearVelocity::ZERO,
                Particle,
                Name::new(format!("Particle_{i}_{j}")),
            ));
        }
    }
}
