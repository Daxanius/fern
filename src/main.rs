#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fern::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use fern::memory::BootInfoFrameAllocator;
use fern::{allocator, memory, print, println};
use x86_64::{VirtAddr, structures::paging::Page};

extern crate alloc;

// Defines our entry point in the bootloader such that it can do type checking with the parameters it gives
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    fern::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("The Fern kernel by Daxanius :3");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::with_capacity(500);
    vec.reserve(500);
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    // […] call `test_main` in test context
    println!("It did not crash!");
    fern::hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fern::hlt_loop();
}

// Test panic that is called
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fern::test_panic_handler(info)
}
