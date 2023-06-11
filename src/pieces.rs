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