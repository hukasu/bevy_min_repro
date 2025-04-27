use std::time::Duration;

use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::AssetServer,
    prelude::{Camera2d, Commands, IntoScheduleConfigs, Local, Query, Res, Transform, With},
    sprite::Sprite,
    time::common_conditions::on_timer,
    window::{
        MonitorSelection, PrimaryWindow, VideoModeSelection, Window, WindowMode, WindowResolution,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            toggle_fullscreen.run_if(on_timer(Duration::from_secs(4))),
        )
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
        Transform::from_xyz(0., -100., f32::EPSILON),
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

fn toggle_fullscreen(mut state: Local<bool>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut().unwrap();
    if *state {
        window.resolution = WindowResolution::new(800., 640.);
        window.mode = WindowMode::Windowed;
    } else {
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current);
    }
    *state = !*state;
}
