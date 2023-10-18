use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LevelResource {
    voxels: Vec<Transform>,
    mesh: Handle<Mesh>
}

#[derive(Component)]
pub struct LevelPlane;

fn setup_level(
    mut commands: Commands,
    mut level_res: ResMut<LevelResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    level_res.voxels = Vec::new();
    level_res.mesh = meshes.add(shape::Plane::default().into());

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: level_res.mesh.clone(),
            material: materials.add(Color::LIME_GREEN.into()),
            ..default()
        },
        LevelPlane
    ));
}

/*

fn update_level(
    level_res: Res<LevelResource>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mesh_handle = level_res.mesh.clone();

    if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
        // println!("{:#?}", mesh)
    }
}

*/

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LevelResource>()
            .add_systems(PostStartup, setup_level);
    }
}
