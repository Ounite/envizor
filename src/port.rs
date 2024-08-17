macro_rules! portiomod {
    ($cat:ident, $argreg:tt, $size:ty) => {
        pub mod $cat {
            use ::core::arch::asm;

            pub fn out(port: u16, v: $size) {
                unsafe {
                    asm!(
                        concat!("out dx,", $argreg),
                        in("dx") port,
                        in($argreg) v
                    );
                };
            }

            pub fn r#in(port: u16) -> $size {
                let res;
                unsafe {
                    asm!(
                        concat!("in ", $argreg, ",dx"),
                        out($argreg) res,
                        in("dx") port,
                    );
                };
                res
            }
        }
    };
}

portiomod!(b, "al", u8);
portiomod!(w, "ax", u16);
portiomod!(dw, "eax", u32);
