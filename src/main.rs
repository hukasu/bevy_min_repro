use bevy::{
    DefaultPlugins,
    app::App,
    asset::AssetServer,
    prelude::{Camera2dBundle, Commands, Query, Res, Transform},
    sprite::SpriteBundle,
    time::Time,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_bird)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        transform: Transform::from_xyz(0., 100., 0.),
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        transform: Transform::from_xyz(0., -100., f32::EPSILON),
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("array_texture.png"),
        ..Default::default()
    });
}

fn move_bird(mut transforms: Query<&mut Transform>, time: Res<Time>) {
    for mut transform in transforms.iter_mut() {
        if transform.translation.y != 0. {
            transform.translation.z = time.elapsed_seconds().sin();
        }
    }
}
