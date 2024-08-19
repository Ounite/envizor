use crate::reg;

#[repr(C, packed)]
pub struct ChsAddress([u8; 3]);

impl ChsAddress {
    pub fn head(&self) -> u8 {
        self.0[0]
    }

    pub fn sector(&self) -> u8 {
        self.0[1] & 0b11111100 >> 2
    }

    pub fn cylinder(&self) -> u16 {
        ((self.0[1] & 0b00000011) as u16) << 8 | self.0[2] as u16
    }
}


#[repr(C, packed)]
struct LBAAddressPacket {
    size: u8,
    _pad: u8,
    blocks_count: u16,
    buffer: u16,
    mem_page: u16,
    lba_addr: u64,
}


pub enum DiskReadError {
    ReadWrongBlockCount(u16),
    BufferTooSmall,
    Other(u8)
}



pub fn read(disk_id: u8, addr: u64, blocks: u16, buffer: &mut [u8]) -> Result<(), DiskReadError> {
    if buffer.len() < blocks as usize * 512 {
        return Err(DiskReadError::BufferTooSmall);
    };

    let mut packet = LBAAddressPacket {
        size: 16,
        _pad: 0x00,
        blocks_count: blocks,
        buffer: buffer.as_ptr() as u16,
        mem_page: 0,
        lba_addr: addr,
    };
    
    unsafe {
        core::arch::asm!(
            "push si",
            "mov si, cx",
            "int 0x13",
            "pop si",
            in("dl") disk_id,
            in("cx") &mut packet as *mut _ as u16,
            in("ah") 0x42_u8
        );
    };

    // xxx should it be gathered inside the branch?
    let ah_value;
    unsafe {
        core::arch::asm!(
            "mov {}, ah",
            out(reg_byte) ah_value
        );
    };
    
    if reg::is_cf_set() {
        return Err(DiskReadError::Other(ah_value));
    };
    
    if packet.blocks_count != blocks {
        return Err(DiskReadError::ReadWrongBlockCount(packet.blocks_count));
    };

    Ok(())
}
