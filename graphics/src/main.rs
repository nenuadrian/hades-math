use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

use csv::ReaderBuilder;
use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::fs::File;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, animate_rotation, animate_scale),
        )
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = TextFont {
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;
    // 2d camera
    commands.spawn(Camera2d);
    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font_size: 35.0,
        ..default()
    };

    let matrix = ndarray::array![[1.23, 4.56, 7.89], [0.12, 3.45, 6.78], [9.01, 2.34, 5.67]];
    let start_x = -400.0; // Starting X position
    let start_y = 250.0; // Starting Y position
    let cell_width = 100.0; // Space between columns
    let cell_height = 50.0; // Space between rows
    for (row_index, row) in matrix.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            let x = start_x + (col_index as f32 * cell_width);
            let y = start_y - (row_index as f32 * cell_height);

            commands.spawn((
                Text2d::new(format!("{}", value)),
                slightly_smaller_text_font.clone(),
                Transform::from_translation(Vec3::new(x, y, 0.0)),
            ));
        }
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        let scale = (ops::sin(time.elapsed_secs()) + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}

// Simple Feedforward Neural Network
struct NeuralNetwork {
    w1: Array2<f32>,
    b1: Array1<f32>,
    w2: Array2<f32>,
    b2: Array1<f32>,
}

impl NeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        NeuralNetwork {
            w1: Array2::random((input_size, hidden_size), Uniform::new(-0.1, 0.1)),
            b1: Array1::zeros(hidden_size),
            w2: Array2::random((hidden_size, output_size), Uniform::new(-0.1, 0.1)),
            b2: Array1::zeros(output_size),
        }
    }

    fn relu(x: &Array1<f32>) -> Array1<f32> {
        x.mapv(|v| v.max(0.0))
    }

    fn relu_derivative(x: &Array1<f32>) -> Array1<f32> {
        x.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 })
    }

    fn softmax(x: &Array1<f32>) -> Array1<f32> {
        let max = x.mapv(|v| v.exp()).sum();
        x.mapv(|v| v.exp() / max)
    }

    fn train(&mut self, images: &Array2<f32>, labels: &Array1<u8>, epochs: usize, lr: f32) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for (x, &y) in images.outer_iter().zip(labels.iter()) {
                // Forward pass
                let z1 = x.dot(&self.w1) + &self.b1;
                let a1 = Self::relu(&z1);
                let z2 = a1.dot(&self.w2) + &self.b2;
                let a2 = Self::softmax(&z2);

                // Compute loss
                let mut y_true = Array1::zeros(a2.len());
                y_true[y as usize] = 1.0;
                let loss = -y_true
                    .iter()
                    .zip(a2.iter())
                    .map(|(&y, &a)| y * a.ln())
                    .sum::<f32>();
                total_loss += loss;

                // Backpropagation
                let dz2 = &a2 - &y_true;
                let dw2 = a1
                    .clone()
                    .insert_axis(ndarray::Axis(1))
                    .dot(&dz2.insert_axis(ndarray::Axis(0)));
                let db2 = dz2.clone();

                let dz1 = dz2.dot(&self.w2.t()) * Self::relu_derivative(&z1);
                let dw1 = x
                    .insert_axis(ndarray::Axis(1))
                    .dot(&dz1.insert_axis(ndarray::Axis(0)));
                let db1 = dz1;

                // Gradient descent
                self.w2 -= &(lr * dw2);
                self.b2 -= &(lr * db2);
                self.w1 -= &(lr * dw1);
                self.b1 -= &(lr * db1);
            }

            println!(
                "Epoch {}: Loss = {:.4}",
                epoch + 1,
                total_loss / images.len() as f32
            );
        }
    }

    fn predict(&self, x: &Array1<f32>) -> usize {
        let z1 = x.dot(&self.w1) + &self.b1;
        let a1 = Self::relu(&z1);
        let z2 = a1.dot(&self.w2) + &self.b2;
        let a2 = Self::softmax(&z2);
        a2.argmax().unwrap()
    }

    fn test(&self, images: &Array2<f32>, labels: &Array1<u8>) -> f32 {
        let mut correct = 0;
        for (x, &y) in images.outer_iter().zip(labels.iter()) {
            if self.predict(&x) == y as usize {
                correct += 1;
            }
        }
        correct as f32 / images.len() as f32
    }
}
