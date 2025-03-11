use crate::{
    player_button::create_buttons,
    world::{Status, StatusUI, TurnCounter},
};
use bevy::{
    color::palettes::css::{BLACK, RED, WHITE},
    prelude::*,
};

pub fn setup(mut commands: Commands) {
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
                if button_validation(&status.0, &text.0) {
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
                if button_validation(&status.0, &text.0) {
                    border_color.0 = WHITE.into();
                } else {
                    border_color.0 = RED.into();
                }
            }
            Interaction::None => *border_color = BLACK.into(),
        }
    }
}

fn button_validation(status: &String, text: &String) -> bool {
    !status.contains("win") && text == "_"
}

pub fn status_system(status: Res<Status>, mut text_query: Query<&mut Text, With<StatusUI>>) {
    let mut text = text_query.single_mut();
    text.0 = status.0.to_string();
}

pub fn win_system(
    mut status: ResMut<Status>,
    query: Query<(Entity, &Children), With<Button>>,
    text_query: Query<&mut Text>,
) {
    let mut board = Vec::new();
    for (e, c) in &query {
        let text = text_query.get(c[0]).unwrap();
        board.push((e.to_string(), text.0.clone()));
    }
    board.sort();
    if validate_tiles(&board[0].1, &board[1].1, &board[2].1) {
        status.0 = format!("{} wins", board[0].1);
    }
    if validate_tiles(&board[3].1, &board[4].1, &board[5].1) {
        status.0 = format!("{} wins", board[3].1);
    }
    if validate_tiles(&board[6].1, &board[7].1, &board[8].1) {
        status.0 = format!("{} wins", board[6].1);
    }
    if validate_tiles(&board[0].1, &board[3].1, &board[6].1) {
        status.0 = format!("{} wins", board[0].1);
    }
    if validate_tiles(&board[1].1, &board[4].1, &board[7].1) {
        status.0 = format!("{} wins", board[1].1);
    }
    if validate_tiles(&board[2].1, &board[5].1, &board[8].1) {
        status.0 = format!("{} wins", board[2].1);
    }
    if validate_tiles(&board[0].1, &board[4].1, &board[8].1) {
        status.0 = format!("{} wins", board[0].1);
    }
    if validate_tiles(&board[2].1, &board[4].1, &board[6].1) {
        status.0 = format!("{} wins", board[2].1);
    }
}

fn validate_tiles(x: &str, y: &str, z: &str) -> bool {
    x == y && x == z && x != "_"
}
