use crate::vulkan_context::VulkanContext;
use ash::vk;

pub fn run_phase_2() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Phase 2: Vulkan Foundation ===\n");

    let ctx = VulkanContext::new()?;
    println!("✓ Vulkan device and queues initialized");

    // Descriptor set layout
    let descriptor_set_layout = ctx.create_descriptor_set_layout(1)?;
    println!("✓ Descriptor set layout created");

    // Pipeline layout
    let set_layouts = [descriptor_set_layout];
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&set_layouts);

    let pipeline_layout = unsafe {
        ctx.device
            .create_pipeline_layout(&pipeline_layout_create_info, None)?
    };
    println!("✓ Pipeline layout created");

    // Descriptor pool and sets
    let descriptor_pool = ctx.create_descriptor_pool(1, 1)?;
    println!("✓ Descriptor pool created");

    let descriptor_sets = ctx.allocate_descriptor_sets(descriptor_pool, descriptor_set_layout, 1)?;
    println!("✓ Descriptor sets allocated");

    // Storage buffer
    let buffer_size = std::mem::size_of::<f32>() as vk::DeviceSize * 256;
    let buffer = ctx.create_buffer(buffer_size)?;
    let _buffer_memory = ctx.allocate_buffer_memory(buffer)?;
    println!("✓ GPU storage buffer created (256 floats = {} bytes)", buffer_size);

    // Command pool and buffers
    let command_buffers = ctx.allocate_command_buffers(1)?;
    println!("✓ Command buffer allocated");

    // Record a simple command buffer
    let command_buffer = command_buffers[0];
    let begin_info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        ctx.device.begin_command_buffer(command_buffer, &begin_info)?;
        ctx.device.end_command_buffer(command_buffer)?;
    }
    println!("✓ Command buffer recorded");

    // Memory management demonstration
    let mem_requirements = unsafe { ctx.device.get_buffer_memory_requirements(buffer) };
    println!("\nMemory Management:");
    println!("  Buffer size: {} bytes", mem_requirements.size);
    println!("  Memory alignment: {} bytes", mem_requirements.alignment);

    // Cleanup
    unsafe {
        ctx.device.destroy_pipeline_layout(pipeline_layout, None);
        ctx.device
            .destroy_descriptor_set_layout(descriptor_set_layout, None);
        ctx.device.destroy_descriptor_pool(descriptor_pool, None);
        ctx.device.destroy_buffer(buffer, None);
    }

    println!("\n✓ Phase 2 completed successfully!");
    println!("  - Vulkan device and queues initialized");
    println!("  - Descriptor pools and layouts created");
    println!("  - GPU memory allocation demonstrated");
    println!("  - Command buffers created and recorded");
    println!("  - Pipeline infrastructure set up");

    Ok(())
}
