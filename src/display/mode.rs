use core::arch::asm;


pub enum ColorMode {
    Monochrome, GrayShades(u8), Colored(u8)
}


pub enum VideoModeInfo {
    Text {
        color_mode: ColorMode,
        columns: u8,
        rows: u8,
    },
    Video {
        color_mode: ColorMode,
        res: (u16, u16),
    }
}


impl VideoModeInfo {
    pub fn get_columns_count(&self) -> u8 {
        match self {
            VideoModeInfo::Text { columns, .. } => *columns,
            VideoModeInfo::Video { res, .. } => (res.0 / 13) as u8,
        }
    }
}


pub enum VideoMode {
    Other(u8),
    Defined {
        identifier: u8,
        info: VideoModeInfo,
    }
}


impl From<u8> for VideoMode {
    fn from(identifier: u8) -> Self {
        match identifier {
            0x03 => TEXT_COLOR4_80X25,
            0x13 => GRAPHICS_COLOR8_320X200,
            ident => Self::Other(ident)
        }
    }
}


pub const TEXT_COLOR4_80X25: VideoMode = VideoMode::Defined { identifier: 0x03, info: VideoModeInfo::Text { color_mode: ColorMode::Colored(4), columns: 80, rows: 25 } };
pub const GRAPHICS_COLOR8_320X200: VideoMode = VideoMode::Defined { identifier: 0x13, info: VideoModeInfo::Video { color_mode: ColorMode::Colored(8), res: (320, 200) } };


pub static mut CURRENT: Option<u8> = None;


pub fn set(mode: impl Into<VideoMode>) {
    let ident = match mode.into() {
        VideoMode::Other(identifier) => identifier,
        VideoMode::Defined { identifier, .. } => identifier,
    };

    unsafe {
        asm!(
            "int 0x10",
            in("ah") 0x00_u8,
            in("al") ident,
        );
    };

    unsafe { CURRENT = Some(ident) };
}

pub fn update_current() {
    unsafe { CURRENT = Some(*(0x449 as *const _)) };
}


pub fn res_to_colrows(res: (u16, u16)) -> (u8, u8) {
    todo!()
}
