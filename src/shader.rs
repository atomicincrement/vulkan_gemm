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


