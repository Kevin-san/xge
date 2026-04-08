use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Native code generation error")]
    NativeCodegenError,
    #[error("WebAssembly code generation error")]
    WasmCodegenError,
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
