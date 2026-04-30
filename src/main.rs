mod phase1;
mod phase2;
mod shader;
mod vulkan_context;

fn main() {
    println!("Vulkan GEMM Project\n");

    // Run Phase 2 (which internally uses Phase 1 concepts)
    if let Err(e) = phase2::run_phase_2() {
        eprintln!("✗ Error: {}", e);
    }
}
