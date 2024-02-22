use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Sunlight;

#[derive(Component)]
pub struct Player {
    speed: f32
}

impl Default for Player {
    fn default() -> Player {
        Player { 
            speed: 0.5 
        }
    }
}

pub fn setup_player(
    mut commands: Commands
) {
    // We need to spawn a directional light for the sun, we spawn it in the player module because we move it along with the player

    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(-20.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            ..default()
        },
        Sunlight
    ));

    // Spawn the player camera

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }.into(),
            ..default()
        },
        Player::default()
    ));
}

pub fn update_player(
    mut query: Query<(&Player, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (player, mut player_transform) = query.get_single_mut().expect("Can't find player???");

    let final_speed = player.speed * time.delta_seconds();

    for &key in keys.get_pressed() {
        match key {

            KeyCode::W => {
                player_transform.translation += Vec3::X * final_speed;
            },

            KeyCode::S => {
                player_transform.translation += Vec3::NEG_X * final_speed;
            },

            KeyCode::A => {

            },

            KeyCode::D => {

            }

            _ => ()
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}
