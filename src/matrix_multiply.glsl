#version 450

// Matrix Multiply Kernel
// Workgroup size: 16x16 = 256 threads
// Each thread processes one output element
// Uses shared memory for cooperative tile loading

layout(local_size_x = 16, local_size_y = 16) in;

layout(std430, binding = 0) readonly buffer MatrixA {
    float dataA[];
};

layout(std430, binding = 1) readonly buffer MatrixB {
    float dataB[];
};

layout(std430, binding = 2) writeonly buffer MatrixC {
    float dataC[];
};

shared float tileA[16][16];
shared float tileB[16][16];

void main() {
    uvec2 gid = gl_GlobalInvocationID.xy;  // Global thread ID
    uvec2 lid = gl_LocalInvocationID.xy;   // Local thread ID (0..15)
    
    const uint MATRIX_SIZE = 64u;
    const uint TILE_SIZE = 16u;
    
    float acc = 0.0;
    
    // Process matrix in tiles
    for (uint t = 0u; t < MATRIX_SIZE; t += TILE_SIZE) {
        // Cooperative tile loading
        // Each thread loads one element into shared memory
        uint local_idx = lid.y * TILE_SIZE + lid.x;
        
        // Load tile from A: each thread loads A[gid.y, t + lid.x]
        tileA[lid.y][lid.x] = dataA[gid.y * MATRIX_SIZE + (t + lid.x)];
        
        // Load tile from B: each thread loads B[t + lid.y, gid.x]
        tileB[lid.y][lid.x] = dataB[(t + lid.y) * MATRIX_SIZE + gid.x];
        
        // Synchronize all threads in workgroup
        barrier();
        memoryBarrierShared();
        
        // Compute partial dot product using loaded tiles
        // acc += sum(tileA[lid.y][k] * tileB[k][lid.x]) for k in [0, TILE_SIZE)
        for (uint k = 0u; k < TILE_SIZE; k++) {
            acc += tileA[lid.y][k] * tileB[k][lid.x];
        }
        
        // Synchronize before next tile
        barrier();
        memoryBarrierShared();
    }
    
    // Write result: C[gid.y, gid.x] = acc
    dataC[gid.y * MATRIX_SIZE + gid.x] = acc;
}
