mod phase1;
mod phase2;
mod phase3;
mod phase4;
mod shader;
mod vulkan_context;

fn main() {
    println!("Vulkan GEMM Project\n");

    // Run Phase 4 (64x64 tile matrix multiply)
    if let Err(e) = phase4::run_phase_4() {
        eprintln!("✗ Error: {}", e);
    }
}
