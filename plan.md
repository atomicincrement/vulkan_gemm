# Build gemm with vulkan

This is a project to learn how we can build neural net implementations
using vulkan command buffers. We will then compare them with CPU implementations
using AVX512-BF16 and AVX512-VNNI.

## Phase 1: GPU Capability Detection ✓

* Query Vulkan device properties for supported formats
* Check for fp4, fp8, fp16 format support
* Verify compute queue capabilities
* Log GPU memory limits and max workgroup sizes
* Document supported formats for target hardware

## Phase 2: Vulkan Foundation ✓

* Initialize Vulkan instance and device
* Set up compute queue and command buffers
* Create descriptor pools and layouts
* Implement memory management (buffer creation, uploads, downloads)
* Build basic shader compilation pipeline from GLSL/SPIR-V

## Phase 3: Vector Addition Kernel

* Write simple compute shader for element-wise vector addition
* Create host-side wrapper functions for shader dispatch
* Implement GPU memory transfer utilities
* Test with small vectors (e.g., 1024 elements)
* Benchmark against CPU baseline

## Phase 4: Matrix Multiplication (4x4 Tile)

* Design 4x4 tile multiplication kernel
  - Load 4x4 tile from matrix A
  - Load 4x4 tile from matrix B
  - Perform 16 multiply-accumulate operations
  - Write 4x4 result tile
* Optimize for workgroup size and memory access patterns
* Test with small matrices (e.g., 16x16, 64x64)

## Phase 5: Full GEMM Implementation

* Tile the matrices into 4x4 blocks
* Dispatch kernel to multiply A^T by B
* Accumulate results into output matrix
* Handle non-aligned matrix dimensions
* Test with various matrix sizes

## Phase 6: CPU Reference Implementation

* Implement AVX512-BF16 vector addition
* Implement AVX512-VNNI matrix multiplication
* Create simple benchmarking framework
* Compare GPU vs CPU performance

## Phase 7: Benchmarking & Analysis

* Create benchmark harness for both implementations
* Test with varying matrix sizes
* Measure throughput and latency
* Document performance findings
* Identify optimization opportunities

