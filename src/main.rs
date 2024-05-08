use std::cell::RefCell;

use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Perlin, Terrace,
};
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

pub mod ui;

mod chain;
mod helpers;

use chain::{ONCHAIN_MAP_SEED};

const NOISEMAP_SIZE: (usize, usize) = (100usize, 100usize);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Build,
    Finished,
    Ledger,
}


#[derive(Resource)]
struct Seed(u32);

#[derive(Resource, Deref)]
struct Root(Entity);

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    pos_x: f32,
    pos_y: f32,
    move_cooldown: Timer,
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
}

// TODO: noise generator from seed.
fn generate_random_map() -> noise::utils::NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    generate_random_map_with_seed(seed)
}


pub struct OnChainPlayer {
    pub address: String,
    pub score: String,
}

/// Creates a procederally-generated map using noise.
fn generate_random_map_with_seed(seed: u32) -> noise::utils::NoiseMap {
    let perlin = Perlin::new(seed);

    let terrace_inverted: Terrace<f64, Perlin, 2> = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0)
        .invert_terraces(true);

    PlaneMapBuilder::new(terrace_inverted)
        .set_size(NOISEMAP_SIZE.0, NOISEMAP_SIZE.1)
        .build()
}

fn get_color(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#0a7e0a"),
        v if v < 0.2 => Color::hex("#0da50d"),
        v if v < 0.3 => Color::hex("#10cb10"),
        v if v < 0.4 => Color::hex("#18ed18"),
        v if v < 0.5 => Color::hex("#3ff03f"),
        v if v < 0.6 => Color::hex("#65f365"),
        v if v < 0.7 => Color::hex("#8cf68c"),
        v if v < 0.8 => Color::hex("#b2f9b2"),
        v if v < 0.9 => Color::hex("#d9fcd9"),
        v if v <= 1.0 => Color::hex("#ffffff"),
        _ => panic!("unexpected value"),
    };
    color_result.expect("Getting color from HEX error")
}

fn get_tile(val: f64, asset_server: &Res<AssetServer>) -> Handle<Image> {
    match val.abs() {
        v if v < 0.1 => asset_server.load("Tile/grass.png"),
        v if v < 0.2 => asset_server.load("Tile/trees.png"),
        v if v < 0.3 => asset_server.load("Tile/grass_1.png"),
        // v if v < 0.4 => Color::hex("#18ed18"),
        // v if v < 0.5 => Color::hex("#3ff03f"),
        // v if v < 0.6 => Color::hex("#65f365"),
        v if v < 0.7 => asset_server.load("Tile/water.png"),
        v if v < 0.8 => asset_server.load("Tile/trees_1.png"),
        v if v < 0.9 => asset_server.load("Tile/forest.png"),
        v if v <= 1.0 => asset_server.load("Tile/sand.png"),
        _ => asset_server.load("Tile/medievalTile_41.png"),
    }
}

fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut moved = false;
    let mut rotation = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        game.player.pos_y += 1f32;
        // rotation = -PI / 2.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        game.player.pos_y -= 1f32;
        // rotation = PI / 2.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        game.player.pos_x += 1f32;
        // rotation = PI;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        game.player.pos_x -= 1f32;
        // rotation = 0.0;
        moved = true;
    }

    // move on the board
    if moved {
        game.player.move_cooldown.reset();
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(game.player.pos_x as f32, game.player.pos_y, 0f32),
            rotation: Quat::from_rotation_y(rotation),
            ..default()
        };
    }
}

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    let seed = ONCHAIN_MAP_SEED.with(|s| s.borrow().to_owned());

    let map = generate_random_map_with_seed(seed);
    let (grid_width, grid_height) = map.size();

    let tile_size = 32_f32;

    let start_x = -(grid_width as f32) * tile_size / 2.0;
    let start_y = -(grid_height as f32) * tile_size / 2.0;

    let root = commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            for col_x in 0..grid_width {
                for col_y in 0..grid_height {
                    let val = map.get_value(col_x, col_y);
                    // if val > 0.8_f64 {
                    // debug!("Value for {}:{} = {}", col_x, col_y, val);
                    // }
                    let x = start_x + col_x as f32 * tile_size;
                    let y = start_y + col_y as f32 * tile_size;

                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            // color: get_tile(val, asset_server),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..default()
                        },
                        texture: get_tile(val, &asset_server),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                    });
                }
            }
        })
        .id();

    let starting_player_pos = GlobalTransform::default().translation_vec3a();

    game.player.pos_x = starting_player_pos.x;
    game.player.pos_y = starting_player_pos.y;

    game.player.entity = Some(
        // Spawn Player
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64f32, 64f32)),
                    ..default()
                },
                texture: asset_server.load("Unit/player.png"),
                ..default()
            })
            .id(),
    );

    commands.insert_resource(Root(root));
    // commands.insert_resource(Player(player));

    next_state.set(AppState::Finished);
    // Create map
}

// Create player sprite
// allow movement of player
pub fn onchain_events(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    ONCHAIN_MAP_SEED.with(|seed| {
        let seed = seed.borrow();
        if *seed != 0 {
            dbg!("Seed is set to {}", *seed);
            commands.insert_resource(Seed(*seed));
            next_state.set(AppState::Build);
        }
    });
    // If resource is set, then do something here!
}

pub fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn_recursive();
}

fn reset(mut events: EventReader<ui::ResetMapEvent>, mut next_state: ResMut<NextState<AppState>>) {
    for _ in events.read() {
        next_state.set(AppState::Build);
    }
}

fn ledger_menu(
    mut events: EventReader<ui::ShowLedgerEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for e in events.read() {
        if e.0 == true {
            println!("Ledger event");
            next_state.set(AppState::Ledger);
        }
    }
}

fn exit_ledger_menu(
    mut commands: Commands,
    mut events: EventReader<ui::ShowLedgerEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    table_root: Res<TableRoot>,
) {
    dbg!("EXIT LEDGER MENU");
    for e in events.read() {
        if e.0 == false {
            commands.entity(table_root.0).despawn_recursive();
            next_state.set(AppState::Finished);
        }
    }
}

#[derive(Resource)]
struct TableRoot(Entity);

impl Default for TableRoot {
    fn default() -> Self {
        TableRoot(Entity::from_raw(0))
    }
}

#[derive(Clone, Copy)]
enum Tables {
    EconomicVictory,
    CulturalVictory,
    DiplomaticVictory,
}

impl ToString for Tables {
    fn to_string(&self) -> String {
        match self {
            Tables::EconomicVictory => "Economic Victory".to_string(),
            Tables::CulturalVictory => "Cultural Victory".to_string(),
            Tables::DiplomaticVictory => "Diplomatic Victory".to_string(),
        }
    }
}

impl From<usize> for Tables {
    fn from(i: usize) -> Self {
        match i {
            0 => Tables::EconomicVictory,
            1 => Tables::CulturalVictory,
            2 => Tables::DiplomaticVictory,
            _ => Tables::EconomicVictory,
        }
    }
}

#[derive(Component)]
struct Tab {
    table_id: Tables,
}

#[derive(Component)]
struct TableRow {
    table_id: Tables,
}

// Economic Victory, Cultural Victory, Diplomatic Victory
fn show_ledger(mut commands: Commands, asset_server: Res<AssetServer>) {
    let table_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(80.0),
                left: Val::Percent(25.0),
                top: Val::Percent(10.0),
                bottom: Val::Percent(10.0),
                right: Val::Percent(25.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                border: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            border_color: Color::WHITE.into(),
            ..default()
        })
        .id();

    commands.insert_resource(TableRoot(table_root));

    commands.entity(table_root).with_children(|parent| {
        // Header
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Ledger Stats",
                TextStyle {
                    font: asset_server.load("PoetsenOne-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        });

        // Tabs for switching tables
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            })
            .with_children(|tabs| {
                for i in 0..3 {
                    tabs.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(33.0),
                            height: Val::Percent(100.0),
                            // Styling for each tab button
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        // color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    })
                    .insert(Tab {
                        table_id: Tables::from(i),
                    })
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text::from_section(
                                Tables::from(i).to_string(),
                                TextStyle {
                                    font: asset_server.load("PoetsenOne-Regular.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        });
                    });
                }
            });

        spawn_rows(Tables::EconomicVictory, parent, &asset_server);
    });
}

fn spawn_rows(table_id: Tables, parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    for i in 0..5 {
        parent
            .spawn(TextBundle {
                text: Text::from_section(
                    format!("Row {} of Table {}", i + 1, table_id.to_string()),
                    TextStyle {
                        font: asset_server.load("PoetsenOne-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::GRAY,
                    },
                ),
                ..default()
            })
            .insert(TableRow { table_id });
    }
}

fn tab_click_system(
    mut commands: Commands,
    table_root: Res<TableRoot>,
    mut interaction_query: Query<(&Interaction, &Tab), (Changed<Interaction>, With<Button>)>,
    mut row_query: Query<(Entity, &TableRow)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, tab) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Clear existing rows
            for (entity, row) in row_query.iter_mut() {
                // if row.table_id == tab.table_id {
                commands.entity(entity).despawn_recursive();
                // }
            }

            // Spawn new rows based on the tab clicked
            commands.entity(table_root.0).with_children(|parent| {
                spawn_rows(tab.table_id, parent, &asset_server);
            });
        }
    }
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info"
                        .to_string(),
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiled Map Editor Example"),
                        canvas: Some(".mygame-canvas".into()),
                        ..Default::default()
                    }),
                    ..default()
                }),
            helpers::camera::CameraPlugin,
            ui::UiPlugin,
        ))
        .init_resource::<Game>()
        .init_resource::<TableRoot>()
        .init_state::<AppState>()
        .add_systems(Update, onchain_events)
        .add_systems(Update, tab_click_system)
        .add_systems(OnEnter(AppState::Build), generate_world)
        .add_systems(OnEnter(AppState::Ledger), show_ledger)
        .add_systems(
            Update,
            (player_movement, reset.run_if(in_state(AppState::Finished))),
        )
        .add_systems(Update, ledger_menu.run_if(in_state(AppState::Finished)))
        .add_systems(Update, exit_ledger_menu.run_if(in_state(AppState::Ledger)))
        .add_systems(OnExit(AppState::Finished), cleanup)
        .run();
}
