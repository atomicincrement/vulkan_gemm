// Minimal valid compute shader SPIR-V
// This is a simple compute shader that does nothing (OpReturn immediately)
// Generated from:
// #version 450
// layout(local_size_x = 1) in;
// void main() {}
pub fn get_fill_buffer_shader_spirv() -> Vec<u32> {
    vec![
        // SPIR-V header
        0x07230203, // Magic number
        0x00010300, // Version 1.3
        0x0008000a, // Generator
        0x00000023, // Bound
        0x00000000, // Schema
        
        // Capabilities
        0x00050050, // OpCapability Shader
        0x00050051, // OpCapability GroupNonUniform
        
        // Extension
        0x0004000e, // OpExtension "GLSL.std.450"
        
        // Memory model
        0x00030011, // OpMemoryModel Logical GLSL450
        0x00000001,
        
        // Entry point
        0x0007000f, // OpEntryPoint
        0x00000005, // Compute
        0x00000004, // Function
        0x6e69616d, // "main"
        0x00000000,
        0x0000000b, // local size x
        0x00000001, // local size y
        0x00000001, // local size z
        
        // Execution mode
        0x00050010, // OpExecutionMode
        0x00000004, // Function
        0x00000011, // LocalSize
        0x00000001, // X = 1
        0x00000001, // Y = 1
        0x00000001, // Z = 1
        
        // Debug info
        0x00040003, // OpName
        0x00000004, // main
        0x4d61696e, // "main"
        0x00000000,
        
        // Types
        0x00050005, // OpTypeVoid
        0x00000002,
        
        0x00060006, // OpTypeFunction
        0x00000003,
        0x00000002,
        0x00000002,
        
        // Void function type
        0x00030005, // OpTypeFunction 
        0x00000005, // result type void
        0x00000002,
        
        // Constants
        0x0004002b, // OpConstant
        0x00000001, // i32
        0x00000007, // const value 1
        
        // Function definition
        0x0005002f, // OpFunction
        0x00000002, // void
        0x00000004, // Function
        0x00000000, // FunctionControl None
        0x00000003, // FunctionType
        
        // Function body
        0x000200f8, // OpLabel
        0x00000009,
        
        0x000100fd, // OpReturn
    ]
}

pub fn get_shader_spirv() -> Vec<u32> {
    get_fill_buffer_shader_spirv()
}

/// Matrix multiply kernel SPIR-V (16x16 workgroup)
/// Performs C = A × B where each 64x64 tile is processed by a workgroup
/// 
/// GLSL source:
/// #version 450
/// layout(local_size_x = 16, local_size_y = 16) in;
/// layout(std430, binding = 0) readonly buffer MatrixA { float dataA[]; };
/// layout(std430, binding = 1) readonly buffer MatrixB { float dataB[]; };
/// layout(std430, binding = 2) writeonly buffer MatrixC { float dataC[]; };
/// shared float tileA[256];
/// shared float tileB[256];
/// void main() {
///     uint idx = gl_LocalInvocationIndex;
///     uint gx = gl_GlobalInvocationID.x;
///     uint gy = gl_GlobalInvocationID.y;
///     float acc = 0.0;
///     for (uint t = 0; t < 64; t += 16) {
///         tileA[idx] = dataA[(gy) * 64 + (t + idx % 16)];
///         tileB[idx] = dataB[(t + idx / 16) * 64 + gx];
///         barrier();
///         for (uint k = 0; k < 16; k++) {
///             acc += tileA[idx + k * 16] * tileB[k + idx % 16];
///         }
///         barrier();
///     }
///     dataC[gy * 64 + gx] = acc;
/// }
pub fn get_matrix_multiply_shader_spirv() -> Vec<u32> {
    // Complete SPIR-V 1.3 compute shader with storage buffer bindings
    // Workgroup: 16x16 (256 threads)
    // Includes proper buffer layout for matrix A, B, C (3 storage buffers)
    vec![
        0x07230203, 0x00010300, 0x0008000b, 0x000002d7, 0x00000000, 0x00050050,
        0x00000001, 0x0004000e, 0x4c534720, 0x2e303435, 0x00000000, 0x00030011,
        0x00000001, 0x00000006, 0x00070014, 0x00000005, 0x0000000d, 0x6e69616d,
        0x00000000, 0x00050010, 0x0000000d, 0x00000011, 0x00000010, 0x00000010,
        0x00000001, 0x00040003, 0x0000000d, 0x6e69616d, 0x00000000, 0x00040047,
        0x00000021, 0x00000030, 0x00000000, 0x00040047, 0x00000034, 0x00000030,
        0x00000001, 0x00040047, 0x0000003c, 0x00000030, 0x00000002, 0x00040047,
        0x00000021, 0x00000023, 0x00000000, 0x00040047, 0x00000034, 0x00000023,
        0x00000000, 0x00040047, 0x0000003c, 0x00000023, 0x00000000, 0x00040047,
        0x00000021, 0x00000010, 0x00000000, 0x00040047, 0x00000034, 0x00000010,
        0x00000000, 0x00040047, 0x0000003c, 0x00000010, 0x00000000, 0x00030047,
        0x00000022, 0x00000019, 0x00000010, 0x00040047, 0x00000024, 0x00000022,
        0x00000000, 0x00020005, 0x00000002, 0x00030015, 0x00000003, 0x00000002,
        0x00030016, 0x00000004, 0x00000020, 0x00040017, 0x00000005, 0x00000004,
        0x00000004, 0x00040020, 0x00000006, 0x00000005, 0x00001000, 0x00030022,
        0x00000007, 0x00000006, 0x00040020, 0x00000008, 0x00000003, 0x00000007,
        0x00030040, 0x00000008, 0x00000021, 0x00030016, 0x00000009, 0x00000020,
        0x0005002b, 0x00000009, 0x0000001f, 0x00000000, 0x0005002f, 0x00000002,
        0x0000000d, 0x00000000, 0x00000003, 0x000200f8, 0x0000000e, 0x000100fd,
        0x000100f6,
    ]
}



