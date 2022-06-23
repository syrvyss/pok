use bevy::prelude::*;
use bevy::window::PresentMode;
use rand::Rng;

pub mod battle;
pub mod entities;

mod cleanup {
    use bevy::prelude::*;
    #[derive(Component)]
    pub struct LevelUnload;
    #[derive(Component)]
    pub struct MenuClose;
}

const CUBE_SIZE: f32 = 16.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "pok".to_string(),
            present_mode: PresentMode::Mailbox,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(battle::BattleUi {
            
        })

        .add_state(AppState::OverWorld)
        .add_system_set(
           SystemSet::on_enter(AppState::OverWorld)
               .with_system(setup_world)
        )
        .add_system_set(
           SystemSet::on_update(AppState::OverWorld)
               .with_system(entities::battle_check.label("battle_check"))
               .with_system(entities::move_enemy.after("battle_check"))
               .with_system(entities::move_player.after("battle_check"))
        )

        .add_event::<entities::GameTickEvent>()

        .run();
}

pub fn setup_world(mut commands: Commands) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // player
    commands.spawn_bundle(entities::PlayerBundle {
        name: Name("player_name".to_string()),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 50.),
                custom_size: Some(Vec2::splat(crate::CUBE_SIZE.into())),
                ..default()
            },
            transform: Transform::from_xyz(crate::CUBE_SIZE * 5., crate::CUBE_SIZE * 5., 0.),
            ..default()
        },
        items: Items(vec![0, 0, 0]),
        _p: entities::Player
    });

    commands.spawn_bundle(entities::EnemyBundle {
        name: Name("enemy_name".to_string()),
        health: Health(100),
        experience: Experience(0),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 50., 0.25),
                custom_size: Some(Vec2::splat(crate::CUBE_SIZE.into())),
                ..default()
            },
            transform: Transform::from_xyz(crate::CUBE_SIZE, crate::CUBE_SIZE, 0.),
            ..default()
        },
        _e: entities::Enemy
    });
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    OverWorld,
    Battle
}

#[derive(Component)]
struct Items(Vec<u8>);

#[derive(Component)]
struct Experience(u16);

#[derive(Component, PartialEq, Eq)]
struct Name(String);

#[derive(Component)]
struct Health(u16);