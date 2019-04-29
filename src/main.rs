#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, link_args, alloc_error_handler)]

#[allow(unused_attributes)]
#[link_args = "/NODEFAULTLIB /SUBSYSTEM:CONSOLE /SAFESEH:NO /DYNAMICBASE:NO /ENTRY:mainCRTStartup /LTCG support/msvcrt.lib vcruntime.lib"]
extern "C" {}

extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::hint::unreachable_unchecked;
use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
use winapi::um::processthreadsapi::ExitProcess;

struct SystemAllocator;
unsafe impl GlobalAlloc for SystemAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // tbh in a 64k or lower you could probably get away with just leaking memory here
        HeapFree(GetProcessHeap(), 0, ptr as *mut _);
    }
}

#[global_allocator]
static A: SystemAllocator = SystemAllocator;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    // A small trick that can save some size - tell the optimizer that this point is unreachable,
    // thereby allowing to to assume branches that panic never actually happen, which they
    // shouldn't in a release build. Obviously very unsafe, so you probably don't want this in
    // debug builds.
    unsafe {
        unreachable_unchecked()
    }
}

#[alloc_error_handler]
fn error_handler(_: core::alloc::Layout) -> ! {
    // Same as above
    unsafe {
        unreachable_unchecked()
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

fn exit_process(exit_code: i32) -> ! {
    unsafe {
        ExitProcess(exit_code as _);
        unreachable_unchecked()
    }
}


#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() -> i32 {
    // Do some stuff
    let a = Box::new(10);
    // Note: returning from this function actually end the process, we need to call ExitProcess explicitly:
    exit_process(*a);
}