# Phase 3.5: State-of-the-Art Discovery

## Objective
Research existing Vulkan matrix kernel implementations to understand current best practices, performance characteristics, and potential optimizations for our GEMM implementation.

## Key Resources & Findings

### 1. Open Source Vulkan Matrix Libraries

#### VkFFT
- **Purpose**: FFT and convolution operations in Vulkan/HIP/CUDA
- **Language**: C++ with SPIR-V shaders
- **Strengths**: 
  - Highly optimized for various hardware (NVIDIA, AMD, Intel)
  - Supports half-precision (fp16) and reduced precision
  - Extensive optimization for memory patterns
  - Well-documented SPIR-V generation
- **URL**: https://github.com/GPUOpen-Tools/VkFFT

#### Vulkan Samples (NVIDIA)
- **Purpose**: Educational examples of Vulkan compute
- **Contains**: Basic matrix operations, texture compression
- **Strengths**:
  - Reference implementations
  - Best practices documentation
  - Workgroup optimization examples
- **URL**: https://github.com/KhronosGroup/Vulkan-Samples

#### GPUOpen Resources (AMD)
- **Purpose**: AMD-specific optimization guides
- **Contains**: Workgroup optimization, memory access patterns
- **Strengths**:
  - RDNA/Zen architecture-specific guidance
  - Shared memory optimization
  - Instruction selection patterns
- **URL**: https://gpuopen.com/learn/

### 2. Optimization Techniques for Vulkan GEMM

#### Workgroup Configuration
- **Optimal size for RDNA**: 256-1024 threads
- **Memory hierarchy**:
  - Registers: ~256 KB per 64 threads (fast)
  - Shared memory (LDS): 96 KB per workgroup (medium speed)
  - Global memory: VRAM (slow, requires careful coalescing)

#### Shared Memory Strategy
- Prefetch tiles to shared memory to reduce global memory traffic
- Tile size = 64x64 with 256-thread workgroup
- Each thread processes 16 elements (64×64 / 256)
- Optimal for RDNA: Avoid bank conflicts (use padding if needed)

#### Data Format Optimization
- **fp32**: Standard, highest accuracy, baseline
- **fp16**: 2x throughput, careful about accumulation overflow
- **bf16**: Better range than fp16, comparable accuracy for ML
- **int8/int4**: Specialized paths for quantized models

#### Memory Access Patterns
- Coalesce global memory reads in 128-byte transactions
- Sequential access per thread (no scatter-gather initially)
- Consider using image buffers vs. storage buffers for cache behavior

### 3. Performance Targets (RDNA Hardware)

Based on AMD Radeon RDNA specifications:
- **Peak Throughput**: 
  - fp32: ~8-12 TFLOPS (consumer/mobile RDNA)
  - fp16: ~16-24 TFLOPS
  - Matrix multiply operations scale better

- **Memory Bandwidth**: ~200-400 GB/s
- **Optimal GEMM Efficiency**: 30-60% of peak for large matrices

### 4. Current Best Practices

1. **Compute Shader Structure**:
   ```glsl
   layout(local_size_x = 256) in;  // Full wavefront for RDNA
   
   // Shared memory for tile buffering
   shared float sA[64][64];
   shared float sB[64][64];
   
   // Each thread processes multiple elements
   float result[16];  // Or use accumulation pattern
   ```

2. **Synchronization**:
   - Use `barrier()` carefully (expensive)
   - Minimize per-thread branching
   - Coalesce work efficiently

3. **Memory Patterns**:
   - Read-only buffers with coherent access
   - Separate read/write buffers when possible
   - Use `coherent` storage class for shared data

### 5. Decision: Custom Implementation vs. Integration

**Recommendation**: **Custom Implementation**

**Rationale**:
- Our scope is educational - building from first principles
- Existing libraries are heavyweight and complex
- VkFFT is optimized for FFT, not GEMM specifically
- Learning value of custom kernel outweighs performance
- Can selectively adopt techniques without full library dependency

**However**, we should:
- Study VkFFT's memory management approach
- Reference NVIDIA samples for best practices
- Apply GPUOpen guidance for RDNA optimization
- Use findings to guide tile sizes and workgroup selection

### 6. Architecture-Specific Tuning (Your Hardware)

Your GPU: **AMD Ryzen 9 9955HX (RADV RAPHAEL_MENDOCINO)**

**Key Characteristics**:
- RDNA 3 architecture (mobile/APU variant)
- 8 compute units typical for mobile
- 11.54 GB shared system memory
- Lower power envelope than desktop RDNA

**Optimization Priorities**:
1. Memory efficiency (priority #1 - limited bandwidth in mobile)
2. Reduced register pressure (shared across CUs)
3. Conservative workgroup sizing (maybe 128-256 threads)

### 7. Recommended Next Steps for Phase 4

Based on research:

1. **Tile size**: 64x64 (good balance for bandwidth)
2. **Workgroup size**: 256 threads
3. **Data format**: Start with fp32, then optimize to fp16
4. **Memory strategy**: 
   - Load 4x4 tiles into registers
   - Use shared memory for blocking if beneficial
5. **Synchronization**: Minimal barriers; structure for independent thread groups

## Research Summary

The Vulkan ecosystem for GEMM is less mature than CUDA, but solid foundations exist:
- VkFFT demonstrates high-performance compute patterns
- NVIDIA samples provide reference implementations
- AMD optimization guides are crucial for our target hardware
- Custom implementation is viable and educational

**Key insight**: Mobile RDNA (our target) favors memory-efficient algorithms over raw compute intensity.
