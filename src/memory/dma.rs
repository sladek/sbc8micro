#[derive(Clone)]
pub struct DmaRequest{
    address: u16,
    data: Vec<u8>,
}

impl DmaRequest {
    pub fn new(address: u16, data: Vec<u8>) -> Self {
        Self { address, data, }
    }
    pub fn get_address(&self) -> u16{
        self.address
    }
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
#[derive(Clone)]
pub struct Dma {
    requests: Vec<DmaRequest>,
}

impl Dma {
    pub fn new(request: DmaRequest) -> Self {
        Self {
            requests: vec![request],
        }
    }
    pub fn get_requests(&self) -> Vec<DmaRequest> {
        self.requests.clone()
    }
}