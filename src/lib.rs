mod hud;
mod setup;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;
use std::ops::AddAssign;
// use anyhow::{Result, Error};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::ViewportConversionError;

/// Number of particles along each axis (total = GRID^2)
const SPAWN_GRID_WIDTH: usize = 16;
const PARTICLE_RADIUS: f32 = 2.0;
const PARTICLE_SPAWN_SPACING: f32 = 5.0;

const ATTRACTOR_MASS: f32 = 10000.0;
const PARTICLE_MASS: f32 = 1.0;
const MAX_PARTICLE_VELOCITY: f32 = 500.0;

#[derive(Debug, Eq, PartialEq, Hash)]
enum PName {
    MaxAcceleration,
    InterParticleGravity,
    AttractorGravity,
}

struct PhysicsManipulable {
    key_increase: KeyCode,
    key_decrease: KeyCode,
    delta: f32,
    value: f32,
    name: PName,
}

impl PhysicsManipulable {
    pub fn handle_keys(&mut self, keys: &Res<ButtonInput<KeyCode>>) {
        if keys.pressed(self.key_increase) {
            self.value += self.delta;
        }
        if keys.pressed(self.key_decrease) {
            self.value -= self.delta;
        }
    }
}

#[derive(Resource)]
struct Physics(HashMap<PName, PhysicsManipulable>);

impl Physics {
    pub fn get_value(&self, name: PName) -> Option<f32> {
        self.0.get(&name).map(|e| e.value)
    }
    pub fn get(&self, name: PName) -> Option<&PhysicsManipulable> {
        self.0.get(&name)
    }
}

impl Default for Physics {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            PName::MaxAcceleration,
            PhysicsManipulable {
                name: PName::MaxAcceleration,
                value: 4000.0,
                key_increase: KeyCode::KeyM,
                key_decrease: KeyCode::KeyN,
                delta: 100.0,
            },
        );
        map.insert(
            PName::InterParticleGravity,
            PhysicsManipulable {
                name: PName::InterParticleGravity,
                value: 400000.0,
                key_increase: KeyCode::KeyG,
                key_decrease: KeyCode::KeyF,
                delta: 10000.0,
            },
        );
        map.insert(
            PName::AttractorGravity,
            PhysicsManipulable {
                name: PName::AttractorGravity,
                value: 50000.0,
                key_increase: KeyCode::KeyC,
                key_decrease: KeyCode::KeyX,
                delta: 1000.0,
            },
        );
        Self(map)
    }
}

trait UpdateHud {
    fn update_hud(&self, hud_text: &mut Text);
}

impl UpdateHud for Physics {
    fn update_hud(&self, hud_text: &mut Text) {
        **hud_text = self
            .0
            .iter()
            .map(|kv| {
                let (_, val) = kv;
                format!(
                    "{:?}: {}\n{:?} to increase, {:?} to decrease",
                    val.name, val.value, val.key_increase, val.key_decrease
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
    }
}

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Particle,
    Wall,
}

#[derive(Component)]
struct Particle {
    mesh_id: AssetId<Mesh>,
    material_id: AssetId<ColorMaterial>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            PhysicsPlugins::default()
                .with_length_unit(10.0)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
        )
        .insert_resource(Gravity::ZERO)
        .insert_resource(Physics::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            (
                nbody_and_cursor_gravity,
                modify_physics_constants,
                hud::update_hud,
            ),
        );

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}

impl AddAssign<f32> for &mut PhysicsManipulable {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs;
    }
}

fn modify_physics_constants(
    keys: Res<ButtonInput<KeyCode>>,
    mut physics_constants: ResMut<Physics>,
) {
    physics_constants
        .0
        .iter_mut()
        .for_each(|(_, p)| p.handle_keys(&keys))
}

trait Attractor {}

fn get_attractor_position(
    attr: Vec2,
    camera: (&Camera, &GlobalTransform),
) -> Result<Vec2, ViewportConversionError> {
    camera.0.viewport_to_world_2d(camera.1, attr)
}

fn nbody_and_cursor_gravity(
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    touches: Res<Touches>,
    physics_constants: ResMut<Physics>,
    mut q_particles: Query<(&GlobalTransform, &mut LinearVelocity), With<Particle>>,
) -> Result<()> {
    let camera = q_camera.single()?;
    // handle finger positions and/or cursor positions
    // create attraction vectors to each
    let mut attractors: Vec<Vec2> = vec![];
    if let Some(raw_cursor_pos) = q_windows.single()?.cursor_position() {
        let cursor_world = get_attractor_position(raw_cursor_pos, camera)?;
        attractors.push(cursor_world);
    }

    for finger in touches.iter() {
        attractors.push(get_attractor_position(finger.position(), camera)?);
    }

    let particle_positions: Vec<Vec2> = q_particles
        .iter_mut()
        .map(|(transform, _)| transform.translation().truncate())
        .collect();

    // Now for each particle, sum forces
    let dt = time.delta_secs();
    let mut i = 0;
    for (_, mut velocity) in q_particles.iter_mut() {
        let pos_i = particle_positions[i];
        let mut accel = Vec2::ZERO;

        // Add attraction to every other particle
        for (j, pos_j) in particle_positions.iter().enumerate() {
            if i == j {
                continue;
            }
            let delta = *pos_j - pos_i;
            let r2 = delta.length_squared().max(1.0);
            let a = physics_constants
                .get(PName::InterParticleGravity)
                .unwrap()
                .value
                * PARTICLE_MASS
                * PARTICLE_MASS
                / r2;
            accel += delta.normalize_or_zero() * a;
        }

        for attractor in attractors.iter() {
            let to_cursor = attractor - pos_i;
            let r2 = to_cursor.length_squared().max(1.0);
            let a = physics_constants
                .get_value(PName::AttractorGravity)
                .unwrap()
                * ATTRACTOR_MASS
                / r2;
            accel += to_cursor.normalize_or_zero() * a;
        }

        // Clamp acceleration and velocity for stability
        accel =
            accel.clamp_length_max(physics_constants.get_value(PName::MaxAcceleration).unwrap());

        velocity.x += accel.x * dt;
        velocity.y += accel.y * dt;
        let speed2 = velocity.x * velocity.x + velocity.y * velocity.y;
        if speed2 > MAX_PARTICLE_VELOCITY * MAX_PARTICLE_VELOCITY {
            let scale = MAX_PARTICLE_VELOCITY / speed2.sqrt();
            velocity.x *= scale;
            velocity.y *= scale;
        }
        i += 1;
    }

    Ok(())
}

// fn update_particle_transparency(
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     meshes.
// }
