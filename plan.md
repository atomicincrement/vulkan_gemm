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

## Phase 3: Vector Addition Kernel ✓

* Write simple compute shader for element-wise vector addition
* Create host-side wrapper functions for shader dispatch
* Implement GPU memory transfer utilities
* Test with small vectors (e.g., 1024 elements)
* Benchmark against CPU baseline

## Phase 3.5: State-of-the-Art Discovery ✓

Research existing Vulkan matrix kernel implementations and best practices:

* **Popular Vulkan Matrix Libraries:**
  - VkFFT (FFT, supports various data types)
  - Vulkan Samples (NVIDIA - contains matrix operations)
  - SPIR-V optimizations and tools
  
* **Academic/Research Kernels:**
  - Papers on Vulkan compute optimization
  - GitHub repositories with open-source implementations
  - Kernel fusion strategies for GEMM
  
* **Performance Benchmarks:**
  - Existing Vulkan GEMM performance numbers
  - Comparison with CUDA/cuBLAS on similar hardware
  - Memory bandwidth utilization patterns
  
* **Optimization Techniques:**
  - Shared memory/local memory strategies
  - Workgroup size optimization for RDNA/Zen architectures
  - Vectorization and instruction-level parallelism
  - Bank conflict avoidance in shared memory
  - Half-precision (fp16) and lower-precision optimizations
  
* **Key Finding:** NVIDIA compute_nbody sample provides exact pattern for Phase 4
  - Shared memory tiling (64x64)
  - 16x16 workgroup configuration
  - Cooperative data loading with barriers
  - Use as reference architecture, not fork

## Phase 4: 64x64 Tile Matrix Multiplication (Infrastructure Complete) ✓

* CPU reference implementation for 64x64 matrix multiply
* GPU buffer allocation and management (3 storage buffers)
* Descriptor sets for shader bindings
* Descriptor layouts and pool creation
* Pipeline layout setup
* Command buffer allocation
* Shader module loading infrastructure

**Status:** Infrastructure complete and tested. CPU reference produces correct results.
Matrix size: 64×64 = 4,096 elements per matrix

**Next step:** Compile GLSL matrix multiply shader to SPIR-V using:
```bash
glslc shader.glsl -o shader.spv
spirv-val shader.spv
```
Then embed SPIR-V bytecode and enable GPU compute dispatch.

**Recommended GLSL approach (16x16 workgroup with shared memory tiling):**
- Workgroup: 16×16 threads (256 total)
- Shared memory: 64×64 float tiles for A and B
- Algorithm: Cooperative tile loading → barrier → compute accumulation → barrier
- Pattern: Identical to NVIDIA compute_nbody sample

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

