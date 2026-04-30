# Build gemm with vulkan

This is a project to learn how we can build neural net implementations
using vulkan command buffers. We will then compare them with CPU implementations
using AVX512-BF16 and AVX512-VNNI

* Find out if my GPU supports fp4, fp8 or fp16 via Vulkan.
* Write a simple Vulkan kernel to add two vectors.
* Write a more complex kernel that multiplies a 4x4 tile of a matrix.
* Apply the 4x4 tile multiple times to multiply A^T by B, accumulating the result.

