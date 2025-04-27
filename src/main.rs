use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::AssetServer,
    prelude::{Camera2d, Commands, Query, Res, Transform},
    sprite::Sprite,
    time::Time,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_bird)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0., 100., 0.),
        Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            ..Default::default()
        },
    ));
    commands.spawn((
        Transform::from_xyz(0., -100., 0.),
        Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            ..Default::default()
        },
    ));

    commands.spawn(Sprite {
        image: asset_server.load("array_texture.png"),
        ..Default::default()
    });
}

fn move_bird(mut transforms: Query<&mut Transform>, time: Res<Time>) {
    for mut transform in transforms.iter_mut() {
        if transform.translation.y != 0. {
            transform.translation.z = time.elapsed_secs().sin();
        }
    }
}
