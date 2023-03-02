// use std::collections::HashMap;
use std::ffi::{ c_void};

extern crate nix;

use std::arch::asm;
use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use nix::libc::c_int;

use nix::sys::ptrace;
use nix::sys::ptrace::AddressType;
use nix::sys::signal::Signal;
use nix::sys::wait::{wait, WaitStatus};
use nix::unistd::Pid;
use nix::unistd::{fork, ForkResult};

// fn set_breakpoint(pid: Pid, addr: u64) -> i64 {
//     // Read 8 bytes from the process memory
//     let value = ptrace::read(pid, addr as *mut i8).unwrap();
//
//     // Insert breakpoint by write new values
//     let bp = (value & (i64::MAX ^ 0xFF)) | 0xCC;
//
//     unsafe {
//         ptrace::write(pid, addr as AddressType, bp as c_int).unwrap();
//     }
//
//     // Return original bytecode
//     value as i64
// }
//
// // fn restore_breakpoint(pid: Pid, addr: u64, orig_value: i64) {
// //     unsafe {
// //         // Restore original bytecode
// //         ptrace::write(pid, addr as *mut c_void, orig_value as *mut c_void).unwrap();
// //     }
// // }
//
// // fn handle_sigstop(pid: Pid, saved_values: &HashMap<u64, i64>) {
// //     let mut regs = ptrace::getregs(pid).unwrap();
// //     println!("Hit breakpoint at 0x{:x}", regs.rip - 1);
// //
// //     match saved_values.get(&(regs.rip - 1)) {
// //         Some(orig) => {
// //             restore_breakpoint(pid, regs.rip - 1, *orig);
// //
// //             // rewind rip
// //             regs.rip -= 1;
// //             ptrace::setregs(pid, regs).expect("Error rewinding RIP");
// //
// //         }
// //         _ => print!("Nothing saved here"),
// //     }
// //
// //     ptrace::cont(pid, None).expect("Restoring breakpoint failed");
// //
// // }
//
// // Code that runs only for child
fn run_child() {
    // Allows process to be traced
    ptrace::traceme().unwrap();

    // Execute binary replacing
    Command::new("ls").exec();

    exit(0);
}
//
// // Code that runs only for parent
unsafe fn run_parent(pid: Pid, breakpoints: &[u64]) {
    // let mut saved_values = HashMap::new();

    // Placing breakpoints
    wait().unwrap();
    // for addr in breakpoints.iter() {
    //     let orig = set_breakpoint(pid, *addr);
    //     saved_values.insert(*addr, orig);
    // }
    ptrace::cont(pid, None).expect("Failed continue process");
    // ptrace::step(pid,None).expect("Failed continue process");

    loop {
        // match wait() {
        //     Ok(WaitStatus::Stopped(pid_t, sig_num)) => {
        //         match sig_num {
        //             Signal::SIGTRAP => {
        //                 println!("signal stop recieved  for {:?}",pid_t);
        //             }
        //
        //             Signal::SIGSEGV => {
        //                 // let regs = ptrace::getregs(pid_t).unwrap();
        //                 println!("Segmentation fault at 0x");
        //                 break;
        //             }
        //             _ => {
        //                 println!("Some other signal - {}", sig_num);
        //                 break;
        //             }
        //         }
        //     }
        //
        //     Ok(WaitStatus::Exited(pid, exit_status)) => {
        //         println!("Process with pid: {} exited with status {}", pid, exit_status);
        //         break;
        //     }
        //
        //     Ok(status) => {
        //         println!("Received status: {:?}", status);
        //         ptrace::cont(pid, None).expect("Failed to deliver signal");
        //     }
        //
        //     Err(err) => {
        //         println!("Some kind of error - {:?}", err);
        //     }
        // }
    }
}

fn main() {
    // let buf = "Hello World\n";
    //
    // let ret1: i32;
    // let ret2: i32;
    // unsafe {
    //     asm!(
    //     "svc 0",
    //     // in("w8") 4,
    //     in("x0") 1, // stdout
    //     in("x1") buf.as_ptr(),
    //     in("x2") buf.len(),
    //     in("x16") 4,
    //     lateout("x0") ret1,
    //     lateout("x1") ret2,
    //     );
    // }
    //
    // println!("ret value {}", ret1);
    // println!("buf  {}",buf);

    // println!("write returned: {}", ret);
    // assert_eq!(x, 6);
    // set breakpoints hash map
    let breakpoints: [u64; 1] = [0x8048451];
    //
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_child();
        }

        Ok(ForkResult::Parent { child }) => unsafe {
            run_parent(child, &breakpoints);
        },

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    };
}
