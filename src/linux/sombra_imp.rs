use crate::{Sombra, SombraError};

pub struct SombraLinux {
    process_name: String,
    auto_reload: bool,
}

impl Sombra for SombraLinux {
    fn build(process_path: &str) -> Self {
        unimplemented!()
    }

    fn create(&self) -> Result<(), SombraError> {
        unimplemented!()
    }

    fn delete(&self) -> Result<(), SombraError> {
        unimplemented!()
    }
}
