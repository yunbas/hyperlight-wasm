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
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <open_cv_wasm_module_path> <input_image_path> <output_image_path>", args[0]);
        std::process::exit(1);
    }

    let open_cv_module_path = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];
    let img = ImageReader::open(Path::new(input_path))
        .expect("Failed to open image file")
        .decode()
        .expect("Failed to decode image");
    
    let input = img.to_rgb8().as_raw().to_vec();
    let size = input.len();

    let mut start = time::Instant::now();

    println!("Create the sandbox");

    let proto_wasm_sandbox = SandboxBuilder::new()
        .with_guest_input_buffer_size(64 * 1024 * 1024)
        .with_guest_output_buffer_size(64 * 1024 * 1024)
        .with_guest_heap_size(512 * 1024 * 1024)
        .with_guest_stack_size(128 * 1024)
        .with_guest_function_call_max_execution_time_millis(60000)
        .build()?;

    println!("Created the sandbox: {:?}", start.elapsed().as_millis());
    start = time::Instant::now();

    println!("Load the runtime");
    let wasm_sandbox = proto_wasm_sandbox.load_runtime()?;
    println!("Loaded the runtime: {:?}", start.elapsed().as_millis());

    // Load the Wasm module into the sandbox
    start = time::Instant::now();
    println!("Load the module");

    let mut loaded_wasm_sandbox = wasm_sandbox.load_module(open_cv_module_path)?;
    println!("Loaded the module: {:?}", start.elapsed().as_millis());

    println!("Call guest function");
    start = time::Instant::now();

    let fn_name = "cropFace";
    let params_opt = Some(vec![ParameterValue::VecBytes(input.clone()), ParameterValue::Int(size as i32), ParameterValue::UInt(img.height()), ParameterValue::UInt(img.width()), ParameterValue::UInt(get_channel_count(&img)), ParameterValue::UInt(100), ParameterValue::UInt(100)]);
    // Call a function in the Wasm module
    let ReturnValue::VecBytes(result) = loaded_wasm_sandbox.call_guest_function(
        fn_name,
        params_opt.clone(),
        ReturnType::VecBytes,
    )?
    else {
        panic!("Failed to get result from call_guest_function")
    };

    println!("Called guest function: {:?}, Value: {:?}", start.elapsed().as_millis(), result);

    image::DynamicImage::ImageRgb8(image::RgbImage::from_raw(100, 100, result).unwrap())
        .save(Path::new(&output_path))
        .expect("Failed to save image");

    Ok(())
}
