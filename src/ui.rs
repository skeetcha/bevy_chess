use crate::{board::*, pieces::*};
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

// Initialize UiCamera and text
fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 2,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
    commands.spawn((
        TextBundle::from_section(
            "Next Move: White", TextStyle {
                font: font,
                font_size: 40.0,
                color: Color::rgb(0.8, 0.8, 0.8)
            }
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
            ..default()
        }),
        NextMoveText
    ));
}

fn next_move_text_update(mut _commands: Commands, turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    if !turn.is_changed() {
        return;
    }

    for (mut text,  _tag) in query.iter_mut() {
        text.sections[0].value = format!("Next move: {}", match turn.0 {
            PieceColor::White => "White",
            PieceColor::Black => "Black"
        });
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_next_move_text)
            .add_system(next_move_text_update);
    }
}