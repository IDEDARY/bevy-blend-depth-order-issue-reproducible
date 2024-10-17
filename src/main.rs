use bevy::core_pipeline::oit::OrderIndependentTransparencySettings;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (zoom_camera, rotate_camera))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        OrbitCamera {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 800.0,
            sensitivity: Vec2::splat(0.1),
        },
        // Add this to fix transparency
        OrderIndependentTransparencySettings::default(),
    )).insert(
        // Msaa currently doesn't work with OIT
        Msaa::Off,
    );

    // Spawn 3 panels
    for x in [-1, 0, 1] {

        // Spawn transparent panel
        commands.spawn((
            Transform::from_xyz(0.0, 0.0, 300.0 * x as f32),
            Mesh3d(mesh.add(Rectangle::new(818.0, 965.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("panel.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
        )).with_children(|panel| {

            // Spawn transparent panel overlay as child
            panel.spawn((
                Transform::from_xyz(0.0, 400.0, 50.0),
                Mesh3d(mesh.add(Rectangle::new(818.0, 169.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("panel_head.png")),
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
            ));

        });
    }

}


// BOILERPLATE  ----------

use bevy::input::mouse::{MouseMotion, MouseWheel};
#[derive(Component)]
pub struct OrbitCamera {
    pub orbit: Vec3,
    pub distance: f32,
    pub sensitivity: Vec2,
}
pub fn rotate_camera(mut mouse_motion_events: EventReader<MouseMotion>, mouse_input: Res<ButtonInput<MouseButton>>, mut query: Query<(&OrbitCamera, &mut Transform)>) {
    let mut delta = Vec2::ZERO;
    if mouse_input.pressed(MouseButton::Left) {
        delta = mouse_motion_events.read().map(|e| e.delta).sum();
    }
    if mouse_input.just_pressed(MouseButton::Left) {
        delta = Vec2::ZERO;
    }
    for (camera, mut transform) in &mut query {

        // ROTATION 
        let (mut rx, mut ry, rz) = transform.rotation.to_euler(EulerRot::YXZ);
        rx += (-delta.x * camera.sensitivity.x).to_radians();
        ry += (-delta.y * camera.sensitivity.x).to_radians();
        ry = ry.clamp(-90_f32.to_radians(), 90_f32.to_radians());
        transform.rotation = Quat::from_euler(EulerRot::YXZ, rx, ry, rz);


        // ORBIT TRANSFORM
        let tx = camera.distance * rx.sin();
        let ty = camera.distance * rx.cos();
        let tz = camera.distance * ry.sin();

        let diff = camera.distance * ry.cos();
        let plane_ratio_decrease = (camera.distance - diff)/camera.distance;

        transform.translation = camera.orbit;
        transform.translation.x += tx * (1.0 - plane_ratio_decrease);
        transform.translation.z += ty * (1.0 - plane_ratio_decrease);
        transform.translation.y += -tz;
    }
}
pub fn zoom_camera(mut mouse_wheel_events: EventReader<MouseWheel>, mut query: Query<&mut OrbitCamera>) {
    let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
    for mut camera in &mut query {
        camera.distance += -delta*25.0;
    }
}
