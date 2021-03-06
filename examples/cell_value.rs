// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of using cell range noise

extern crate noise;

use noise::{cell2_value, cell3_value, cell4_value, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("cell2_value.png", &Seed::new(0), 256, 256, scaled_cell2_value);
    debug::render_png("cell3_value.png", &Seed::new(0), 256, 256, scaled_cell3_value);
    debug::render_png("cell4_value.png", &Seed::new(0), 256, 256, scaled_cell4_value);
    println!("\nGenerated cell2_value.png, cell3_value.png and cell4_value.png");
}

fn scaled_cell2_value(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell2_value(seed, &[point[0] / 32.0f32, point[1] / 32.00]) * 2.0 - 1.0
}

fn scaled_cell3_value(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell3_value(seed, &[point[0] / 32.0f32, point[1] / 32.00, 0.0]) * 2.0 - 1.0
}

fn scaled_cell4_value(seed: &Seed, point: &Point2<f32>) -> f32 {
    cell4_value(seed, &[point[0] / 32.0f32, point[1] / 32.00, 0.0, 0.0]) * 2.0 - 1.0
}
