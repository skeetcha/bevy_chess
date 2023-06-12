mod pieces;
mod board;

use pieces::*;
use board::*;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
	App::new()
		.insert_resource(Msaa::Sample4)
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Chess!".into(),
				resolution: (800., 800.).into(),
				..default()
			}),
			..default()
		}))
		.add_plugins(DefaultPickingPlugins
			.build()
			.disable::<DebugPickingPlugin>()
		)
		.add_plugin(BoardPlugin)
		.add_startup_system(setup)
		.add_startup_system(create_pieces)
		.run();
}

fn setup(mut commands: Commands) {
	// Camera
	commands.spawn((Camera3dBundle {
		transform: Transform::from_matrix(Mat4::from_rotation_translation(
			Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
			Vec3::new(-7., 20., 4.)
		)),
		..default()
	}, RaycastPickCamera::default()));

	// Light
	commands.spawn(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
		..default()
	});
}

fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Load all the meshes
	let king_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
	let king_cross_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
	let pawn_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
	let knight_1_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
	let knight_2_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
	let rook_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
	let bishop_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
	let queen_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

	// Add some materials
	let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
	let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

	spawn_rook(&mut commands, white_material.clone(), PieceColor::White, rook_handle.clone(), (0, 0));
	spawn_knight(&mut commands, white_material.clone(), PieceColor::White, knight_1_handle.clone(), knight_2_handle.clone(), (0, 1));
	spawn_bishop(&mut commands, white_material.clone(), PieceColor::White, bishop_handle.clone(), (0, 2));
	spawn_queen(&mut commands, white_material.clone(), PieceColor::White, queen_handle.clone(), (0, 3));
	spawn_king(&mut commands, white_material.clone(), PieceColor::White, king_handle.clone(), king_cross_handle.clone(), (0, 4));
	spawn_bishop(&mut commands, white_material.clone(), PieceColor::White, bishop_handle.clone(), (0, 5));
	spawn_knight(&mut commands, white_material.clone(), PieceColor::White, knight_1_handle.clone(), knight_2_handle.clone(), (0, 6));
	spawn_rook(&mut commands, white_material.clone(), PieceColor::White, rook_handle.clone(), (0, 7));

	for i in 0..8 {
		spawn_pawn(&mut commands, white_material.clone(), PieceColor::White, pawn_handle.clone(), (1, i));
	}

	spawn_rook(&mut commands, black_material.clone(), PieceColor::Black, rook_handle.clone(), (7, 0));
	spawn_knight(&mut commands, black_material.clone(), PieceColor::Black, knight_1_handle.clone(), knight_2_handle.clone(), (7, 1));
	spawn_bishop(&mut commands, black_material.clone(), PieceColor::Black, bishop_handle.clone(), (7, 2));
	spawn_queen(&mut commands, black_material.clone(), PieceColor::Black, queen_handle.clone(), (7, 3));
	spawn_king(&mut commands, black_material.clone(), PieceColor::Black, king_handle.clone(), king_cross_handle.clone(), (7, 4));
	spawn_bishop(&mut commands, black_material.clone(), PieceColor::Black, bishop_handle.clone(), (7, 5));
	spawn_knight(&mut commands, black_material.clone(), PieceColor::Black, knight_1_handle.clone(), knight_2_handle.clone(), (7, 6));
	spawn_rook(&mut commands, black_material.clone(), PieceColor::Black, rook_handle.clone(), (7, 7));

	for i in 0..8 {
		spawn_pawn(&mut commands, black_material.clone(), PieceColor::Black, pawn_handle.clone(), (6, i));
	}
}