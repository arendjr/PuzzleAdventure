use bevy::prelude::*;

use crate::{constants::*, fonts::Fonts, GameEvent};

#[derive(Component)]
pub struct Menu;

#[derive(Resource)]
pub struct MenuState {
    pub is_open: bool,
    selected_button: MenuButton,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            is_open: true,
            selected_button: MenuButton::Start,
        }
    }
}

impl MenuState {
    fn move_selected_button(&mut self, delta: isize) {
        let num_buttons = MenuButton::__Last as isize;
        let selected_button_index =
            (self.selected_button as isize + num_buttons + delta) % num_buttons;
        self.selected_button = MenuButton::from_index(selected_button_index);
    }
}

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum MenuButton {
    Start,
    Editor,
    OtherGames,
    Exit,
    __Last,
}

impl MenuButton {
    fn from_index(index: isize) -> Self {
        match index {
            0 => Self::Start,
            1 => Self::Editor,
            2 => Self::OtherGames,
            3 => Self::Exit,
            _ => unreachable!(),
        }
    }
}

pub fn setup_menu(commands: &mut Commands, fonts: &Fonts) {
    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(500.),
                    height: Val::Px(400.),
                    border: UiRect::all(Val::Px(2.)),
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Auto),
                    row_gap: Val::Px(40.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: GRAY_BACKGROUND.into(),
                border_color: RED.into(),
                z_index: ZIndex::Global(100),
                ..Default::default()
            },
        ))
        .with_children(|cb| {
            cb.spawn(MenuButtonBundle::new(MenuButton::Start))
                .with_children(|cb| MenuButtonBundle::populate(cb, "Start", fonts));
            cb.spawn(MenuButtonBundle::new(MenuButton::Editor))
                .with_children(|cb| MenuButtonBundle::populate(cb, "Level Editor", fonts));
            cb.spawn(MenuButtonBundle::new(MenuButton::OtherGames))
                .with_children(|cb| MenuButtonBundle::populate(cb, "Other Games", fonts));
            cb.spawn(MenuButtonBundle::new(MenuButton::Exit))
                .with_children(|cb| MenuButtonBundle::populate(cb, "Exit", fonts));
        });
}

pub fn render_menu(
    mut menu_query: Query<&mut Style, With<Menu>>,
    mut button_query: Query<(&MenuButton, &mut BackgroundColor)>,
    menu_state: Res<MenuState>,
) {
    if !menu_state.is_changed() {
        return;
    }

    let mut menu_style = menu_query
        .get_single_mut()
        .expect("there must be a single menu");
    menu_style.display = if menu_state.is_open {
        Display::Flex
    } else {
        Display::None
    };

    for (menu_button, mut background_color) in &mut button_query {
        *background_color = if menu_button == &menu_state.selected_button {
            RED
        } else {
            BLUE
        }
        .into();
    }
}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    button: ButtonBundle,
}

impl MenuButtonBundle {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Self {
                button: ButtonBundle {
                    background_color: BLUE.into(),
                    style: Style {
                        height: Val::Px(60.),
                        width: Val::Px(300.),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        )
    }

    pub fn populate(cb: &mut ChildBuilder, text: impl Into<String>, fonts: &Fonts) {
        cb.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: fonts.poppins_light.clone(),
                    font_size: 40.,
                    color: WHITE,
                },
            ),
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn on_menu_keyboard_input(
    mut events: EventWriter<GameEvent>,
    mut menu_state: ResMut<MenuState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => menu_state.move_selected_button(-1),
            ArrowDown => menu_state.move_selected_button(1),
            Enter | Space => {
                handle_button_press(events, menu_state);
                return;
            }
            Escape => {
                events.send(GameEvent::Exit);
            }

            _ => continue,
        };
    }
}

pub fn on_menu_interaction_input(
    events: EventWriter<GameEvent>,
    button_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (interaction, menu_button) in &button_query {
        match *interaction {
            Interaction::Pressed => {
                menu_state.selected_button = *menu_button;
                handle_button_press(events, menu_state);
                return;
            }
            Interaction::Hovered => {
                menu_state.selected_button = *menu_button;
            }
            Interaction::None => {}
        }
    }
}

fn handle_button_press(mut events: EventWriter<GameEvent>, mut menu_state: ResMut<MenuState>) {
    match menu_state.selected_button {
        MenuButton::Start => {
            events.send(GameEvent::LoadRelativeLevel(0));
            menu_state.is_open = false;
        }
        MenuButton::Editor => {
            events.send(GameEvent::LoadRelativeLevel(0));
            events.send(GameEvent::ToggleEditor);
            menu_state.is_open = false;
        }
        MenuButton::OtherGames => { /* TODO */ }
        MenuButton::Exit => {
            events.send(GameEvent::Exit);
        }
        MenuButton::__Last => unreachable!(),
    }
}
