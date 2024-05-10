use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::{WindowResized, WindowResolution},
};

#[derive(Component)]
struct Canvas;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800.0, 800.0),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();

    // camera
    commands.spawn(Camera2dBundle::default());
    let width = window.resolution.width();
    let height = window.resolution.height();
    info!("{} {}", width, height);
    // quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(1., 1.)).into(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.0))
                .with_scale(Vec3::new(width, height, 1.)),
            material: materials.add(CustomMaterial {
                color: Color::BLUE,
                resolution: Vec2::new(width, height),
            }),

            ..default()
        },
        Canvas,
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    resolution: Vec2,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/uniforms.wgsl".into()
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut q: Query<&mut Transform, With<Canvas>>,
    window: Query<&Window>,
) {
    let window = window.single();
    for _ in resize_reader.read() {
        let mut transform = q.single_mut();
        transform.scale = Vec3::new(window.resolution.width(), window.resolution.height(), 1.);
        info!(
            "{} {}",
            window.resolution.width(),
            window.resolution.height()
        );
    }

    
}
