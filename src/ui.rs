use crate::AppState;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), setup_button)
            // .add_systems(Update, setup_ledger_menu_buttons.run_if(in_state(AppState::Ledger)))
            .add_systems(Update, button_system) //button_system.run_if(in_state(AppState::Finished)))
            .add_event::<ResetMapEvent>()
            .add_event::<ShowLedgerEvent>();
    }
}

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
pub enum MenuButtonAction {
    Reset,  // Reset the map
    Ledger, // Leaderboard
}

#[derive(Event)]
pub struct ResetMapEvent;

#[derive(Event)]
pub struct ShowLedgerEvent(pub bool);

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut reset_events: EventWriter<ResetMapEvent>,
    mut ledger_events: EventWriter<ShowLedgerEvent>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
    state: Res<State<AppState>>,
) {
    println!("state: {:?}", state.get());
    for (interaction, mut color, mut border_color, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse_buttons.clear_just_pressed(MouseButton::Left);
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                match menu_button_action {
                    MenuButtonAction::Reset => {
                        reset_events.send(ResetMapEvent);
                    }
                    MenuButtonAction::Ledger if state.get().to_owned() == AppState::Ledger => {
                        dbg!("CLICKED LEDGER");
                        ledger_events.send(ShowLedgerEvent(false));
                    }
                    MenuButtonAction::Ledger => {
                        dbg!("CLICKED LEDGER");
                        ledger_events.send(ShowLedgerEvent(true));
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None if state.get().to_owned() == AppState::Ledger => {
                if *menu_button_action == MenuButtonAction::Ledger {
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::RED;
                }
                dbg!("NONE IN LEDGER");
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Reset button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(45.0),
                            border: UiRect::all(Val::Px(3.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Reset,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset",
                        TextStyle {
                            font: asset_server.load("PoetsenOne-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            // Ledger button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(45.0),
                            border: UiRect::all(Val::Px(3.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Ledger,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Ledger",
                        TextStyle {
                            font: asset_server.load("PoetsenOne-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    // next_state.set(AppState::Build);
}
