use ash::{vk, Device, Instance, Entry};
use std::ffi::CStr;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub queue: vk::Queue,
    pub queue_family_index: u32,
    pub command_pool: vk::CommandPool,
}

impl VulkanContext {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load Vulkan entry point
        let entry = unsafe { Entry::load()? };

        // Create instance
        let app_info = vk::ApplicationInfo::builder()
            .application_name(CStr::from_bytes_with_nul(b"vulkan_gemm\0")?)
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 2, 0));

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        // Find suitable physical device with compute support
        let physical_devices = unsafe { instance.enumerate_physical_devices()? };
        let physical_device = *physical_devices
            .first()
            .ok_or("No Vulkan device found")?;

        // Find compute queue family
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        
        let queue_family_index = queue_families
            .iter()
            .position(|qf| qf.queue_flags.contains(vk::QueueFlags::COMPUTE))
            .ok_or("No compute queue family found")? as u32;

        // Create device
        let queue_priorities = [1.0];
        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&queue_priorities);

        let queue_create_infos = [*queue_create_info];
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos);

        let device = unsafe {
            instance.create_device(physical_device, &device_create_info, None)?
        };

        // Get queue
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        // Create command pool
        let command_pool_create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool =
            unsafe { device.create_command_pool(&command_pool_create_info, None)? };

        Ok(VulkanContext {
            entry,
            instance,
            physical_device,
            device,
            queue,
            queue_family_index,
            command_pool,
        })
    }

    pub fn create_buffer(&self, size: vk::DeviceSize) -> Result<vk::Buffer, Box<dyn std::error::Error>> {
        let buffer_create_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { self.device.create_buffer(&buffer_create_info, None)? };
        Ok(buffer)
    }

    pub fn allocate_buffer_memory(
        &self,
        buffer: vk::Buffer,
    ) -> Result<vk::DeviceMemory, Box<dyn std::error::Error>> {
        let mem_requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };

        let mem_props = unsafe {
            self.instance
                .get_physical_device_memory_properties(self.physical_device)
        };

        let mem_type_index = self.find_memory_type(
            mem_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &mem_props,
        )?;

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(mem_type_index);

        let memory = unsafe { self.device.allocate_memory(&alloc_info, None)? };
        unsafe { self.device.bind_buffer_memory(buffer, memory, 0)? };

        Ok(memory)
    }

    fn find_memory_type(
        &self,
        type_bits: u32,
        properties: vk::MemoryPropertyFlags,
        mem_props: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        for i in 0..mem_props.memory_type_count {
            if (type_bits & (1 << i)) != 0
                && mem_props.memory_types[i as usize]
                    .property_flags
                    .contains(properties)
            {
                return Ok(i);
            }
        }
        Err("No suitable memory type found".into())
    }

    pub fn create_descriptor_set_layout(
        &self,
        binding_count: u32,
    ) -> Result<vk::DescriptorSetLayout, Box<dyn std::error::Error>> {
        let mut bindings = Vec::new();
        for i in 0..binding_count {
            bindings.push(
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(i)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
                    .build(),
            );
        }

        let layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&bindings);

        let layout = unsafe {
            self.device
                .create_descriptor_set_layout(&layout_create_info, None)?
        };

        Ok(layout)
    }

    pub fn create_descriptor_pool(
        &self,
        max_sets: u32,
        binding_count: u32,
    ) -> Result<vk::DescriptorPool, Box<dyn std::error::Error>> {
        let pool_sizes = vec![vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(binding_count)
            .build()];

        let pool_create_info = vk::DescriptorPoolCreateInfo::builder()
            .max_sets(max_sets)
            .pool_sizes(&pool_sizes);

        let pool = unsafe {
            self.device
                .create_descriptor_pool(&pool_create_info, None)?
        };

        Ok(pool)
    }

    pub fn allocate_descriptor_sets(
        &self,
        pool: vk::DescriptorPool,
        layout: vk::DescriptorSetLayout,
        count: u32,
    ) -> Result<Vec<vk::DescriptorSet>, Box<dyn std::error::Error>> {
        let layouts = vec![layout; count as usize];
        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(pool)
            .set_layouts(&layouts);

        let sets = unsafe { self.device.allocate_descriptor_sets(&alloc_info)? };
        Ok(sets)
    }

    pub fn allocate_command_buffers(
        &self,
        count: u32,
    ) -> Result<Vec<vk::CommandBuffer>, Box<dyn std::error::Error>> {
        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        let buffers = unsafe { self.device.allocate_command_buffers(&alloc_info)? };
        Ok(buffers)
    }

    pub fn submit_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_buffers = [command_buffer];
        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&command_buffers);

        unsafe {
            self.device
                .queue_submit(self.queue, &[*submit_info], vk::Fence::null())?;
            self.device.queue_wait_idle(self.queue)?;
        }

        Ok(())
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
