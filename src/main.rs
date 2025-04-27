use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource, Default)]
struct Data;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_resource::<Data>()
        .configure_schedules(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..Default::default()
        })
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(Update, game.run_if(in_state(AppState::InGame)))
        .run();
}

fn menu(mut _data: ResMut<Data>) {}
fn game(mut _data: ResMut<Data>) {}
