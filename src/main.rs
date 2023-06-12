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
			//.disable::<DebugPickingPlugin>()
		)
		.add_plugin(BoardPlugin)
		.add_plugin(PiecesPlugin)
		.add_startup_system(setup)
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