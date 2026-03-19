pub const EMBEDDING_DIM: usize = 256;
pub const MAX_SEQ_LEN: usize = 64;

/// ANE-Native MIL for a 256-ch Linear Layer (Simplified for Inference)
/// This follows the maderix/ANE layout: [1, C, 1, S]
pub const SIMPLE_LINEAR_MIL: &str = r#"
main(input0: tensor<f16, [1, 256, 1, 64]>) -> (output0: tensor<f16, [1, 256, 1, 64]>) {
  block0() {
    %weights = const_tensor<f16, [1, 256, 1, 256]>(value: 0.0)
    %0 = core.matrix_multiply(a: input0, b: %weights)
    return (%0)
  }
}
"#;

pub fn generate_identity_weights() -> Vec<u8> {
    // Generate an identity weight blob for a 256x256 matrix in f16
    let mut weights = vec![0u8; 1 * 256 * 1 * 256 * 2]; // 2 bytes per f16
    // Simple mock: fill with a pattern to verify ANE execution
    for i in 0..256 {
        let offset = (i * 256 + i) * 2;
        if offset + 1 < weights.len() {
            weights[offset] = 0x00;
            weights[offset + 1] = 0x3C; // 1.0 in f16
        }
    }
    weights
}
