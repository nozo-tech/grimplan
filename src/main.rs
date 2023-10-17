use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

#[derive(Resource, Default)]
struct LevelResource {
    voxels: Vec<Vec3>
}

#[derive(Component)]
struct LevelQuad;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .init_resource::<LevelResource>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_level)
        .add_systems(PostUpdate, update_level)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(
            OrthographicProjection::default()
        ),
        ..default()
    });
}

fn setup_level(
    mut commands: Commands,
    mut level_res: ResMut<LevelResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::default().into()),
            material: materials.add(Color::LIME_GREEN.into()),
            ..default()
        },
        LevelQuad
    ));
}

fn update_level(
    mut query: Query<&Handle<Mesh>, With<LevelQuad>>,
    level_res: Res<LevelResource>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mesh_handle = query.single();

    if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
        println!("{:#?}", mesh)
    }
}
