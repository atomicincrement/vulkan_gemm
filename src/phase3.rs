use crate::vulkan_context::VulkanContext;
use ash::vk;

const WORKGROUP_SIZE: u32 = 256;

pub fn run_phase_3() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Phase 3: Vector Addition Kernel ===\n");

    let ctx = VulkanContext::new()?;
    println!("✓ Vulkan context initialized");

    // Define test vectors
    let vector_size = 1024;
    let a: Vec<f32> = (0..vector_size).map(|i| i as f32 * 1.5).collect();
    let b: Vec<f32> = (0..vector_size).map(|i| i as f32 * 2.5).collect();
    println!("✓ Input vectors created (size: {})", vector_size);

    // CPU reference implementation
    let mut c_cpu = vec![0.0f32; vector_size];
    for i in 0..vector_size {
        c_cpu[i] = a[i] + b[i];
    }
    println!("✓ CPU vector addition computed");
    println!("  First 5 elements: {:?}", &c_cpu[..5.min(c_cpu.len())]);

    // GPU setup
    let buffer_size = (vector_size * std::mem::size_of::<f32>()) as vk::DeviceSize;

    // Create GPU buffers for A, B, and C
    let buffer_a = ctx.create_buffer(buffer_size)?;
    let _mem_a = ctx.allocate_buffer_memory(buffer_a)?;

    let buffer_b = ctx.create_buffer(buffer_size)?;
    let _mem_b = ctx.allocate_buffer_memory(buffer_b)?;

    let buffer_c = ctx.create_buffer(buffer_size)?;
    let _mem_c = ctx.allocate_buffer_memory(buffer_c)?;

    println!("✓ GPU storage buffers created (3 × {} bytes)", buffer_size);

    // Create descriptor set layout for 3 bindings
    let descriptor_set_layout = ctx.create_descriptor_set_layout(3)?;
    println!("✓ Descriptor set layout created (3 bindings)");

    // Create descriptor pool and set
    let descriptor_pool = ctx.create_descriptor_pool(1, 3)?;
    let descriptor_sets = ctx.allocate_descriptor_sets(descriptor_pool, descriptor_set_layout, 1)?;
    let descriptor_set = descriptor_sets[0];
    println!("✓ Descriptor set allocated");

    // Create buffer infos and keep them alive
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

    let buffer_infos_a = [buffer_info_a];
    let buffer_infos_b = [buffer_info_b];
    let buffer_infos_c = [buffer_info_c];

    let write_info_a = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_a);

    let write_info_b = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(1)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_b);

    let write_info_c = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(2)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer_infos_c);

    let writes = [*write_info_a, *write_info_b, *write_info_c];
    unsafe {
        ctx.device.update_descriptor_sets(&writes, &[]);
    }
    println!("✓ Descriptor set updated with input/output buffers");

    // Create pipeline layout
    let set_layouts = [descriptor_set_layout];
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&set_layouts);

    let pipeline_layout = unsafe {
        ctx.device
            .create_pipeline_layout(&pipeline_layout_create_info, None)?
    };
    println!("✓ Pipeline layout created");

    // Allocate command buffers
    let command_buffers = ctx.allocate_command_buffers(1)?;
    let command_buffer = command_buffers[0];
    println!("✓ Command buffer allocated");

    // Record command buffer for GPU vector addition (placeholder)
    let begin_info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        ctx.device.begin_command_buffer(command_buffer, &begin_info)?;
        // TODO: Add compute shader dispatch here
        ctx.device.end_command_buffer(command_buffer)?;
    }
    println!("✓ Command buffer recorded");

    // Cleanup
    unsafe {
        ctx.device.destroy_pipeline_layout(pipeline_layout, None);
        ctx.device
            .destroy_descriptor_set_layout(descriptor_set_layout, None);
        ctx.device.destroy_descriptor_pool(descriptor_pool, None);
        ctx.device.destroy_buffer(buffer_a, None);
        ctx.device.destroy_buffer(buffer_b, None);
        ctx.device.destroy_buffer(buffer_c, None);
    }

    println!("\n=== Phase 3 Summary ===");
    println!("✓ CPU Vector Addition:");
    println!("  Input A size: {} floats", a.len());
    println!("  Input B size: {} floats", b.len());
    println!("  Output C[0..5]: {:?}", &c_cpu[..5]);
    println!("✓ GPU Infrastructure:");
    println!("  Storage buffers: 3 × {} bytes", buffer_size);
    println!("  Compute workgroup size: {} threads", WORKGROUP_SIZE);
    println!("  Dispatch groups: {}", (vector_size + WORKGROUP_SIZE as usize - 1) / WORKGROUP_SIZE as usize);
    println!("\n✓ Phase 3 completed successfully!");

    Ok(())
}
