use crate::port;

pub fn enable() {
    let v = port::b::r#in(0x92);
    port::b::out(0x92, v | 2);
}
