#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![no_std]
#![no_main]


mod display;
mod keyboard;
mod panic;
mod control;
mod fs;
mod disk;
mod mbr;
mod mm;
mod port;
mod reg;
mod externs;

use display::text;
use crate::mbr::MbrRecord;

const KERNEL_ADDR: usize = 0x1000000;


#[no_mangle]
pub extern "C" fn main(boot_disk_id: u8, mbr_record: &MbrRecord) -> ! {
    display::mode::set(display::mode::TEXT_COLOR4_80X25);
    
    text::print_string("boot disk id: ");
    text::print_u16(boot_disk_id as u16);
    text::print_newline();
    
    text::print_string("enabling a20... ");
    mm::a20::enable();
    text::print_string("enabled\r\n");
    
    text::print_string("finding bial partition... ");
    let (part_index, part) = mbr_record.partitions.iter().enumerate().find(|(_, part)| part.sys_id == 0x3b).unwrap_or_else(|| control::abort("bial partition was not found"));
    text::print_string("found - #");
    text::print_u16(part_index as u16);
    text::print_newline();

    text::print_string("loading kernel into memory... ");
    
    let drive = fs::sofs::SOFSDrive::new(boot_disk_id);
    let fs = sulphur_fs::SulphurFS::new(&drive, part.start_lba_addr, part.blocks_count).unwrap_or_else(|_| control::abort("failed to initialise fs driver for bial partition"));
    
    let mut buffer = [0x00; 4096];
    if fs.get_meta().expect("getting fs meta").cluster_size != 8 {
        control::abort("unsupported cluster size in bial partition")
    };
    
    let root_dir = fs.get_root_dir().expect("getting root dir");
    
    match root_dir.find_file(&mut buffer, "kernel.px").expect("searching kernel file") {
        Some(file) => {
            file.read(&mut buffer, unsafe { core::slice::from_raw_parts_mut(KERNEL_ADDR as *mut _, file.size() as usize) })
                .unwrap_or_else(|_| control::abort("failed to read kernel file"));
        },
        None => control::abort("kernel was not found")
    };

    text::print_string("loaded\r\n");

    text::print_string("launching kernel");

    let kernel_main = unsafe { core::mem::transmute::<_, extern "C" fn() -> !>(KERNEL_ADDR) };
    kernel_main()
}
