use bevy::prelude::*;

pub mod entities;

pub struct BattleUi;
impl Plugin for BattleUi {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(crate::AppState::Battle)
                    .with_system(is_saved_data)
                    .with_system(setup_battle_ui)
            );
    }
}

fn is_saved_data() {
    println!("it's working!");
}

fn setup_battle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Name, With<entities::Player>>,
    enemy_query: Query<&Name, Without<entities::Player>>
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