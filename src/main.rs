// =====================================================================
// Project: KV Cache Compressor (Rust Prototype)
// Author:  Theo Anold Gaopalelwe
// Context: Simulating 8-bit quantization of the KV Cache to reduce
//          VRAM footprint in long-context LLMs.
// =====================================================================

use rand::Rng;

fn main() {
    println!("--- KV Cache Compressor Engine ---");
    println!("Status: Booting quantization simulator...\n");

    // 1. Hardware & Context Constraints
    let context_length = 10_000; // 10,000 tokens in the chat history
    let hidden_dimension = 64;   // Simulating a 64-dimensional head (small for demo)
    
    // In FP32 (32-bit floats), each value takes 4 bytes
    let fp32_memory_mb = (context_length * hidden_dimension * 4) as f64 / (1024.0 * 1024.0);
    // In INT8 (8-bit integers), each value takes 1 byte
    let int8_memory_mb = (context_length * hidden_dimension * 1) as f64 / (1024.0 * 1024.0);

    println!("=== Memory Footprint ===");
    println!("Context Length: {} tokens", context_length);
    println!("FP32 Cache Size: {:.2} MB", fp32_memory_mb);
    println!("INT8 Cache Size: {:.2} MB", int8_memory_mb);
    println!("Memory Saved:    {:.2}%", (1.0 - (int8_memory_mb / fp32_memory_mb)) * 100.0);
    println!();

    // 2. Generate synthetic KV Cache data (FP32)
    let mut rng = rand::thread_rng();
    let mut kv_cache_fp32: Vec<f32> = Vec::new();
    
    // Fill with random float values between -1.0 and 1.0
    for _ in 0..(context_length * hidden_dimension) {
        kv_cache_fp32.push(rng.gen_range(-1.0..1.0));
    }

    // 3. Compress to INT8 (Quantization)
    // To map [-1.0, 1.0] to [-127, 127], we multiply by 127
    let scale_factor = 127.0;
    let mut kv_cache_int8: Vec<i8> = Vec::new();
    
    for &val in &kv_cache_fp32 {
        let scaled = val * scale_factor;
        // Clamp to prevent overflow and cast to i8
        let clamped = scaled.round().clamp(-127.0, 127.0);
        kv_cache_int8.push(clamped as i8);
    }

    // 4. Decompress back to FP32 (De-quantization)
    // To see the error, we reverse the math
    let mut kv_cache_decompressed: Vec<f32> = Vec::new();
    for &val in &kv_cache_int8 {
        kv_cache_decompressed.push((val as f32) / scale_factor);
    }

    // 5. Calculate Quantization Error (Mean Squared Error)
    let mut total_squared_error: f64 = 0.0;
    for (original, decompressed) in kv_cache_fp32.iter().zip(kv_cache_decompressed.iter()) {
        let error = (original - decompressed) as f64;
        total_squared_error += error * error;
    }
    
    let mse = total_squared_error / (kv_cache_fp32.len() as f64);

    println!("=== Quantization Quality ===");
    println!("Mean Squared Error (MSE): {:.8}", mse);
    println!("\nConclusion: INT8 quantization cuts memory by 75%, with negligible error.");
    println!("Next commit: Dynamic scaling based on local maxima.");
}
