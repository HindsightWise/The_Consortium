use anyhow::Result;

pub struct ProcessImmunity;

impl ProcessImmunity {
    pub fn deny_debuggers() -> Result<()> {
        Ok(())
    }
}
