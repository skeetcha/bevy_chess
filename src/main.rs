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
		.add_startup_system(setup)
		.add_startup_system(create_board)
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
			}));
		}
	}
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

	spawn_rook(&mut commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 0.));
	spawn_knight(&mut commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 1.));
	spawn_bishop(&mut commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 2.));
	spawn_queen(&mut commands, white_material.clone(), queen_handle.clone(), Vec3::new(0., 0., 3.));
	spawn_king(&mut commands, white_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(0., 0., 4.));
	spawn_bishop(&mut commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 5.));
	spawn_knight(&mut commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 6.));
	spawn_rook(&mut commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 7.));

	for i in 0..8 {
		spawn_pawn(&mut commands, white_material.clone(), pawn_handle.clone(), Vec3::new(1., 0., i as f32));
	}

	spawn_rook(&mut commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 0.));
	spawn_knight(&mut commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 1.));
	spawn_bishop(&mut commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 2.));
	spawn_queen(&mut commands, black_material.clone(), queen_handle.clone(), Vec3::new(7., 0., 3.));
	spawn_king(&mut commands, black_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(7., 0., 4.));
	spawn_bishop(&mut commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 5.));
	spawn_knight(&mut commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 6.));
	spawn_rook(&mut commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 7.));

	for i in 0..8 {
		spawn_pawn(&mut commands, black_material.clone(), pawn_handle.clone(), Vec3::new(6., 0., i as f32));
	}
}