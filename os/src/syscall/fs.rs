//! File and filesystem-related syscalls
use crate::memory::stack::USER_STACK;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    unsafe extern "C" {
        safe fn boot_stack_lower_bound();
    }
    let buf_address = buf as usize;
    if buf_address == 0 {
        return -1;
    }
    if buf_address < boot_stack_lower_bound as usize {
        // in the stack
        let top = USER_STACK.get_sp();
        // 用户栈大小必须为 4096，且按照 4096 字节对齐
        let bottom = top - 4096;
        if buf_address < bottom || buf_address >= top - 0x8 {
            return -1;
        }
    }
    // else {
    //     println!("buf {:#x}", buf_address);
    // }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            // panic!("Unsupported fd in sys_write!");
            -1
        }
    }
}
