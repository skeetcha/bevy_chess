use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
			OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut selected_square: ResMut<SelectedSquare>| {
				selected_square.entity = Some(event.target);
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
			.add_startup_system(create_board)
			.add_system(color_squares);
	}
}