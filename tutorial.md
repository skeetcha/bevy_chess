# Chess game in Rust using Bevy

Originally posted [here](https://caballerocoll.com/blog/bevy-chess-tutorial/) and updated for Bevy 0.10.1.

[Bevy](https://bevyengine.org/) is a data-driven game engine built in Rust. It's really straight forward to use, and a joy to work with.

In this tutorial we're going to use Bevy to make Chess, so if you've been meaning to start playing around with Bevy, this is for you!

The prerequisites for this tutorial are to have a basic-intermediate understanding of Rust, knowledge of the rules of Chess, and to be familiar with the concept of Entity Component System (ECS). If you don't know much about it, I recommend you read the [Wikipedia](https://en.wikipedia.org/wiki/Entity_component_system) page.

If you have any doubts, check out the [Bevy book](https://bevyengine.org/learn/book/introduction/), look through the [examples](https://github.com/bevyengine/bevy/tree/master/examples), or join the [Official Discord](https://discord.gg/gMUk5Ph) to ask questions!

~~This tutorial was made for Bevy 0.3, but it'll be updated to new versions when they come out. The tutorial is currently up to date with version 0.4!~~

If you don't care about the steps, and just want to see the code, [here](https://github.com/guimcaballero/bevy_chess) is the repository.

Last thing before we start, showing off Bevy concepts has a higher priority than having "good code" in this tutorial, so we'll be doing some things in an awkward way that allows us to introduce more concepts, like for example using parenting in cases where it's not really needed.

# Creating a project

As with any other Rust project, we will start by running `cargo new bevy_chess` and `cd bevy_chess`. This will create an empty project. The first step will be to add Bevy as a dependency by running `cargo add bevy`.

# Fast Compiles

A very important part of game development (as with most development) is iterating and seeing the results of our changes. Rust's long compilation times can throw a wrench on that, so as the [Bevy book](https://bevyengine.org/learn/book/getting-started/setup/) says, it's really recommended to enable Fast Compiles. This is an optional step, but it doesn't take too much time to set up and will help you out on the long run. To accomplish that, we need a couple things:

1. **LLD Linker:** The normal linker is a bit slow, so we can swap it out for the LLD Linker to get a speedup:

    a. **Ubuntu:** `sudo apt-get install lld`
    b. **Arch:** `sudo pacman -S lld`
    c. **Windows:** `cargo install -f cargo-binutils` and `rustup component add llvm-tools-preview`
    d. **MacOS:** `brew install michaeleisel/zld/zld`

2. Enable nightly Rust for this project: `rustup toolchain install nightly` to install nightly, and `rustup override set nightly` on the project directory to enable it.

3. Copy the contents of [this file](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds) into bevy_chess/.cargo/config.

With that, fast compiles should be enabled! If you now execute cargo run, you should see all the dependencies installing. This first time will be slow, as everything needs to be downloaded, but all future compilations should be much faster. When it's done, you should see `Hello, world!` as output.

# Getting started

Now, getting some text as output is pretty cool, but better than that is to have a window open! So let's work on that. Let's remove all the contents of `main.rs` and replace them with the following:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

After `cargo run`, you should see an empty window like the following open:

![https://caballerocoll.com/images/bevy_empty_window.png]

Let's explain what we did this step. `App::new()` creates a new `AppBuilder`, which has methods like `add_plugins`, `add_system` and `add_startup_system`, which we will be using to register our Systems and Plugins. Plugins are collections of App logic and configuration, mostly used to register systems and initialize Resources. We'll get to what resources are later, and we'll also create some of our own Plugins.

With `.add_plugins(DefaultPlugins)` we're adding all of the default Bevy plugins, which include things like `WindowPlugin`, `InputPlugin`, and `TransformPlugin`. Those provide most of the features we expect from a game engine. Bevy's modularity allows us to enable only the parts that we want to use. For our game, we'll enable the defaults.

# Changing Window settings

The window we opened is great, but we want to be able to change some stuff, like the title or the size. For that we will change the settings on the `WindowPlugin`.

We'll change the `main` function to the following:

```rust
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
}
```

The first resource we're adding, `Msaa`, is for the future. It's setting up antialiasing for our game. After that we change some settings in `WindowPlugin`, setting the title, width, and height. Here you can check all the other properties you can change. An important thing to note, is that this resource has to be set up before adding the default plugins, otherwise they won't work.

You should now see a bigger window with "Chess!" as the title. Everything is still empty though, so let's change that!

# Adding a camera and a plane

Now we need something to display on the screen. To achieve that, we'll create our first startup system. Startup systems are like normal systems (which we'll use later), but they only run once at the start of the game. These work great for creating entities, setting resources, or any other thing you might want to do at the start of the game.

```rust
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0, ..default() })),
		material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
		transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
		..default()
	});

	// Camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_matrix(Mat4::from_rotation_translation(
			Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
			Vec3::new(-7., 20., 4.)
		)),
		..default()
	});

	// Light
	commands.spawn(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
		..default()
	});
}
```

`setup` will be a startup system, which takes `commands`, and the `meshes` and `materials` resources. Commands is used to spawn and despawn entities, while `meshes` and `materials` are used to register meshes and materials.

We use commands to spawn a Plane, a Camera and a Light, by using `PbrBundle`, `Camera3dBundle`, and `PointLightBundle`, which are Bundles. Bundles are just an easy to use collection of Components. We can override the bundle properties, like for example the `transform`, which allows us to move the entities around or to change their rotation, or the mesh and material.

If you now run `cargo run`, you'll see that nothing has changed! This is because we haven't registered our `setup` system. Let's do that with `add_startup_system`:

```rust
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
		.add_startup_system(setup)
		.run();
}
```

With `add_startup_system` we're adding `setup` as a startup system, which will only run once at the beginning of the game. If we use `add_system` instead, it will run every frame.

You can now run the game to see a flat plane from a camera looking slightly down:

![https://caballerocoll.com/images/bevy_flat_plane.png]

# Making a game board

We now have a very boring board, let's change that! We'll change the current plane to be a grid of squares of alternating colors.

For that, we'll first split the current `setup` system into two:

```rust
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
		.add_startup_system(setup)
		.add_startup_system(create_board)
		.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_matrix(Mat4::from_rotation_translation(
			Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
			Vec3::new(-7., 20., 4.)
		)),
		..default()
	});

	// Light
	commands.spawn(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
		..default()
	});
}

fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

}
```

Before implementing `create_board`, let's take a side-trip through Asset Town. The `meshes.add(Mesh::from(shape::Plane { size: 8.0, ..default() }))` line we used before registers a plane mesh to the `Assets<Mesh>` resource, and returns a `Handle<Mesh>`, which is what `PbrBundle` uses. Bevy will then use that handle to get the actual mesh and render it.

This is great, because if we want to create multiple entities with the same `Mesh`, we can just provide them the same handle and just add the Mesh once. All of this also applies to `materials` and `StandardMaterial`.

We will add two materials, a white and a black one, and one Plane mesh, to create a plane per square in the board.

```rust
fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Add meshes and materials
	let mesh = meshes.add(Mesh::from(shape::Plane { size: 1., ..default() }));
	let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
	let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

	// Spawn 64 squares
	for i in 0..8 {
		for j in 0..8 {
			commands.spawn(PbrBundle {
				mesh: mesh.clone(),
				// Change material according to position to get alternating pattern
				material: if (i + j + 1) % 2 == 0 {
					white_material.clone()
				} else {
					black_material.clone()
				},
				transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
				..default()
			});
		}
	}
}
```

We use two for loops to spawn the 64 squares, and we use `if (i + j + 1) % 2 == 0` to generate the alternating pattern of colors. Here's the result:

![https://caballerocoll.com/images/bevy_chess_board.png]

Note: We could have used a single Plane with a Texture to make the pattern instead of making different squares, but doing it this way will help us later down the line, when we want to select pieces and squares for movements.

# Adding pieces

Every chess game needs some pieces to play with, so let's go ahead and get some models to play with. We'll use GLTF models, which for the most part work great in Bevy. Asset management is still a bit work in progress though, so some things might not work yet.

I got some Chess piece models from [Sketchfab](https://sketchfab.com/3d-models/chess-kit-94f58cabfc0044acb83c28f5b70c79f9). Sketchfab does some weird stuff with the autoconversion to GLTF, so I downloaded the models in OBJ, the original format, and used [AnyConv](https://anyconv.com/obj-to-glb-converter/) to convert them to GLB, which is the binary format of GLTF. You can do this yourself, or you can go over to the [repo](https://github.com/guimcaballero/bevy_chess/tree/main/assets/models/chess_kit) where you can download the file.

Now that we have the models, we need a way to load them onto Bevy. For that we'll use the `AssetServer` resource. It provides a `load()` function to which we can pass the path of the asset we want to load. In our case, we have the .glb file in `assets/models/chess_kit/pieces.glb`. We can get each of the models in the file like so:

```rust
fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Load all the meshes
	let king_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
	let king_cross_Handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
	let pawn_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
	let knight_1_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
	let knight_2_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
	let rook_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
	let bishop_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
	let queen_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

	// Add some materials
	let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
	let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());
}
```

Notice that load assumes that the path you're passing is inside the `assets` folder.

The `#Mesh0/Primitive0` part let's us select which of the meshes we want from the GLTF file. The pieces are separated into different meshes, and some of the pieces are separated into two meshes, like the King and the Knight.

We've also added a white and a black material for the pieces, which we will now spawn. As some of the meshes have a bit of a translation, we'll use a parent entity to keep the actual position, and use a child to keep the mesh. This will also help us combine the meshes for the King and the Knight.

The best way to solve this would be to go into a 3D editing software and fix the models so each of them is at the origin and is just a single mesh, but doing it this way gives me an excuse to talk about parenting.

```rust
fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Load all the meshes
	[...]

	// Add some materials
	[...]

	// Spawn parent entity
	commands.spawn(PbrBundle {
		transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
		..default()
	})
	// Add children to parent
	.with_children(|parent| {
		parent.spawn(PbrBundle {
			mesh: king_handle.clone(),
			material: white_material.clone(),
			transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
			..default()
		});
		parent.spawn(PbrBundle {
			mesh: king_cross_handle.clone(),
			material: white_material.clone(),
			transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
			..default()
		});
	});
}
```

Here we used the `with_children()` function to add two children to a parent entity. The function takes a closure, that in turn takes a `parent` parameter, which is similar to `commands`, and let's us spawn children. This two children are moved with respect to their parent, to compensate for the translation that the model has, and with scaling added to make them fit in a square.

Don't forget to add `create_pieces` as a startup system! If you run the game now you should see a single white King on it's square:

![https://caballerocoll.com/images/bevy_chess_king.png]

Great, we got pieces on our board! But if you check the previous code, it's more than 20 lines just to spawn a piece. This function is going to get really busy really soon if we don't break it up. Let's create a new `pieces.rs` file. We'll make separate functions to spawn each of the pieces:

```rust
use bevy::prelude::*;

pub fn spawn_king(commands: &mut Commands, material: Handle<StandardMaterial>, mesh: Handle<Mesh>, mesh_cross: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_cross,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., -1.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_knight(commands: &mut Commands, material: Handle<StandardMaterial>, mesh_1: Handle<Mesh>, mesh_2: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh_1,
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(-0.2, 0., 0.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_2,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., 0.9)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_queen(commands: &mut Commands, material: Handle<StandardMaterial>, mesh: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., -0.95)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_bishop(commands: &mut Commands, material: Handle<StandardMaterial>, mesh: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.1, 0., 0.)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_rook(commands: &mut Commands, material: Handle<StandardMaterial>, mesh: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.1, 0., 1.8)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_pawn(commands: &mut Commands, material: Handle<StandardMaterial>, mesh: Handle<Mesh>, position: Vec3) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., 2.6)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}
```

```rust
fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Load all the meshes
	[...]

	// Add some materials
	[...]

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
```

That's a lot, but with that out of the way, we now have all of our pieces on the board:

![https://caballerocoll.com/images/bevy_chess_all_pieces.png]

# Selection plugin

Cool, our board and our pieces are all set up, but we still need to implement all of the game logic.

The next step we're going to work in is selecting pieces. For that, we'll use the [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking/) library. It provides a simple API to select meshes in your game.

To use it first we need to follow the short instructions shown in the README. We'll run `cargo add bevy_mod_picking`, then import the library and register the `DefaultPickingPlugins` it provides:

```rust
mod pieces;
use pieces::*;

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
```

Next we need to add a component to the Camera object, to mark it as a pick source. For that, we'll use the `with()` function, which adds components to the previous Entity:

```rust
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
```

This way, our Camera entity will be spawned with all of the components in the `Camera3dBundle` bundle and the `RaycastPickCamera` component, and nothing will change for the `PointLightBundle` spawned under it.

We now just need to add the `PickableBundle` and `RaycastPickTarget` bundles to the entities we want to be able to select. In our case, we'll add it to the board squares:

```rust
fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	[...]
			commands.spawn((PbrBundle {
				mesh: mesh.clone(),
				// Change material according to position to get alternating pattern
				material: if (i + j + 1) % 2 == 0 {
					white_material.clone()
				} else {
					black_material.clone()
				},
				transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
				..default()
			}, PickableBundle::default(), RaycastPickableBundle::default()));
    [...]
}
```

If you now run the game, you should see a small green ball that follows your mouse all over the board, cool! Feel free to remove the `DebugPickingPlugin` when you want, it's just an easy way to check that everything is working correctly.

We should implement something to select squares, but first, notice that our `main.rs` file has been growing a lot, so it would be great to split it up a bit.

# Refactor

We'll do a couple quick adjustments to keep our code a bit cleaner. First we'll move `create_pieces` into `pieces.rs` and make it public, and we'll change all of the `spawn_[piece]` functions to private.

Next we'll create a new `board.rs` file and we'll move `create_board` there and make it public. We now just have to import it in `main.rs`, and we're ready to go.

# Adding components to board square

We're going to create our very own component now, so exciting! Let's go to `board.rs` and add the following:

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Square {
	pub x: u8,
	pub y: u8
}
```

That's it, that's our component. No boilerplate needed. We can now just add this to the squares like this:

```rust
fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	[...]
			commands.spawn((PbrBundle {
				mesh: mesh.clone(),
				// Change material according to position to get alternating pattern
				material: if (i + j + 1) % 2 == 0 {
					white_material.clone()
				} else {
					black_material.clone()
				},
				transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
				..default()
			}, PickableBundle::default(),
			RaycastPickTarget::default(),
			Square {
				x: i,
				y: j
			}));
	[...]
}
```

`Square` is a normal Rust struct, so we can do something like the following to add some functions which we can later use on the component:

```rust
impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}
```

We'll make a small change to help with selecting squares. In the way we're creating squares now, we are adding only two materials, and cloning the handles into each square. We'll change it to use one material per square, this way we can change a square's color individually. It's an easy change:

```rust
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
```

~~Note: In theory this should work by just creating 4 materials and replacing the handles in the squares, but there's a bug in Bevy that makes that not work. Update: This has now been fixed! In the [last section](#extra-steps) we'll rework this to use the slightly more efficient version.~~ Update: This has been reworked a bit to simply just pass the target of the event (whether it be the mouse hovering over an entity such as with `OnPointer::<Over>` or the mouse clicking on an entity as with `OnPointer::<Click>`).

Everything should look exactly the same. Let's create a resource to keep track of which square is currently selected as well as which square is being hovered over:

```rust
#[derive(Default, Resource)]
struct SelectedSquare {
    entity: Option<Entity>
}

#[derive(Default, Resource)]
struct HoverSquare {
	entity: Option<Entity>
}
```

Easy as that! We're deriving `Default` so that when the plugin is initialized it starts with a `None` value, but we could provide an initial value in case we wanted to, by implementing `FromResources`. You can see how to do that with [this](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/state.rs) Bevy example.

We'll now make a system that changes the color of the squares:

```rust
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
```

There's a lot of new stuff here, so let's unpack that. The system first gets the entity under the cursor using our `HoverSquare` resource, which gives us the entity.

`Query` is a tad more interesting, it provides us with an iterable of all the entities that have the Components we select, in this case `Entity` (which all entities have), `Square`, and `Handle<StandardMaterial>`. We can now iterate over it with `query.iter()`, which will provide us access to the components.

Note: In queries, components have to be references (i.e. have `&`), except `Entity`, which is used normally.

For each of the squares, we first get the actual material from the `Assets<StandardMaterial>` resource using `get_mut()`, and then we set the base color according to if it's hovered, selected, white or black, in that order. This way hovered squares get painted the hovered color even if they're selected.

Feel free to play around with the colors and change them to something else!

We still need a way to select squares, but before we do that, we're going to create our a plugin to keep things a bit more clean. In `board.rs` we're going to add the following, and change all of the systems to private:

```rust
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<SelectedSquare>()
			.init_resource::<HoverSquare>()
			.add_startup_system(create_board)
			.add_system(color_squares);
	}
}
```

`init_resource` is what takes care of initializing `SelectedSquare` and `HoverSquare`. If we don't add this, Bevy will complain when we try to use the resource. Now in `main.rs` we can add `BoardPlugin` like `DefaultPlugins` or `DefaultPickingPlugins`:

```rust
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
```

This way we don't have to keep track of which board systems exist from `main.rs`, and everything is self contained in `board.rs`.

If you run the game now, you should see the square under the cursor being highlighted by the color we selected, and when we click on the square, it should highlight with the other color we selected.

# Adding types of pieces

Currently, our pieces are just an empty parent object with a child that holds the mesh, and we have no way to distinguish between them. Let's create a `Piece` component to keep what piece it is.

```rust
#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(Clone, Copy, Component)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current Position
    pub x: u8,
    pub y: u8
}
```

We first add two enums with all the possibilities, and then declare the `Piece` component, which will have a `PieceColor`, `PieceType`, and the piece position. The reason that we have the position here again, and we don't just use the `Transform` component, is that we will want the pieces to move from one square to the next, and there will be times when the piece is halfway between two squares, and we want to know the one it's actually supposed to be on.

We're also going to change how we call the `spawn_[piece]` function calls to be like the following:

```rust
spawn_rook(commands, white_material.clone(), PieceColor::White, rook_handle.clone(), (0, 0));
```

And the definitions to be like:

```rust
pub fn spawn_rook(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::Rook,
        x: position.0,
        y: position.1
    }))
    .with_children(|parent| {
        [...]
    });
}
```

Great, now we spawn the pieces with the correct values set in the Pieces component. We should now be ready to implement movement.

# Movement

We're going to create a system that takes care of moving the pieces to the `x, y` coordinates on the `Piece` component. This way we just have to change those values, and the piece will start moving to it's new coordinates. This is pretty straight-forward:

```rust
fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
```

We'll also make a `PiecesPlugin`, very similar to the `BoardPlugin`:

```rust
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}
```

Remember to add this plugin in `main.rs`, and change `create_pieces` to private!

We now need to implement something to change the unit positions. We'll change the callback for the `Click` event to also move the pieces for now, and we'll refactor it into something neater later:

```rust
#[derive(Default, Resource)]
struct SelectedPiece {
	entity: Option<Entity>
}

fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	[...]
			OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece)>| {
				if let Ok(square) = squares_query.get(event.target) {
					selected_square.entity = Some(event.target);

					if let Some(selected_piece_entity) = selected_piece.entity {
						if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
							piece.x = square.x;
							piece.y = square.y;
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
			})
	[...]
}
```

We first added a new `SelectedPiece` resource, which will work like `SelectedSquare`, keeping track of the currently selected piece. If there is a selected piece and the player clicks another square, we want the piece to move there and then deselect the piece. If there is no piece selected, and the user clicks a square, we want to select the piece on that square, if there is one. ~~If the player clicked outside of the board, we deselect everything.~~ For that last part (the deselecting), I can't get it to work with `bevy_mod_picking`, so if anyone has any idea of how to get it to work, please feel free to submit a PR with the relevant code and I'll update this part of the tutorial.

This code is a bit confusing, so we'll rework it later. For now, it does what we want! Look at those pieces moving:

![https://caballerocoll.com/images/bevy_chess_pieces_moving.gif]

# Implementing available squares

We now need to limit what squares a piece can move to, because currently every piece can move anywhere. There's two approaches here. One of them is computing which squares are available and allowed for that piece, and checking if the move is one of those squares, and the other approach is to check that the move is a valid square (allowed and available). The first one is a bit more complicated, but it allows us to highlight all of the available squares, while the second is easier to do. (Actually the first one can be done with the second, just checking all 64 squares). We'll go with the second one, but feel free to implement the first one on your own!

Now, we'll add some helper functions first to make our work easier implementing validation. The first function we'll add just returns the color of a square if there is a piece, and returns `None` if it's empty:

```rust
fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
	for piece in pieces {
		if piece.x == pos.0 && piece.y == pos.1 {
			return Some(piece.color);
		}
	}

	None
}
```

This way we can make test for things like not moving into a piece of the same color, or allowing diagonal movement for pawns only when there's a piece of the opposite color. Let's also add a function to check if all the squares between two are empty. This will serve us for the movement of the Rook, Bishop and Queen:

```rust
fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
	// Same column
	if begin.0 == end.0 {
		for piece in pieces {
			if piece.x == begin.0 && ((piece.y > begin.1 && piece.y < end.1) || (piece.y > end.1 && piece.y < begin.1)) {
				return false;
			}
		}
	}

	// Same row
	if begin.1 == end.1 {
		for piece in pieces {
			if piece.y == begin.1 && ((piece.x > begin.0 && piece.x < end.0) || (piece.x > end.0 && piece.x < begin.0)) {
				return false;
			}
		}
	}

	// Diagonals
	let x_diff = (begin.0 as i8 - end.0 as i8).abs();
	let y_diff = (begin.1 as i8 - end.1 as i8).abs();

	if x_diff == y_diff {
		for i in 1..x_diff {
			let pos = if begin.0 < end.0 && begin.1 < end.1 {
				// left bottom - right top
				(begin.0 + i as u8, begin.1 + i as u8)
			} else if begin.0 < end.0 && begin.1 > end.1 {
				// left top - right bottom
				(begin.0 + i as u8, begin.1 - i as u8)
			} else if begin.0 > end.0 && begin.1 < end.1 {
				// right bottom - left top
				(begin.0 - i as u8, begin.1 + i as u8)
			} else {
				// begin.0 > end.0 && begin.1 > end.1
				// right top - left bottom
				(begin.0 - i as u8, begin.1 - i as u8)
			};

			if color_of_square(pos, pieces).is_some() {
				return false;
			}
		}
	}

	true
}
```

The way we implemented it works both in straight lines and in diagonals, but we could have split it into two separate functions if we wanted to.

The function first checks for a path in the same column, then in the same row, and then in the four diagonals. We can now implement a method to check if a move is valid for a certain `Piece`. We'll count taking a piece as a valid move, and we'll deal with the actual taking of the piece later. This function is a bit long, but here it is in it's entirety:

```rust
impl Piece {
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y == new_position.1))
                // Vertical
                || ((self.y as i8 - new_position.1 as i8).abs() == 1 && (self.x == new_position.0))
                // Diagnoal
                || ((self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y as i8 - new_position.1 as i8).abs() == 1)
            },
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                && ((self.x as i8 - new_position.0 as i8).abs() == (self.y as i8 - new_position.1 as i8).abs()
                    || ((self.x == new_position.0 && self.y != new_position.1)
                    || (self.y == new_position.1 && self.x != new_position.0)))
            },
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                && (self.x as i8 - new_position.0 as i8).abs() == (self.y as i8 - new_position.1 as i8).abs()
            },
            PieceType::Knight => {
                ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            },
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            },
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == 1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 sqauares
                    if self.x == 1 && new_position.0 as i8 - self.x as i8 == 2 && (self.y == new_position.1) && is_path_empty((self.x, self.y), new_position, &pieces) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == 1 && (self.y as i8 - new_position.1 as i8).abs() == 1 {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::Black) {
                            return true;
                        }
                    }
                } else {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == -1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 sqauares
                    if self.x == 6 && new_position.0 as i8 - self.x as i8 == -2 && (self.y == new_position.1) && is_path_empty((self.x, self.y), new_position, &pieces) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == -1 && (self.y as i8 - new_position.1 as i8).abs() == 1 {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::White) {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }
}
```

We first check that there is no piece of the same color there, and then we match on the type of piece, because each has a different movement and needs different logic.

Most pieces are relatively simple to check. For example, for the King we check three possibilities: same row and a movement of 1 vertically, same column and movement of 1 horizontally, or a diagonal movement (movement of 1 horizontally and movement of 1 vertically). With the Queen, Bishop, and Rook we also check that the path is empty.

Pawns are a bit more interesting. As they can only move in one direction, we separate the two colors, so that white can only go up and black can only go down. The movement is then split into three different possibilities: move forward one square, move two squares if it's on the initial square, and taking a piece in the diagonals.

We won't implement en passant or castling, although those shouldn't take too much time if you want to figure them out.

Finally, we just need to call the function before moving a piece to check that the move is valid:

```rust
fn create_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	[...]
						let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| * piece).collect();

						if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
							if piece.is_move_valid((square.x, square.y), pieces_vec) {
								piece.x = square.x;
								piece.y = square.y;
							}
						}
	[...]
}
```

Cool, now pieces can only do legal moves! But if you notice, when we move to a piece occupied by a piece of a different color, nothing happens and both pieces stand in the same place. Let' s solve that by implementing taking pieces.

# Taking pieces

This next chapter will be lighter than the past one, we just have to check the landing square for pieces of the opposite color, and despawn them if there are.

For that we first need to add `Commands` as a parameter to our closure as part of the `OnPointer::<Click>` event listener:

```rust
|In(event): In<ListenedEvent<Click>>, mut entity_commands: Commands, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece, &Children)>|
```

`Commands` has the `entity` function which takes an `Entity`, and returns `EntityCommands`. Through that, we can use the `despawn` function to despawn the entity.

A note about Commands, Resources, and Queries: they have to be in this order, otherwise the system won't work. If you try putting Resources before Commands, or Queries before any of the other two, Bevy will complain.

Here's the new function that also takes care of despawning a piece after it's taken:

```rust
OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut entity_commands: Commands, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece, &Children)>| {
	if let Ok(square) = squares_query.get(event.target) {
		selected_square.entity = Some(event.target);

		if let Some(selected_piece_entity) = selected_piece.entity {
			let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query.iter_mut().map(|(entity, piece, children)| {
				(
					entity,
					*piece,
					children.iter().map(|entity| *entity).collect()
				)
			}).collect();

			let pieces_vec = pieces_query.iter_mut().map(|(_, piece, _)| * piece).collect();

			if let Ok((_piece_entity, mut piece, _piece_children)) = pieces_query.get_mut(selected_piece_entity) {
				if piece.is_move_valid((square.x, square.y), pieces_vec) {
					for (other_entity, other_piece, _other_children) in pieces_entity_vec {
						if other_piece.x == square.x && other_piece.y == square.y && other_piece.color != piece.color {
							// Despawn piece
							entity_commands.entity(other_entity).despawn_recursive();
						}
					}

					// Move piece
					piece.x = square.x;
					piece.y = square.y;
				}
			}

			selected_square.entity = None;
			selected_piece.entity = None;
		} else {
			for (piece_entity, piece, _) in pieces_query.iter_mut() {
				if piece.x == square.x && piece.y == square.y {
					selected_piece.entity = Some(piece_entity);
					break;
				}
			}
		}
	}

	Bubble::Up
})
```

The first difference you'll see is that we added `&Children` to the Query. The `despawn` function won't despawn child entities automatically, so in a normal game we'd use `despawn_recursive`, which does, but this is a nice way to show how to access children entities from a system. When we refactor this function later we'll change it `despawn_recursive`.

`Children` is a vector of entities, so we can iterate over it and despawn the child entities.

We also added `pieces_entity_vec`, so the borrow checker doesn't complain about having borrowed `pieces_query` twice. Finally, we added a loop to check if there is any piece of the opposite color in that square, and if there is, we despawn it and it's children.

If you run the game now, you can take a piece and it will be despawned, cool!

# Turns

The last two things we need to do are turns, and ending the game when the King is taken. We won't implement checking for mates or anything else, but it also shouldn't be hard if you want to add it to `is_move_valid`.

For turns we'll create a resource that contains who is the next player to move, and we'll check if the selected piece is of that color. After moving, we'll switch the resource to the opposite color.

```rust
#[derive(Resource)]
struct PlayerTurn(PieceColor);

impl Default for PlayerTurn {
	fn default() -> Self {
		Self(PieceColor::White)
	}
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<SelectedSquare>()
			.init_resource::<HoverSquare>()
			.init_resource::<SelectedPiece>()
			.init_resource::<PlayerTurn>()
			.add_startup_system(create_board)
			.add_system(color_squares);
	}
}
```

We can now change the `OnPointer::<Click>` closure to check if the selected piece is of the correct color before selecting it:

```rust
OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut entity_commands: Commands, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, mut turn: ResMut<PlayerTurn>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece, &Children)>| {
	if let Ok(square) = squares_query.get(event.target) {
		selected_square.entity = Some(event.target);

		if let Some(selected_piece_entity) = selected_piece.entity {
			let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query.iter_mut().map(|(entity, piece, children)| {
				(
					entity,
					*piece,
					children.iter().map(|entity| *entity).collect()
				)
			}).collect();

			let pieces_vec = pieces_query.iter_mut().map(|(_, piece, _)| * piece).collect();

			if let Ok((_piece_entity, mut piece, _piece_children)) = pieces_query.get_mut(selected_piece_entity) {
				if piece.is_move_valid((square.x, square.y), pieces_vec) {
					for (other_entity, other_piece, _other_children) in pieces_entity_vec {
						if other_piece.x == square.x && other_piece.y == square.y && other_piece.color != piece.color {
							// Despawn piece
							entity_commands.entity(other_entity).despawn_recursive();
						}
					}

					// Move piece
					piece.x = square.x;
					piece.y = square.y;

					turn.0 = match turn.0 {
						PieceColor::White => PieceColor::Black,
						PieceColor::Black => PieceColor::White
					};
				}
			}

			selected_square.entity = None;
			selected_piece.entity = None;
		} else {
			for (piece_entity, piece, _) in pieces_query.iter_mut() {
				if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
					selected_piece.entity = Some(piece_entity);
					break;
				}
			}
		}
	}

	Bubble::Up
})
```

And with that, turns are done! If you play the game and try to move a Black piece at the start it won't work, but after moving a White you'll be able to.

We just need to end the game when the King is taken, and we'll be done with out minimal chess.

# Ending the game

We have to choose what to do when the King is taken. For this tutorial, we'll just exit the game and print the result to the console, but you can change it later to do something else, like start a new game, or maybe show some text on screen displaying the winner.

Exiting the game involves sending Events. We just need to send the `AppExit` event to `Events<AppExit>`, using the `send` method it provides.

```rust
OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>, mut entity_commands: Commands, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, mut turn: ResMut<PlayerTurn>, mut app_exit_events: ResMut<Events<AppExit>>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece, &Children)>| {
	if let Ok(square) = squares_query.get(event.target) {
		selected_square.entity = Some(event.target);

		if let Some(selected_piece_entity) = selected_piece.entity {
			let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query.iter_mut().map(|(entity, piece, children)| {
				(
					entity,
					*piece,
					children.iter().map(|entity| *entity).collect()
				)
			}).collect();

			let pieces_vec = pieces_query.iter_mut().map(|(_, piece, _)| * piece).collect();

			if let Ok((_piece_entity, mut piece, _piece_children)) = pieces_query.get_mut(selected_piece_entity) {
				if piece.is_move_valid((square.x, square.y), pieces_vec) {
					for (other_entity, other_piece, _other_children) in pieces_entity_vec {
						if other_piece.x == square.x && other_piece.y == square.y && other_piece.color != piece.color {
							if other_piece.piece_type == PieceType::King {
								// If the king is taken, we should exit
								println!("{} won! Thanks for playing!", match turn.0 {
									PieceColor::White => "White",
									PieceColor::Black => "Black"
								});
								app_exit_events.send(AppExit);
							}

							// Despawn pice
							entity_commands.entity(other_entity).despawn_recursive();
						}
					}

					// Move piece
					piece.x = square.x;
					piece.y = square.y;

					turn.0 = match turn.0 {
						PieceColor::White => PieceColor::Black,
						PieceColor::Black => PieceColor::White
					};
				}
			}

			selected_square.entity = None;
			selected_piece.entity = None;
		} else {
			for (piece_entity, piece, _) in pieces_query.iter_mut() {
				if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
					selected_piece.entity = Some(piece_entity);
					break;
				}
			}
		}
	}

	Bubble::Up
})
```

`AppExit` isn't included in `bevy::prelude`, so we need to change the use statement to include that. Then when we take a piece, we check it it's the King, and if it is we print a statement to the console and send the event. That's it, our game finishes when we take the King!

![https://caballerocoll.com/images/bevy_chess_finished.gif]

The main game logic is all done now, but we'll make some other changes to top it all off.