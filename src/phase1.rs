use ash::vk;
use std::ffi::CStr;

pub fn detect_gpu_capabilities() -> Result<(), Box<dyn std::error::Error>> {
    let entry = unsafe { ash::Entry::load()? };

    // Create Vulkan instance
    let app_info = vk::ApplicationInfo::builder()
        .application_name(CStr::from_bytes_with_nul(b"vulkan_gemm\0")?)
        .application_version(vk::make_api_version(0, 0, 1, 0))
        .api_version(vk::make_api_version(0, 1, 2, 0));

    let create_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

    let instance = unsafe { entry.create_instance(&create_info, None)? };

    // Enumerate physical devices
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };

    if physical_devices.is_empty() {
        println!("No Vulkan-capable GPUs found");
        return Ok(());
    }

    println!("Found {} GPU(s)\n", physical_devices.len());

    for (idx, device) in physical_devices.iter().enumerate() {
        println!("=== GPU {} ===", idx);

        // Get device properties
        let properties = unsafe { instance.get_physical_device_properties(*device) };
        let device_name = unsafe {
            CStr::from_ptr(properties.device_name.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        println!("Name: {}", device_name);
        println!("Type: {:?}", properties.device_type);
        println!(
            "API Version: {}.{}.{}",
            vk::api_version_major(properties.api_version),
            vk::api_version_minor(properties.api_version),
            vk::api_version_patch(properties.api_version)
        );

        // Get device features
        let features = unsafe { instance.get_physical_device_features(*device) };
        println!("Shader Float64: {}", features.shader_float64 != 0);

        // Get memory properties
        let memory_props =
            unsafe { instance.get_physical_device_memory_properties(*device) };
        println!("Memory Heaps: {}", memory_props.memory_heap_count);
        for i in 0..memory_props.memory_heap_count as usize {
            let heap = memory_props.memory_heaps[i];
            println!("  Heap {}: {:.2} GB", i, heap.size as f64 / 1e9);
        }

        // Check queue families
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(*device) };
        println!("Queue Families: {}", queue_families.len());
        for (q_idx, queue_family) in queue_families.iter().enumerate() {
            println!(
                "  Family {}: count={}, flags={:?}",
                q_idx, queue_family.queue_count, queue_family.queue_flags
            );
        }

        // Check format support
        println!("\nFormat Support:");
        check_format_support(
            &instance,
            *device,
            "R32G32B32A32_SFLOAT (fp32)",
            vk::Format::R32G32B32A32_SFLOAT,
        )?;
        check_format_support(
            &instance,
            *device,
            "R16G16B16A16_SFLOAT (fp16)",
            vk::Format::R16G16B16A16_SFLOAT,
        )?;
        check_format_support(
            &instance,
            *device,
            "R16G16B16A16_SNORM (bfloat16*)",
            vk::Format::R16G16B16A16_SNORM,
        )?;

        // Note: FP8 and FP4 are not standard Vulkan formats, typically require extensions
        println!("Note: FP8 and FP4 require KHR_8bit_storage or KHR_16bit_storage extensions");

        println!();
    }

    unsafe { instance.destroy_instance(None); }

    Ok(())
}

fn check_format_support(
    instance: &ash::Instance,
    device: vk::PhysicalDevice,
    name: &str,
    format: vk::Format,
) -> Result<(), Box<dyn std::error::Error>> {
    let props = unsafe { instance.get_physical_device_format_properties(device, format) };

    let mut support_types = Vec::new();
    if props
        .linear_tiling_features
        .intersects(vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER)
    {
        support_types.push("linear");
    }
    if props
        .optimal_tiling_features
        .intersects(vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER)
    {
        support_types.push("optimal");
    }
    if props
        .buffer_features
        .intersects(vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER)
    {
        support_types.push("buffer");
    }

    if support_types.is_empty() {
        println!("  {}: NOT SUPPORTED", name);
    } else {
        println!("  {}: ✓ ({})", name, support_types.join(", "));
    }

    Ok(())
}
