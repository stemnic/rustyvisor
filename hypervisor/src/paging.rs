// TODO (enhancement):
// if we run more rich guest OS or add more rich features to hypervisor,
// we need to refine this implmentation :-D

use crate::memlayout::{DRAM_END, PAGE_SIZE, heap_end};

// VirtualAddress
/////

#[derive(Debug)]
pub struct VirtualAddress {
    addr: usize,
}

impl VirtualAddress {
    pub fn new(addr: usize) -> VirtualAddress {
        VirtualAddress { addr: addr }
    }

    pub fn new_from_vpn(vpn : [usize; 3]) -> VirtualAddress {
        let addr = 
            (vpn[2]) << 30 |
            (vpn[1]) << 21 |
            (vpn[0]) << 12 
        ;
        VirtualAddress { addr: addr }
    }

    pub fn to_vpn(&self) -> [usize; 3] {
        [
            (self.addr >> 12) & 0x1ff, //L0 9bit
            (self.addr >> 21) & 0x1ff, //L1 9bit
            (self.addr >> 30) & 0x3ff, //L2 11bit
        ]
    }

    pub fn to_offset(&self) -> usize {
        self.addr & 0x3ff //Offsett 12bit
    }

    pub fn to_usize(&self) -> usize {
        self.addr
    }

    pub fn as_pointer(&self) -> *mut usize {
        self.addr as *mut usize
    }
}

// PhysicalAddress
/////

#[derive(Copy, Clone, Debug)]
pub struct PhysicalAddress {
    addr: usize,
}

impl PhysicalAddress {
    pub fn new(addr: usize) -> PhysicalAddress {
        PhysicalAddress { addr: addr }
    }

    pub fn to_ppn(&self) -> usize {
        self.addr >> 12 //ppn 44bit
    }

    pub fn to_ppn_array(&self) -> [usize; 3] {
        [
            (self.addr >> 12) & 0x1ff,      //L0 9bit
            (self.addr >> 21) & 0x1ff,      //L1 9bit
            (self.addr >> 30) & 0x3ff_ffff, //L2 26bit
        ]
    }

    pub fn to_usize(&self) -> usize {
        self.addr
    }

    pub fn as_pointer(&self) -> *mut usize {
        self.addr as *mut usize
    }
}

// Page
/////

#[derive(Copy, Clone, Debug)]
pub struct Page {
    addr: PhysicalAddress,
}

impl Page {
    pub fn from_address(addr: PhysicalAddress) -> Page {
        Page { addr: addr }
    }

    pub fn address(&self) -> PhysicalAddress {
        self.addr
    }
    /// Clears allocated memory for page
    pub fn clear(&self) {
        unsafe {
            let ptr = self.addr.as_pointer();
            for i in 0..512 { 
                ptr.add(i).write(0)
            }
        }
    }
}

// Page Allocator (soooo tiny version)
/////

static mut base_addr: usize = 0;
static mut last_index: usize = 0;
static mut initialized: bool = false;

pub fn init() {
    unsafe {
        base_addr = (heap_end() & !(0xfff as usize)) + 4096; // Mock heap that is page aligned
        last_index = 0;
        initialized = true;
    }
}

pub fn set_alloc_base(addr: usize) {
    unsafe {
        base_addr = addr;
    }
}

/// Allocated a page in the mock heap at the end of the elf
pub fn alloc() -> Page {
    // TODO: this unsafe block is evil!
    unsafe {
        if !initialized {
            panic!("page manager was used but not initialized");
        }

        last_index += 1;
        let addr = base_addr + (PAGE_SIZE as usize) * (last_index - 1);
        if addr > DRAM_END {
            panic!("memory exhausted; 0x{:016x}", addr)
        }
        let p = Page::from_address(PhysicalAddress::new(addr));
        p.clear();
        p
    }
}

/// Makes sure the root page follows a 16KiB boundry
pub fn alloc_16() -> Page {
    let mut root_page = alloc();
    while root_page.address().to_usize() & (0b11_1111_1111_1111 as usize) > 0 {
        log::debug!(
            "a page 0x{:016x} was allocated, but it does not follow 16KiB boundary. drop.",
            root_page.address().to_usize()
        );
        root_page = alloc();
    }
    alloc();
    alloc();
    alloc();
    root_page
}

pub fn alloc_continuous(num: usize) -> Page {
    if num <= 0 {
        panic!("invalid arg for alloc_contenious: {}", num);
    }

    let first = alloc();
    for _ in 0..(num - 1) {
        let _ = alloc();
    }

    first
}

// Page Table
/////

#[derive(Debug)]
struct PageTableEntry {
    pub ppn: [usize; 3],
    pub flags: u16,
}

pub enum PageTableEntryFlag {
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Access = 1 << 6,
    Dirty = 1 << 7,
    // TODO (enhancement): RSW
}

impl PageTableEntry {
    pub fn from_value(v: usize) -> PageTableEntry {
        let ppn = [   (v >> 10) & 0x1ff,       // PPN[0] 9 bit
                                (v >> 19) & 0x1ff,       // PPN[1] 9 bit
                                (v >> 28) & 0x3ff_ffff]; // PPN[2] 26 bit
        PageTableEntry {
            ppn: ppn,
            flags: (v & (0x1ff as usize)) as u16, // flags 8 bit (seems like this is 9?)
        }
    }

    pub unsafe fn from_memory(paddr: PhysicalAddress) -> PageTableEntry {
        let ptr = paddr.as_pointer();
        let entry = *ptr;
        PageTableEntry::from_value(entry)
    }

    pub fn to_usize(&self) -> usize {
        (if (self.ppn[2] >> 25) & 1 > 0 {
            0x3ff << 54
        } else {
            0
        }) | ((self.ppn[2] as usize) << 28)
            | ((self.ppn[1] as usize) << 19)
            | ((self.ppn[0] as usize) << 10)
            | (self.flags as usize)
    }

    pub fn next_page(&self) -> Page {
        Page::from_address(PhysicalAddress::new(
            (self.ppn[2] << 30) | (self.ppn[1] << 21) | (self.ppn[0] << 12),
        ))
    }

    pub fn set_flag(&mut self, flag: PageTableEntryFlag) {
        self.flags |= flag as u16;
    }

    pub fn is_valid(&self) -> bool {
        self.flags & (PageTableEntryFlag::Valid as u16) != 0
    }
    // A leaf has one or more RWX bits set
	pub fn is_leaf(&self) -> bool {
		self.flags & 0xe != 0
	}

	pub fn is_branch(&self) -> bool {
		!self.is_leaf()
	}
}

pub struct PageTable {
    pub page: Page,
}

// TODO (enhancement): this naming is not so good.
// This implementation assumes the paging would be done with Sv39,
// but the word 'page table" is a more general idea.
// We can rename this like `Sv39PageTable` or change the implementation in a more polymorphic way.
impl PageTable {
    fn set_entry(&self, i: usize, entry: PageTableEntry) {
        let ptr = self.page.address().as_pointer() as *mut usize;
        unsafe { ptr.add(i).write(entry.to_usize()) }
    }

    fn get_entry(&self, i: usize) -> PageTableEntry {
        let ptr = self.page.address().as_pointer() as *mut usize;
        unsafe { PageTableEntry::from_value(ptr.add(i).read()) }
    }

    pub fn from_page(page: Page) -> PageTable {
        PageTable { page: page }
    }

    pub fn resolve(&self, vaddr: &VirtualAddress) -> PhysicalAddress {
        self.resolve_intl(vaddr, self, 2)
    }

    fn resolve_intl(
        &self,
        vaddr: &VirtualAddress,
        pt: &PageTable,
        level: usize,
    ) -> PhysicalAddress {
        let vpn = vaddr.to_vpn();

        let entry = pt.get_entry(vpn[level]);
        if !entry.is_valid() {
            panic!("failed to resolve vaddr: 0x{:016x}", vaddr.addr)
        }

        if level == 0 {
            let addr_base = entry.next_page().address().to_usize();
            PhysicalAddress::new(addr_base | vaddr.to_offset())
        } else {
            let next_page = entry.next_page();
            let new_pt = PageTable::from_page(next_page);
            self.resolve_intl(vaddr, &new_pt, level - 1)
        }
    }

    pub fn map(&self, vaddr: VirtualAddress, dest: &Page, perm: u16) {
        self.map_intl(vaddr, dest, self, perm, 2)
    }

    fn map_intl(
        &self,
        vaddr: VirtualAddress,
        dest: &Page,
        pt: &PageTable,
        perm: u16,
        level: usize,
    ) {
        let vpn = vaddr.to_vpn();

        if level == 0 {
            // register `dest`  addr
            let new_entry = PageTableEntry::from_value(
                ((dest.address().to_usize() as i64 >> 2) as usize)
                    | (PageTableEntryFlag::Valid as usize)
                    | (PageTableEntryFlag::Dirty as usize)
                    | (PageTableEntryFlag::Access as usize)
                    | (perm as usize),
            );
            pt.set_entry(vpn[0], new_entry);
        } else {
            // walk the page table
            let entry = pt.get_entry(vpn[level]);
            if !entry.is_valid() {
                // if no entry found, create new page and assign it.
                let new_page = alloc();
                let new_entry = PageTableEntry::from_value(
                    ((new_page.address().to_usize() as i64 >> 2) as usize)
                        | (PageTableEntryFlag::Valid as usize),
                );
                pt.set_entry(vpn[level], new_entry);
                let new_pt = PageTable::from_page(new_page);
                self.map_intl(vaddr, dest, &new_pt, perm, level - 1);
            } else {
                let next_page = entry.next_page();
                let new_pt = PageTable::from_page(next_page);
                self.map_intl(vaddr, dest, &new_pt, perm, level - 1);
            };
        }
    }

    fn print_walk_page_table(&self, next_pt:PageTable, level: usize, vpn: [usize ; 3]) -> usize {
        // Very messy but it works
        let mut physical_addr = 0;
        let mut virtual_addr = 0;
        let mut total_pages = 0;
        let mut total_valid_entries = 0;
        for i in 0..512{
            let entry = next_pt.get_entry(i);
            if entry.is_valid(){
                total_valid_entries = total_valid_entries + 1;
                if level == 0 {
                    let new_physical_addr = entry.to_usize() << 2 & !0x3ff ;
                    //print!("vpn:{:?} ppn:{:?} entry: {:?}, physical 0x{:x} ", vpn, entry.ppn, entry, new_physical_addr);
                    if new_physical_addr - PAGE_SIZE as usize == physical_addr {
                        //print!("SAME ");
                        virtual_addr = VirtualAddress::new_from_vpn(vpn).to_usize() + i * PAGE_SIZE as usize;
                        //log::info!("Virt: 0x{:x} => Phys: 0x{:x}", virtual_addr, new_physical_addr);
                        physical_addr = new_physical_addr;
                        total_pages = total_pages + 1;
                    } else {
                        if total_pages != 0 {
                            log::info!("...");
                            log::info!("Virt: 0x{:x} => Phys: 0x{:x}", virtual_addr, physical_addr); 
                            log::info!("Num pages after each other: {}", total_pages);
                            log::info!("")   
                        }else{
                            log::info!("")
                        }
                        physical_addr = new_physical_addr;
                        total_pages = total_pages + 1;
                        virtual_addr = VirtualAddress::new_from_vpn(vpn).to_usize() + i * PAGE_SIZE as usize;
                        log::info!("Virt: 0x{:x} => Phys: 0x{:x}", virtual_addr, new_physical_addr);
                        //log::info!("");
                        total_pages = 0;
                    }
                } else {
                    let mut vpn = vpn;
                    vpn[level] = i;
                    let next_page = entry.next_page();
                    let new_pt = PageTable::from_page(next_page);
                    total_valid_entries = total_valid_entries + self.print_walk_page_table(new_pt, level - 1, vpn);
                }
            }
        }
        if total_pages != 0 {
            log::info!("...");
            log::info!("Virt: 0x{:x} => Phys: 0x{:x}", virtual_addr, physical_addr); 
            log::info!("Num pages after each other: {}", total_pages);
            log::info!("")
        }
        total_valid_entries
    }

    pub fn print_page_allocations(&self){
        // Walking all the entries in the pagetable
        // assumes Sv39
        if unsafe {initialized} {
            let pt = PageTable::from_page(self.page);
            log::info!("");
            log::info!("PAGE ALLOCATION TABLE");
            log::info!("ALLOCATED: 0x{:x} -> 0x{:x}",
                     unsafe{base_addr}, 
                     unsafe{base_addr + (PAGE_SIZE as usize) * (last_index - 1)}
            );
            log::info!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            let vpn = [0 ; 3];
            let total_entries = self.print_walk_page_table(pt, 2, vpn);
            //log::info!("{:?}", pagemapping);

            log::info!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            log::info!(
                    "Allocated: {:>6} pages ({:>10} bytes).",
                    total_entries,
                    total_entries * PAGE_SIZE as usize
            );
            /* 
            log::info!(
                     "Free     : {:>6} pages ({:>10} bytes).",
                     num_pages - num,
                     (num_pages - num) * PAGE_SIZE
            );
            */
            log::info!("");
            
        } else {
            log::info!("Pageing not initilized");
        }

    }
}

/*
/// Print all page allocations
/// This is mainly used for debugging.

pub fn print_page_allocations() {
	unsafe {
		let num_pages = last_index+1;
		let mut beg = base_addr as *const PageTableEntry;
		let end = beg.add(num_pages);
		let alloc_beg = base_addr;
		let alloc_end = base_addr + num_pages * PAGE_SIZE as usize;
		println!();
		println!(
		         "PAGE ALLOCATION TABLE\nMETA: {:p} -> {:p}\nPHYS: \
		          0x{:x} -> 0x{:x}",
		         beg, end, alloc_beg, alloc_end
		);
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
		let mut num = 0;
		while beg < end {
			if (*beg).is_taken() {
				let start = beg as usize;
				let memaddr = base_addr
				              + (start - HEAP_START)
				                * PAGE_SIZE;
				print!("0x{:x} => ", memaddr);
				loop {
					num += 1;
					if (*beg).is_last() {
						let end = beg as usize;
						let memaddr = base_addr
						              + (end - HEAP_START)
						                * PAGE_SIZE
						              + PAGE_SIZE - 1;
						print!(
						       "0x{:x}: {:>3} page(s)",
						       memaddr,
						       (end - start + 1)
						);
						println!(".");
						break;
					}
					beg = beg.add(1);
				}
			}
			beg = beg.add(1);
		}
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
		println!(
		         "Allocated: {:>6} pages ({:>10} bytes).",
		         num,
		         num * PAGE_SIZE
		);
		println!(
		         "Free     : {:>6} pages ({:>10} bytes).",
		         num_pages - num,
		         (num_pages - num) * PAGE_SIZE
		);
		println!();
	}
}
*/