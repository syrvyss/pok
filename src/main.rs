use bevy::prelude::*;
use bevy::window::PresentMode;
use rand::Rng;

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

        .add_state(AppState::OverWorld)
        .add_system_set(
           SystemSet::on_enter(AppState::OverWorld)
               .with_system(setup_world)
        )
        .add_system_set(
           SystemSet::on_update(AppState::OverWorld)
               .with_system(battle_check.label("battle_check"))
               .with_system(move_enemy.after("battle_check"))
               .with_system(move_player.after("battle_check"))
        )

        .add_system_set(
            SystemSet::on_enter(AppState::Battle)
                .with_system(is_saved_data)
                .with_system(setup_battle_ui)
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

fn is_saved_data() {
    println!("it's working!");
}

fn setup_world(mut commands: Commands) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // player
    commands.spawn_bundle(PlayerBundle {
        name: Name("player_name".to_string()),
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
        name: Name("enemy_name".to_string()),
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

fn setup_battle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Name, With<Player>>,
    enemy_query: Query<&Name, Without<Player>>
) {
    let player_name = player_query.single();
    let enemy_name = enemy_query.single();

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(85.0), Val::Percent(85.0)),
            justify_content: JustifyContent::SpaceBetween,
            margin: Rect::all(Val::Auto),
            padding: Rect::all(Val::Percent(10.)),
            ..default()
        },
        color: UiColor(Color::DARK_GRAY),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                &player_name.0,
                TextStyle {
                    font: asset_server.load("fonts/ComicMono-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default()
            ),
            style: Style {
                margin: Rect { 
                    top: Val::Auto, 
                    bottom: Val::Auto, 
                    ..default() 
                },

                size: Size::new(Val::Percent(20.), Val::Percent(5.)),
                ..default()
            },
            ..default()
        });

        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                &enemy_name.0,
                TextStyle {
                    font: asset_server.load("fonts/ComicMono-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default()
            ),
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::FlexEnd,
                margin: Rect {
                    top: Val::Auto,
                    bottom: Val::Auto,
                    ..default() 
                },
                size: Size::new(Default::default(), Val::Percent(5.)),
                ..default()
            },
            ..default()
        });
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

fn move_enemy(
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
    mut app_state: ResMut<State<AppState>>,
    enemy_query: Query<&mut Transform, Without<Player>>,
    player_query: Query<&mut Transform, With<Player>>
) {
    let player_pos = player_query.single();
        for enemy in enemy_query.iter() {
            if enemy.translation == player_pos.translation {
                app_state.push(AppState::Battle).unwrap()
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

#[derive(Component, PartialEq, Eq)]
struct Name(String);

#[derive(Component)]
struct Health(u16);