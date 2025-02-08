use crate::{
    player_button::create_buttons,
    world::{Status, StatusUI, TurnCounter},
};
use bevy::{
    app::{App, Startup, Update},
    color::{
        palettes::css::{BLACK, RED, WHITE},
        Color,
    },
    core_pipeline::core_2d::Camera2d,
    ecs::{
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children},
    text::{TextColor, TextFont},
    ui::{
        widget::{Button, Text},
        AlignItems, BorderColor, FlexDirection, Interaction, JustifyContent, Node, Val,
    },
    utils::default,
    winit::WinitSettings,
    DefaultPlugins,
};

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, status_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<TurnCounter>();
    commands.init_resource::<Status>();
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(create_buttons)
        .with_child((
            StatusUI,
            Text::new("O's turn"),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}

pub fn button_system(
    mut turn: ResMut<TurnCounter>,
    mut status: ResMut<Status>,
    mut button_query: Query<
        (&Interaction, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut border_color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if validate_button(&text.0) {
                    if turn.0 % 2 == 0 {
                        text.0 = "X".to_string();
                        status.0 = "O's turn".to_string();
                    } else {
                        text.0 = "O".to_string();
                        status.0 = "X's turn".to_string();
                    }
                    turn.0 += 1;
                }
            }
            Interaction::Hovered => {
                if !validate_button(&text.0) {
                    border_color.0 = RED.into();
                } else {
                    border_color.0 = WHITE.into();
                }
            }
            Interaction::None => *border_color = BLACK.into(),
        }
    }
}

fn validate_button(text: &String) -> bool {
    text == "_"
}

pub fn status_system(status: Res<Status>, mut test_query: Query<&mut Text, With<StatusUI>>) {
    let mut text = test_query.single_mut();
    text.0 = status.0.to_string();
}
