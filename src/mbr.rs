use crate::disk::ChsAddress;


#[repr(C, packed)]
pub struct MbrRecord {
    pub bootstrap: [u8; 440],
    pub disk_id: u32,
    pub record_flag: u16,
    pub partitions: [PartitionRecord; 4],
    pub signature: u16,
}


#[repr(C, packed)]
pub struct PartitionRecord {
    pub attributes: PartitionAttributes,
    pub start_chs_addr: ChsAddress,
    pub sys_id: u8,
    pub end_chs_addr: ChsAddress,
    pub start_lba_addr: u32,
    pub blocks_count: u32,
}


pub struct PartitionAttributes(u8);


impl PartitionAttributes {
    pub fn is_set<const BIT: u8>(&self) -> bool {
        self.0 & (1 << BIT) != 0
    }

    pub fn set<const BIT: u8>(&mut self, state: bool) {
        if state {
            self.0 |= 1 << BIT;
        } else {
            self.0 &= 0xFF ^ (1 << BIT);
        };
    }
}

