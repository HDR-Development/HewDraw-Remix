//! Crate that wraps up NX syscalls
//!
//! While this particular code was taken from [skyline-ex](https://github.com/Skyline-ex/skylinex),
//! the original source of it was likely `libnx`, so thank you to all of those that have put in the work
//! so that we don't have to.
std::arch::global_asm!(include_str!("./svc.s"));

#[repr(C)]
pub struct MemoryInfo {
    pub addr: u64,
    pub size: u64,
    pub ty: u32,
    pub attr: u32,
    pub perm: u32,
    pub device_refcount: u32,
    pub ipc_recount: u32,
    pub padding: u32,
}

extern "C" {
    fn svcQueryMemory(memory_info: *mut MemoryInfo, page_info: &mut u32, addr: u64) -> u32;
    fn svcOutputDebugString(str: *const u8, len: usize) -> u32;
    fn svcGetInfo(out: *mut u64, id0: u32, handle: u32, id1: u64);
    fn svcQueryIoMapping(out: *mut u64, out_size: *mut usize, io: u64, size: usize) -> u32;
    fn svcConnectToNamedPort(out: *mut u32, name: *const u8) -> u32;
    fn svcSendSyncRequest(handle: u32) -> u32;
    fn svcCreateResourceLimit(out: *mut u32) -> u32;
    fn svcCloseHandle(handle: u32) -> u32;
    fn svcGetResourceLimitLimitValue(out: *mut i64, handle: u32, value: u32) -> u32;
    fn svcGetResourceLimitCurrentValue(out: *mut i64, handle: u32, value: u32) -> u32;
    fn svcSetResourceLimitLimitValue(handle: u32, resource: u32, value: i64) -> u32;
}

pub fn query_memory(address: u64) -> MemoryInfo {
    unsafe {
        let mut memory_info = std::mem::MaybeUninit::uninit();
        let result = svcQueryMemory(memory_info.as_mut_ptr(), &mut 0u32, address);
        if result != 0 {
            panic!(
                "svcQueryMemory should never fail, but failed with code {:#x}",
                result
            );
        }
        memory_info.assume_init()
    }
}

pub fn query_io_mapping(address: u64, size: usize) -> Result<(u64, usize), u32> {
    unsafe {
        let mut out_addr = 0u64;
        let mut out_size = 0usize;
        let result = svcQueryIoMapping(&mut out_addr, &mut out_size, address, size);
        if result != 0 {
            Err(result)
        } else {
            Ok((out_addr, out_size))
        }
    }
}

pub fn output_debug_string(string: &str) -> Result<(), u32> {
    unsafe {
        let result = svcOutputDebugString(string.as_ptr(), string.len());
        if result != 0 {
            Err(result)
        } else {
            Ok(())
        }
    }
}

pub fn get_program_id() -> u64 {
    unsafe {
        let mut id = 0u64;
        svcGetInfo(&mut id, 18, 0xffff8001, 0);
        id
    }
}

pub fn get_heap_region_address() -> *mut u8 {
    unsafe {
        let mut addr = 0u64;
        svcGetInfo(&mut addr, 4, 0xffff8001, 0);
        addr as _
    }
}

pub fn connect_to_named_port(name: &str) -> Result<u32, u32> {
    unsafe {
        let mut out = 0;
        let result = svcConnectToNamedPort(&mut out, [name, "\0"].concat().as_ptr());
        if result != 0 {
            Err(result)
        } else {
            Ok(out)
        }
    }
}

pub fn send_sync_request(handle: u32) -> Result<(), u32> {
    unsafe {
        let result = svcSendSyncRequest(handle);
        if result != 0 {
            Err(result)
        } else {
            Ok(())
        }
    }
}

pub fn get_tls() -> *mut u8 {
    unsafe {
        let tls_ptr: *mut u8;
        std::arch::asm!("mrs {}, tpidrro_el0", out(reg) tls_ptr);
        tls_ptr
    }
}

pub fn create_resource_limit() -> Result<u32, u32> {
    unsafe {
        let mut out = 0;
        let result = svcCreateResourceLimit(&mut out);
        if result != 0 {
            Err(result)
        } else {
            Ok(out)
        }
    }
}

pub fn close_handle(handle: u32) -> Result<(), u32> {
    unsafe {
        let result = svcCloseHandle(handle);
        if result != 0 {
            Err(result)
        } else {
            Ok(())
        }
    }
}

pub fn get_resource_limit_limit(handle: u32, resource: u32) -> Result<i64, u32> {
    unsafe {
        let mut out = 0;
        let result = svcGetResourceLimitLimitValue(&mut out, handle, resource);
        if result != 0 {
            Err(result)
        } else {
            Ok(out)
        }
    }
}

pub fn get_resource_limit_current(handle: u32, resource: u32) -> Result<i64, u32> {
    unsafe {
        let mut out = 0;
        let result = svcGetResourceLimitCurrentValue(&mut out, handle, resource);
        if result != 0 {
            Err(result)
        } else {
            Ok(out)
        }
    }
}

pub fn set_resource_limit_limit(handle: u32, resource: u32, value: i64) -> Result<(), u32> {
    unsafe {
        let result = svcSetResourceLimitLimitValue(handle, resource, value);
        if result != 0 {
            Err(result)
        } else {
            Ok(())
        }
    }
}
