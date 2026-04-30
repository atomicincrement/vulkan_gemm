mod phase1;
mod phase2;
mod phase3;
mod shader;
mod vulkan_context;

fn main() {
    println!("Vulkan GEMM Project\n");

    // Run Phase 3 (which builds on Phase 1 and 2 concepts)
    if let Err(e) = phase3::run_phase_3() {
        eprintln!("✗ Error: {}", e);
    }
}
