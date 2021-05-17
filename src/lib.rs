use std::ffi::{CString, CStr};

#[derive(Clone, Debug)]
pub struct Policy {
    inherit_resources: Vec<u64>,
    file_read: Vec<CString>,
    file_write: Vec<CString>,
    file_append: Vec<CString>,
    file_create: Vec<CString>,
    directory_create: Vec<CString>,
    directory_delete: Vec<CString>,
}

impl Policy {
    pub fn new() -> Self {
        Self {
            inherit_resources: Vec::new(),
            file_read: Vec::new(),
            file_write: Vec::new(),
            file_append: Vec::new(),
            file_create: Vec::new(),
            directory_create: Vec::new(),
            directory_delete: Vec::new(),
        }
    }

    pub fn allow_inherit_resource(&mut self, resource: u64) -> Result<(), String> {
        self.inherit_resources.push(resource);
        Ok(())
    }

    pub fn get_inherited_resources(&self) -> Vec<u64> {
        self.inherit_resources.clone()
    }

    pub fn allow_file_read(&mut self, path: &CStr) -> Result<(), String> {
        self.file_read.push(path.to_owned());
        Ok(())
    }
}

