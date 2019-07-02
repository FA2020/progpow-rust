use crate::types::{H256, PpCompute, ProgPowError, Hardware};
use progpow_gpu::{GPU, Driver};

pub struct PpGPU {
    pub gpu: GPU,
}

impl PpGPU {

    pub fn new() -> Self {
        PpGPU {
            gpu: GPU::new(Driver::OCL)
        }
    }

    pub fn get_solutions(&self) -> Option<(u64, [u8; 32])> {
        self.gpu.solutions().unwrap()
    }
}

impl PpCompute for PpGPU {

    fn init(&mut self) -> Result<(), ProgPowError> {
        self.gpu.init();
        Ok(())
    }

    fn verify(&self, header_hash: &[u8; 32], height: u64, nonce: u64) -> Result<([u32; 8], [u32; 8]), ProgPowError> {
        unimplemented!()
    }

    fn compute(&self, header: &[u8], height: u64, epoch: i32, boundary: u64) {
        self.gpu.compute(header, height, epoch, boundary);
    }

    fn hardware(&self) -> Hardware {
        Hardware::GPU
    }
}
