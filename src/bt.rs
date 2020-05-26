use skyline::hooks::{getRegionAddress, Region};
use skyline::logging::hex_dump_ptr;
use std::ptr::null;
/*
define print_register_bt
  echo $pc =
  print_addr $pc $arg0
  set $cur_frame_fp = (void **)$x29
  set $cur_frame_lr = (void *)$x30
  print_addr ($cur_frame_lr-4) $arg0
  while $cur_frame_fp != 0
    set $cur_frame_lr = $cur_frame_fp[1]
    if $cur_frame_lr != 0
      print_addr ($cur_frame_lr-4) $arg0
    end
    set $cur_frame_fp = (void **)$cur_frame_fp[0]
  end
end
 */
#[inline(always)]
pub fn dump_trace() {
    unsafe {
        let txt = getRegionAddress(Region::Text) as u64;
        println!("Current txt: {:#x?}", txt);

        let mut lr = get_lr();
        let mut fp = get_fp();
        println!("Current LR: {:#x}", lr as u64);
        while fp != null() {
            lr = *fp.offset(1) as *const u64;
            if lr != null() {
                println!("LR: {:#x}", (lr as u64) - txt);
            }
            fp = (*fp as *const u64);
        }
    }
}

#[inline(always)]
fn get_lr() -> *const u64 {
    let r;
    unsafe { asm!("mov $0, x30" : "=r"(r) ::: "volatile") }
    r
}

#[inline(always)]
fn get_fp() -> *const u64 {
    let r;
    unsafe { asm!("mov $0, x29" : "=r"(r) ::: "volatile") }
    r
}
