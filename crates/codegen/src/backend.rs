
use crate::error::CodegenError;
use mir::mir::MirModule;
use anyhow::Result;

pub trait Backend {
    fn generate(&self, module: &MirModule) -> Result<String, CodegenError>;
}

pub struct NativeBackend;

impl Backend for NativeBackend {
    fn generate(&self, _module: &MirModule) -> Result<String, CodegenError> {
        // TODO: Implement native code generation
        Ok(String::from("# Native code generation not implemented yet"))
    }
}

pub struct WasmBackend;

impl Backend for WasmBackend {
    fn generate(&self, _module: &MirModule) -> Result<String, CodegenError> {
        // TODO: Implement WebAssembly code generation
        Ok(String::from(";; WebAssembly code generation not implemented yet"))
    }
}

pub struct CodeGenerator {
    backend: Box<dyn Backend>,
}

impl CodeGenerator {
    pub fn new_native() -> Self {
        CodeGenerator {
            backend: Box::new(NativeBackend),
        }
    }

    pub fn new_wasm() -> Self {
        CodeGenerator {
            backend: Box::new(WasmBackend),
        }
    }

    pub fn generate(&self, module: &MirModule) -> Result<String, CodegenError> {
        self.backend.generate(module)
    }
}

