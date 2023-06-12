use bevy::prelude::*;

pub fn spawn_king(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, mesh_cross: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::King,
        x: position.0,
        y: position.1
    }))
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

pub fn spawn_knight(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh_1: Handle<Mesh>, mesh_2: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::Knight,
        x: position.0,
        y: position.1
    }))
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

pub fn spawn_queen(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::Queen,
        x: position.0,
        y: position.1
    }))
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., -0.95)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_bishop(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::Bishop,
        x: position.0,
        y: position.1
    }))
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.1, 0., 0.)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

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
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.1, 0., 1.8)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

pub fn spawn_pawn(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    commands.spawn((PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..default()
    },
    Piece {
        color: piece_color,
        piece_type: PieceType::Pawn,
        x: position.0,
        y: position.1
    }))
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh,
            material: material,
            transform: Transform::from_translation(Vec3::new(-0.2, 0., 2.6)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..default()
        });
    });
}

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