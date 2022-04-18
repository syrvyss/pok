use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::window::PresentMode;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

const CUBE_SIZE: f32 = 16.;
const TIMESTEP_2_PER_SECOND: f64 = 30. / 60.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "pok".to_string(),
            present_mode: PresentMode::Mailbox,
            ..default()
        })
        .add_plugins(DefaultPlugins)

        //.add_state(AppState::Menu)
        //.add_system_set(
        //    SystemSet::on_enter(AppState::Menu)
        //)

        .add_state(AppState::OverWorld)
        .add_system_set(
           SystemSet::on_enter(AppState::OverWorld)
               .with_system(setup_world)
        )
        .add_system_set(
           SystemSet::on_update(AppState::OverWorld)
               .with_system(move_player)
               .with_system(battle_check)
               .with_system(game_tick)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::OverWorld)

        )

        .add_event::<GameTickEvent>()

        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    OverWorld,
    Battle
}

fn setup_world(
    mut commands: Commands, 
    mut app_state: ResMut<State<AppState>>
) {

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // player
    commands.spawn_bundle(PlayerBundle {
        name: Name("test".to_string()),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 50.),
                custom_size: Some(Vec2::splat(CUBE_SIZE.into())),
                ..default()
            },
            transform: Transform::from_xyz(CUBE_SIZE * 5., CUBE_SIZE * 5., 0.),
            ..default()
        },
        items: Items(vec![0, 0, 0]),
        _p: Player
    });

    commands.spawn_bundle(EnemyBundle {
        name: Name("test".to_string()),
        health: Health(100),
        experience: Experience(0),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 50., 0.25),
                custom_size: Some(Vec2::splat(CUBE_SIZE.into())),
                ..default()
            },
            transform: Transform::from_xyz(CUBE_SIZE, CUBE_SIZE, 0.),
            ..default()
        },
        _e: Enemy
    });
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut ev_tick: EventWriter<GameTickEvent>
) {
    let mut player_pos = player_query.single_mut();

    if input.just_pressed(KeyCode::W) {
        player_pos.translation.y += CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::A) {
        player_pos.translation.x -= CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::S) {
        player_pos.translation.y -= CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::D) {
        player_pos.translation.x += CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }
}

fn game_tick(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut ev_tick: EventReader<GameTickEvent>
) {
    for event in ev_tick.iter() {
        let mut rng = rand::thread_rng();
        for mut enemy_pos in enemy_query.iter_mut() {
            let rand_num = rng.gen_range(0..4);
            match rand_num {
                0 => enemy_pos.translation.x += CUBE_SIZE,
                1 => enemy_pos.translation.x -= CUBE_SIZE,
                2 => enemy_pos.translation.y += CUBE_SIZE,
                3 => enemy_pos.translation.y -= CUBE_SIZE,
                _ => println!("something went wrong")
            }
        }
    }
}

fn battle_check(
    mut ev_tick: EventReader<GameTickEvent>,
    mut app_state: ResMut<State<AppState>>,
    enemy_query: Query<&mut Transform, Without<Player>>,
    player_query: Query<&mut Transform, With<Player>>
) {
    let player_pos = player_query.single();
    for event in ev_tick.iter() {
        for enemy in enemy_query.iter() {
            if collide(enemy.translation, enemy.scale.truncate(), player_pos.translation, player_pos.scale.truncate()).is_some() {
                app_state.set(AppState::Battle).unwrap()
            }
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    items: Items,
    _p: Player,

    #[bundle]
    sprite: SpriteBundle
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    health: Health,
    experience: Experience,
    _e: Enemy,

    #[bundle]
    sprite: SpriteBundle
}

struct GameTickEvent();

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Items(Vec<u8>);

#[derive(Component)]
struct Experience(u16);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(u16);