use bevy::{prelude::*, app::AppExit, ecs::event::{EventReader, EventWriter}};
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

fn select_square(In(event): In<ListenedEvent<Click>>, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, squares_query: Query<&Square>) -> Bubble {
	if let Ok(_) = squares_query.get(event.target) {
		selected_square.entity = Some(event.target);
	} else {
		selected_square.entity = None;
		selected_piece.entity = None;
	}

	Bubble::Up
}

fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, materials: Res<SquareMaterials>) {
	// Add meshes and materials
	let mesh = meshes.add(Mesh::from(shape::Plane { size: 1., ..default() }));
	
	// Spawn 64 squares
	for i in 0..8 {
		for j in 0..8 {
			commands.spawn((PbrBundle {
				mesh: mesh.clone(),
				// Change material according to position to get alternating pattern
				material: if (i + j + 1) % 2 == 0 {
					materials.white_color.clone()
				} else {
					materials.black_color.clone()
				},
				transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
				..default()
			}, PickableBundle::default(),
			RaycastPickTarget::default(),
			Square {
				x: i,
				y: j
			},
			OnPointer::<Click>::run_callback(select_square),
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

fn color_squares(selected_square: Res<SelectedSquare>, hover_square: Res<HoverSquare>, materials: Res<SquareMaterials>, mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>) {
	for (entity, square, mut material) in query.iter_mut() {
		*material = if Some(entity) == hover_square.entity {
			materials.highlight_color.clone()
		} else if Some(entity) == selected_square.entity {
			materials.selected_color.clone()
		} else if square.is_white() {
			materials.white_color.clone()
		} else {
			materials.black_color.clone()
		};
	}
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<SelectedSquare>()
			.init_resource::<HoverSquare>()
			.init_resource::<SelectedPiece>()
			.init_resource::<PlayerTurn>()
			.init_resource::<SquareMaterials>()
			.add_event::<ResetSelectedEvent>()
			.add_startup_system(create_board)
			.add_system(color_squares)
			.add_system(select_piece)
			.add_system(move_piece.before(select_piece))
			.add_system(reset_selected)
			.add_system(despawn_taken_pieces);
	}
}

#[derive(Resource)]
pub struct PlayerTurn(pub PieceColor);

impl Default for PlayerTurn {
	fn default() -> Self {
		Self(PieceColor::White)
	}
}

impl PlayerTurn {
	fn change(&mut self) {
		self.0 = match self.0 {
			PieceColor::White => PieceColor::Black,
			PieceColor::Black => PieceColor::White		
		}
	}
}

fn select_piece(selected_square: Res<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, turn: Res<PlayerTurn>, squares_query: Query<&Square>, pieces_query: Query<(Entity, &Piece)>) {
	if !selected_square.is_changed() {
		return;
	}

	let square_entity = if let Some(entity) = selected_square.entity {
		entity
	} else {
		return;
	};

	let square = if let Ok(square) = squares_query.get(square_entity) {
		square
	} else {
		return;
	};

	if selected_piece.entity.is_none() {
		// Select the piece in the currently selected square
		for (piece_entity, piece) in pieces_query.iter() {
			if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
				// piece_entity is now the entity in the same square
				selected_piece.entity = Some(piece_entity);
				break;
			}
		}
	}
}

fn move_piece(mut commands: Commands, selected_square: Res<SelectedSquare>, selected_piece: Res<SelectedPiece>, mut turn: ResMut<PlayerTurn>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece)>, mut reset_selected_event: EventWriter<ResetSelectedEvent>) {
	if !selected_square.is_changed() {
		return;
	}
	
	let square_entity = if let Some(entity) = selected_square.entity {
		entity
	} else {
		return;
	};

	let square = if let Ok(square) = squares_query.get(square_entity) {
		square
	} else {
		return;
	};

	if let Some(selected_piece_entity) = selected_piece.entity {
		let pieces_vec = pieces_query
			.iter_mut()
			.map(|(_, piece)| *piece)
			.collect::<Vec<Piece>>();

		let pieces_entity_vec = pieces_query
			.iter_mut()
			.map(|(entity, piece)| (entity, *piece))
			.collect::<Vec<(Entity, Piece)>>();

		// Move the selected piece to the selected square
		let mut piece = if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
			piece
		} else {
			return;
		};

		if piece.is_move_valid((square.x, square.y), pieces_vec) {
			// Check if a piece of ther opposite color exists in this square and despawn it
			for (other_entity, other_piece) in pieces_entity_vec {
				if other_piece.x == square.x && other_piece.y == square.y && other_piece.color != piece.color {
					// Mark the piece as taken
					commands.entity(other_entity).insert(Taken);
				}
			}

			// Move piece
			piece.x = square.x;
			piece.y = square.y;

			// Change turn
			turn.change();
		}

		reset_selected_event.send(ResetSelectedEvent);
	}
}

struct ResetSelectedEvent;

fn reset_selected(mut event_reader: EventReader<ResetSelectedEvent>, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>) {
	for _event in event_reader.iter() {
		selected_square.entity = None;
		selected_piece.entity = None;
	}
}

#[derive(Component)]
struct Taken;

fn despawn_taken_pieces(mut commands: Commands, mut app_exit_events: EventWriter<AppExit>, query: Query<(Entity, &Piece, &Taken)>) {
	for (entity, piece, _taken) in query.iter() {
		// If the king is taken, we should exit
		if piece.piece_type == PieceType::King {
			println!("{} won! Thanks for playing!", match piece.color {
				PieceColor::White => "Black",
				PieceColor::Black => "White"
			});

			app_exit_events.send(AppExit);
		}

		// Despawn piece and children
		commands.entity(entity).despawn_recursive();
	}
}

#[derive(Resource)]
struct SquareMaterials {
	highlight_color: Handle<StandardMaterial>,
	selected_color: Handle<StandardMaterial>,
	black_color: Handle<StandardMaterial>,
	white_color: Handle<StandardMaterial>
}

impl FromWorld for SquareMaterials {
	fn from_world(world: &mut World) -> Self {
		let world = world.cell();
		let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();

		SquareMaterials {
			highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
			selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
			black_color: materials.add(Color::rgb(0., 0.1, 0.1).into()),
			white_color: materials.add(Color::rgb(1., 0.9, 0.9).into())
		}
	}
}