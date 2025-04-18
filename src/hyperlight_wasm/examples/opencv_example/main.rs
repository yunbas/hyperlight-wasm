/*
Copyright 2024 The Hyperlight Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::path::Path;
use std::time;

use image::{ImageReader, ColorType, DynamicImage};

use hyperlight_wasm::{ParameterValue, Result, ReturnType, ReturnValue, SandboxBuilder};

fn get_channel_count(img: &DynamicImage) -> u32 {
    match img.color() {
        ColorType::L8 | ColorType::L16 => 1,        // Grayscale
        ColorType::La8 | ColorType::La16 => 2,      // Grayscale with alpha
        ColorType::Rgb8 | ColorType::Rgb16 => 3,    // RGB
        ColorType::Rgba8 | ColorType::Rgba16 => 4,  // RGBA
        _ => 0, // Unknown or unsupported color type
    }
}

fn main() -> Result<()> {

}
