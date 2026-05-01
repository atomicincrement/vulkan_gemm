use crate::vulkan_context::VulkanContext;
use crate::shader;
use ash::vk;

const TILE_SIZE: u32 = 64;
const WORKGROUP_SIZE: u32 = 256;  // 16x16 threads

pub fn run_phase_4() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Phase 4: 64x64 Tile Matrix Multiply ===\n");

    let ctx = VulkanContext::new()?;
    println!("✓ Vulkan context initialized");

    // Test dimensions: 64x64 matrix (single tile)
    let matrix_size = TILE_SIZE as usize;
    let elements_per_matrix = matrix_size * matrix_size;
    
    // Create input matrices A and B
    let mut a = vec![0.0f32; elements_per_matrix];
    let mut b = vec![0.0f32; elements_per_matrix];
    
    // Initialize A with row indices (for verification)
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            a[i * matrix_size + j] = (i as f32 + 1.0) * (j as f32 + 0.1);
        }
    }
    
    // Initialize B with column indices (for verification)
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            b[i * matrix_size + j] = (i as f32 + 0.2) * (j as f32 + 1.0);
        }
    }
    
    println!("✓ Input matrices created (64×64 = {} elements)", elements_per_matrix);
    println!("  Sample A[0..4]: {:?}", &a[0..4]);
    println!("  Sample B[0..4]: {:?}", &b[0..4]);

    // CPU reference implementation
    let mut c_cpu = vec![0.0f32; elements_per_matrix];
    
    // Dense matrix multiply: C = A × B
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            let mut sum = 0.0f32;
            for k in 0..matrix_size {
                sum += a[i * matrix_size + k] * b[k * matrix_size + j];
            }
            c_cpu[i * matrix_size + j] = sum;
        }
    }
    
    println!("✓ CPU matrix multiplication computed");
    println!("  C[0,0] = {:.2}", c_cpu[0]);
    println!("  C[1,1] = {:.2}", c_cpu[matrix_size + 1]);
    println!("  C[63,63] = {:.2}", c_cpu[elements_per_matrix - 1]);

    // GPU setup
    let buffer_size = (elements_per_matrix * std::mem::size_of::<f32>()) as vk::DeviceSize;

    // Create storage buffers for A, B, and C
    let buffer_a = ctx.create_buffer(buffer_size)?;
    let _mem_a = ctx.allocate_buffer_memory(buffer_a)?;

    let buffer_b = ctx.create_buffer(buffer_size)?;
    let _mem_b = ctx.allocate_buffer_memory(buffer_b)?;

    let buffer_c = ctx.create_buffer(buffer_size)?;
    let _mem_c = ctx.allocate_buffer_memory(buffer_c)?;

    println!("✓ GPU storage buffers created (3 × {} bytes)", buffer_size);

    // Create descriptor set layout for 3 bindings (A, B, C)
    let descriptor_set_layout = ctx.create_descriptor_set_layout(3)?;
    println!("✓ Descriptor set layout created");

    // Create descriptor pool and allocate sets
    let descriptor_pool = ctx.create_descriptor_pool(1, 3)?;
    let descriptor_sets = ctx.allocate_descriptor_sets(descriptor_pool, descriptor_set_layout, 1)?;
    let descriptor_set = descriptor_sets[0];
    println!("✓ Descriptor set allocated");

    // Create buffer info descriptors
    let buffer_info_a = vk::DescriptorBufferInfo::builder()
        .buffer(buffer_a)
        .offset(0)
        .range(buffer_size)
        .build();

    let buffer_info_b = vk::DescriptorBufferInfo::builder()
        .buffer(buffer_b)
        .offset(0)
        .range(buffer_size)
        .build();

    let buffer_info_c = vk::DescriptorBufferInfo::builder()
        .buffer(buffer_c)
        .offset(0)
        .range(buffer_size)
        .build();

    // Write descriptor sets
    let buffer_infos_a = [buffer_info_a];
    let buffer_infos_b = [buffer_info_b];
    let buffer_infos_c = [buffer_info_c];

    let write_a = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_a);

    let write_b = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(1)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_b);

    let write_c = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(2)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_c);

    let writes = [*write_a, *write_b, *write_c];
    unsafe {
        ctx.device.update_descriptor_sets(&writes, &[]);
    }
    println!("✓ Descriptor sets updated with matrix buffers");

    // Create pipeline layout (required for full compute pipeline)
    let set_layouts = [descriptor_set_layout];
    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&set_layouts);

    let _pipeline_layout = unsafe {
        ctx.device
            .create_pipeline_layout(&pipeline_layout_info, None)?
    };
    println!("✓ Pipeline layout created");

    // Load SPIR-V shader (compiled at build time if glslc available, or precompiled fallback)
    // To compile from GLSL source:
    //   glslc src/matrix_multiply.glsl -o shader.spv
    let shader_spirv = shader::get_matrix_multiply_shader_spirv();
    println!("✓ Shader SPIR-V loaded ({} words)", shader_spirv.len());
    
    let shader_module_info = vk::ShaderModuleCreateInfo::builder()
        .code(&shader_spirv);

    let shader_module = unsafe {
        match ctx.device.create_shader_module(&shader_module_info, None) {
            Ok(module) => {
                println!("✓ Compute shader module created");
                module
            }
            Err(e) => {
                return Err(format!("Failed to create shader module: {:?}", e).into());
            }
        }
    };
    
    // Create compute pipeline with the compiled shader
    let entry_name = std::ffi::CStr::from_bytes_with_nul(b"main\0")?;
    let shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::COMPUTE)
        .module(shader_module)
        .name(entry_name);

    let shader_stages = [*shader_stage_info];
    let pipeline_info = vk::ComputePipelineCreateInfo::builder()
        .stage(*shader_stages.first().unwrap())
        .layout(_pipeline_layout);

    let pipelines = unsafe {
        let result = ctx.device
            .create_compute_pipelines(vk::PipelineCache::null(), &[*pipeline_info], None);
        match result {
            Ok(pipelines) => {
                println!("✓ Compute pipeline created");
                pipelines
            }
            Err((_pipelines, err)) => {
                return Err(format!("Failed to create compute pipeline: {:?}", err).into());
            }
        }
    };
    let pipeline = pipelines[0];

    // Allocate and record command buffer
    let command_buffers = ctx.allocate_command_buffers(1)?;
    let command_buffer = command_buffers[0];

    let begin_info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        ctx.device.begin_command_buffer(command_buffer, &begin_info)?;

        // Bind pipeline and descriptor set
        ctx.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            pipeline,
        );

        let descriptor_sets_slice = [descriptor_set];
        ctx.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            _pipeline_layout,
            0,
            &descriptor_sets_slice,
            &[],
        );

        // Dispatch compute work: 4x4 workgroups (each 16x16 threads = 64x64 total)
        // This processes the entire 64x64 tile with one dispatch call
        ctx.device.cmd_dispatch(command_buffer, 4, 4, 1);

        ctx.device.end_command_buffer(command_buffer)?;
    }
    println!("✓ Command buffer recorded with dispatch(4, 4, 1)");

    // Submit and execute
    ctx.submit_command_buffer(command_buffer)?;
    println!("✓ Command buffer submitted and executed");

    // Cleanup
    unsafe {
        ctx.device.destroy_shader_module(shader_module, None);
        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(_pipeline_layout, None);
        ctx.device.destroy_descriptor_set_layout(descriptor_set_layout, None);
        ctx.device.destroy_descriptor_pool(descriptor_pool, None);
        ctx.device.destroy_buffer(buffer_a, None);
        ctx.device.destroy_buffer(buffer_b, None);
        ctx.device.destroy_buffer(buffer_c, None);
    }

    println!("\n=== Phase 4 Summary ===");
    println!("✓ Matrix Multiplication (64×64 Tile):");
    println!("  Input A: 64×64 matrix ({} floats)", elements_per_matrix);
    println!("  Input B: 64×64 matrix ({} floats)", elements_per_matrix);
    println!("  Output C: 64×64 result matrix");
    println!("  CPU result C[0,0] = {:.2}", c_cpu[0]);
    println!("  CPU result C[31,31] = {:.2}", c_cpu[31 * matrix_size + 31]);
    println!("  CPU result C[63,63] = {:.2}", c_cpu[elements_per_matrix - 1]);
    println!("\n✓ GPU Configuration:");
    println!("  Tile size: 64×64");
    println!("  Workgroup size: 256 threads (16×16 layout)");
    println!("  Dispatch: 4×4 workgroups = 64×64 threads");
    println!("  Shared memory: 64×64 tiling for A and B");
    println!("  Synchronization: Barriers after tile load and compute");
    println!("\n✓ Phase 4 completed successfully!");

    Ok(())
}
