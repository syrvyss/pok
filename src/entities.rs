use bevy::prelude::*;
use rand::Rng;

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut ev_tick: EventWriter<GameTickEvent>
) {
    let mut player_pos = player_query.single_mut();

    if input.just_pressed(KeyCode::W) {
        player_pos.translation.y += crate::CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::A) {
        player_pos.translation.x -= crate::CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::S) {
        player_pos.translation.y -= crate::CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }

    if input.just_pressed(KeyCode::D) {
        player_pos.translation.x += crate::CUBE_SIZE;
        ev_tick.send(GameTickEvent());
    }
}

pub fn move_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut ev_tick: EventReader<GameTickEvent>
) {
    for event in ev_tick.iter() {
        let mut rng = rand::thread_rng();
        for mut enemy_pos in enemy_query.iter_mut() {
            let rand_num = rng.gen_range(0..4);
            match rand_num {
                0 => enemy_pos.translation.x += crate::CUBE_SIZE,
                1 => enemy_pos.translation.x -= crate::CUBE_SIZE,
                2 => enemy_pos.translation.y += crate::CUBE_SIZE,
                3 => enemy_pos.translation.y -= crate::CUBE_SIZE,
                _ => println!("something went wrong")
            }
        }
    }
}

pub fn battle_check(
    mut app_state: ResMut<State<crate::AppState>>,
    enemy_query: Query<&mut Transform, Without<Player>>,
    player_query: Query<&mut Transform, With<Player>>
) {
    let player_pos = player_query.single();
        for enemy in enemy_query.iter() {
            if enemy.translation == player_pos.translation {
                app_state.push(crate::AppState::Battle).unwrap()
            }
        }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: crate::Name,
    items: crate::Items,
    _p: Player,

    #[bundle]
    sprite: SpriteBundle
}

#[derive(Bundle)]
pub struct EnemyBundle {
    name: crate::Name,
    health: crate::Health,
    experience: crate::Experience,
    _e: Enemy,

    #[bundle]
    sprite: SpriteBundle
}

pub struct GameTickEvent();

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;
