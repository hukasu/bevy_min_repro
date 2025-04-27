use std::time::Duration;

use bevy::{
    DefaultPlugins,
    app::App,
    asset::AssetServer,
    prelude::{Camera2dBundle, Commands, IntoSystemConfig, Local, Query, Res, Transform, With},
    sprite::SpriteBundle,
    time::common_conditions::on_timer,
    window::{PrimaryWindow, Window, WindowMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(toggle_fullscreen.run_if(on_timer(Duration::from_secs(4))))
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

fn toggle_fullscreen(mut state: Local<bool>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    if *state {
        window.resolution = WindowResolution::new(800., 640.);
        window.mode = WindowMode::Windowed;
    } else {
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode = WindowMode::Fullscreen;
    }
    *state = !*state;
}
