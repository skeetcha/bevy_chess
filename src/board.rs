use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::pieces::*;

#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Add meshes and materials
	let mesh = meshes.add(Mesh::from(shape::Plane { size: 1., ..default() }));
	
	// Spawn 64 squares
	for i in 0..8 {
		for j in 0..8 {
			commands.spawn((PbrBundle {
				mesh: mesh.clone(),
				// Change material according to position to get alternating pattern
				material: if (i + j + 1) % 2 == 0 {
					materials.add(Color::rgb(1., 0.9, 0.9).into())
				} else {
					materials.add(Color::rgb(0., 0.1, 0.1).into())
				},
				transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
				..default()
			}, PickableBundle::default(),
			RaycastPickTarget::default(),
			Square {
				x: i,
				y: j
			},
			OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece)>| {
				if let Ok(square) = squares_query.get(event.target) {
					selected_square.entity = Some(event.target);

					if let Some(selected_piece_entity) = selected_piece.entity {
						let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| * piece).collect();

						if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
							if piece.is_move_valid((square.x, square.y), pieces_vec) {
								piece.x = square.x;
								piece.y = square.y;
							}
						}

						selected_square.entity = None;
						selected_piece.entity = None;
					} else {
						for (piece_entity, piece) in pieces_query.iter_mut() {
							if piece.x == square.x && piece.y == square.y {
								selected_piece.entity = Some(piece_entity);
								break;
							}
						}
					}
				}

				Bubble::Up
			}),
			OnPointer::<Over>::run_callback(|In(event): In<ListenedEvent<Over>>, mut hover_square: ResMut<HoverSquare>| {
				hover_square.entity = Some(event.target);
				Bubble::Up
			})));
		}
	}
}

#[derive(Default, Resource)]
struct SelectedSquare {
    entity: Option<Entity>
}

#[derive(Default, Resource)]
struct HoverSquare {
	entity: Option<Entity>
}

#[derive(Default, Resource)]
struct SelectedPiece {
	entity: Option<Entity>
}

fn color_squares(selected_square: Res<SelectedSquare>, hover_square: Res<HoverSquare>, mut materials: ResMut<Assets<StandardMaterial>>, query: Query<(Entity, &Square, &Handle<StandardMaterial>)>) {
	for (entity, square, material_handle) in query.iter() {
		let material = materials.get_mut(material_handle).unwrap();

		material.base_color = if Some(entity) == hover_square.entity {
			Color::rgb(0.8, 0.3, 0.3)
		} else if Some(entity) == selected_square.entity {
			Color::rgb(0.9, 0.1, 0.1)
		} else if square.is_white() {
			Color::rgb(1., 0.9, 0.9)
		} else {
			Color::rgb(0., 0.1, 0.1)
		};
	}
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<SelectedSquare>()
			.init_resource::<HoverSquare>()
			.init_resource::<SelectedPiece>()
			.add_startup_system(create_board)
			.add_system(color_squares);
	}
}