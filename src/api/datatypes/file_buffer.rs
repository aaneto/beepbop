pub struct FileBuffer {
    pub data: Vec<u8>,
    pub name: String,
}

impl FileBuffer {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        FileBuffer { name, data }
    }
}
