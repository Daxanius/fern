use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

// On interrupts, the cpu can switch to a different stack, we can define 7 interrupt stacks here for different scenarios
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // Select the 0th stack as the stack for handling double interrupts

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;

            // We haven't done memory management yet, so we use a static mut as stack storage lol
            // it is important that this is mutable such that the bootloader does not map
            // this to a read-only page
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end // Write the stack end, since they are downwards growing
        };
        tss
    };
}

lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;

    GLOBAL_DESCRIPTOR_TABLE.0.load();
    unsafe {
        CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code_selector);
        load_tss(GLOBAL_DESCRIPTOR_TABLE.1.tss_selector);
    }
}
