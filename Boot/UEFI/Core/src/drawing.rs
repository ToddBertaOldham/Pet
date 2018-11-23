// *************************************************************************
// drawing.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#[derive(Copy, Clone)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color {
    pub const WHITE : Color = Color { r : 1.0, g : 1.0, b : 1.0, a : 1.0 };
    pub const BLACK : Color = Color { r : 0.0, g : 0.0, b : 0.0, a : 0.0 };
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32
}