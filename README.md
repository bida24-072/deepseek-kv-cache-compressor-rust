# KV Cache Compressor (Rust)

> **Status:** Early Prototype · **Target:** Quantizing KV Cache to reduce VRAM footprint.

## The Problem
In long-context LLMs, the KV Cache (Key-Value Cache) grows linearly with the context length. A 32k-token conversation can consume massive amounts of VRAM, leading to out-of-memory (OOM) errors on consumer hardware. 

The standard solution is **Quantization**: compressing the cache from FP32 (32-bit floating point) to INT8 (8-bit integer). However, this introduces quantization error.

## The Solution (Deliberately Narrow Scope)
This Rust prototype simulates a synthetic KV Cache for a 10,000-token context. It:
1. Generates the cache in FP32.
2. Compresses it to INT8.
3. Decompresses it back to FP32.
4. Measures the Mean Squared Error (MSE) to quantify the quality loss.

## Architecture
- `src/main.rs` — Generation, compression, decompression, and error calculation.
- **Dependencies:** `rand` for generating synthetic cache values.

## Why Rust?
DeepSeek builds core infrastructure (like DSec) in Rust. Memory management at this level requires strict safety and zero-cost abstractions. This proves quantization logic can be handled natively without Python overhead.

## Why This Matters (To Me)
This is the 3rd pillar in my series exploring DeepSeek's inference memory hierarchy:
1. **MoE Expert Residency:** Dynamic compute memory.
2. **Engram Prefetching:** Static memory lookup.
3. **KV Cache Compression:** Context memory.

I am building these to learn and contribute to the open-source AI infrastructure ecosystem.

## Roadmap
- [ ] Implement dynamic per-channel scaling (better than global scaling).
- [ ] Add 4-bit quantization.
- [ ] Integrate a real KV Cache trace from a small model.

---
**Author:** Theo Anold Gaopalelwe  
*Founder, AXIOMIQ Digital Innovations*  
[GitHub Profile](https://github.com/bida24-072)
