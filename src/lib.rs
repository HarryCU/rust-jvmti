extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate time;

use agent::Agent;
use native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};
use thread::Thread;
use context::static_context;

pub mod agent;
pub mod capabilities;
pub mod class;
pub mod context;
pub mod emulator;
pub mod environment;
pub mod error;
pub mod event;
pub mod event_handler;
pub mod native;
pub mod runtime;
pub mod thread;
pub mod util;
pub mod version;

fn on_method_entry() {
    println!("Method entry");
}

fn on_method_exit() {
    println!("Method exit");
}

fn on_thread_start(thread: Thread) {
    println!("[TS-{}]", thread.name);

    static_context().thread_start(&thread.id);
}

fn on_thread_end(thread: Thread) {
    println!("[TE-{}]", thread.name);
}

fn on_monitor_wait(thread: Thread) {
    println!("[W1-{}]", thread.name);
}

fn on_monitor_waited(thread: Thread) {
    println!("[W2-{}]", thread.name);
}

fn on_monitor_contended_enter(thread: Thread) {
    println!("[C1-{}]", thread.name);
}

fn on_monitor_contended_entered(thread: Thread) {
    println!("[C2-{}]", thread.name);
}

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is directly called by the
/// Java Virtual Machine.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {

    let mut agent = Agent::new(vm);
//    agent.on_method_entry(Some(on_method_entry));
//    agent.on_method_exit(Some(on_method_exit));
    agent.on_thread_start(Some(on_thread_start));
    agent.on_thread_end(Some(on_thread_end));
    agent.on_monitor_wait(Some(on_monitor_wait));
    agent.on_monitor_waited(Some(on_monitor_waited));
    agent.on_monitor_contended_enter(Some(on_monitor_contended_enter));
    agent.on_monitor_contended_entered(Some(on_monitor_contended_entered));

    agent.update();

    return 0;
}

///
/// `Agent_OnUnload` is the exit point of the agent code. It is called when the JVM has finished
/// running and the virtual machine is unloading the agent from memory before shutting down.
/// Note: this method is also called when the JVM crashes due to an internal error.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) {
}
