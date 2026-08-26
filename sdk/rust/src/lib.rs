//! Native SDK entry point for embedding the Nexus runtime.
pub use nexus_runtime::{Runtime, RuntimeError, SkillManifest};

/// Connects to the built-in `sim://nxr-1` transport.
pub fn connect(uri: &str) -> Result<Runtime, String> {
    if uri == "sim://nxr-1" {
        Ok(Runtime::nxr1())
    } else {
        Err(format!("unsupported local transport: {uri}"))
    }
}
