mod buffer;

pub use buffer::*;

const HDR_BATTLE_OBJECT_MAGIC: u64 = 0x5443454A424F5F48; // H_OBJECT
const HDR_MAGIC_OFFSET:     isize = -4;

pub fn is_hdr_object(address: *const *const u64) -> bool {
    if address.is_null() {
        false
    } else {
        unsafe {
            *address.offset(HDR_MAGIC_OFFSET) == HDR_BATTLE_OBJECT_MAGIC as _
        }
    }
}