use sulphur_fs::drive;
use crate::disk;

pub struct SOFSDrive {
    disk_id: u8,
}


impl SOFSDrive {
    pub fn new(id: u8) -> Self {
        Self {
            disk_id: id,
        }
    }
}


impl drive::Drive for SOFSDrive {
    fn read(&self, addr: u32) -> drive::DErr<[u8; 512]> {
        let mut buffer = [0x00; 512];

        disk::read(self.disk_id, addr as u64, 1, &mut buffer).map_err(|_| drive::DriveIOError::AddressOutOfBoundsError)?;

        Ok(buffer)
    }

    fn write(&self, addr: u32, data: &[u8; 512]) -> drive::DErr<()> {
        todo!()
    }
} 
