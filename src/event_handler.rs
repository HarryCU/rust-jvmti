use super::event::*;
use super::native::*;
use super::native::jvmti_native::*;
use libc::{c_char, c_uchar, c_void};

pub static mut CALLBACK_TABLE: EventCallbacks = EventCallbacks {
    vm_init: None,
    vm_death: None,
    vm_object_alloc: None,
    vm_start: None,
    method_entry: None,
    method_exit: None,
    exception: None,
    exception_catch: None,
    monitor_wait: None,
    monitor_waited: None,
    monitor_contended_enter: None,
    monitor_contended_entered: None,
    thread_start: None,
    thread_end: None,
    field_access: None,
    field_modification: None,
    garbage_collection_start: None,
    garbage_collection_finish: None
};

///
/// Generates a native `jvmtiEventCallbacks` structure holding the local extern even handler methods.
///
pub fn local_event_callbacks() -> jvmtiEventCallbacks {
    jvmtiEventCallbacks {
        VMInit: Some(local_cb_vm_init), //jvmtiEventVMInit,
        VMDeath: Some(local_cb_vm_death), //jvmtiEventVMDeath,
        ThreadStart: Some(local_cb_thread_start), //jvmtiEventThreadStart,
        ThreadEnd: Some(local_cb_thread_end), //jvmtiEventThreadEnd,
        ClassFileLoadHook: Some(local_cb_class_file_load_hook), //jvmtiEventClassFileLoadHook,
        ClassLoad: Some(local_cb_class_load), //jvmtiEventClassLoad,
        ClassPrepare: Some(local_cb_class_prepare), //jvmtiEventClassPrepare,
        VMStart: Some(local_cb_vm_start), //jvmtiEventVMStart,
        Exception: Some(local_cb_exception), //jvmtiEventException,
        ExceptionCatch: Some(local_cb_exception_catch), //jvmtiEventExceptionCatch,
        SingleStep: Some(local_cb_single_step), //jvmtiEventSingleStep,
        FramePop: Some(local_cb_frame_pop), //jvmtiEventFramePop,
        Breakpoint: Some(local_cb_breakpoint), //jvmtiEventBreakpoint,
        FieldAccess: Some(local_cb_field_access), //jvmtiEventFieldAccess,
        FieldModification: Some(local_cb_field_modification), //jvmtiEventFieldModification,
        MethodEntry: Some(local_cb_method_entry), //jvmtiEventMethodEntry,
        MethodExit: Some(local_cb_method_exit), //jvmtiEventMethodExit,
        NativeMethodBind: Some(local_cb_native_method_bind), //jvmtiEventNativeMethodBind,
        CompiledMethodLoad: Some(local_cb_compiled_method_load), //jvmtiEventCompiledMethodLoad,
        CompiledMethodUnload: Some(local_cb_compiled_method_unload), //jvmtiEventCompiledMethodUnload,
        DynamicCodeGenerated: Some(local_cb_dynamic_code_generated), //jvmtiEventDynamicCodeGenerated,
        DataDumpRequest: Some(local_cb_data_dump_request), //jvmtiEventDataDumpRequest,
        reserved72: None, //jvmtiEventReserved,
        MonitorWait: Some(local_cb_monitor_wait), //jvmtiEventMonitorWait,
        MonitorWaited: Some(local_cb_monitor_waited), //jvmtiEventMonitorWaited,
        MonitorContendedEnter: Some(local_cb_monitor_contended_enter), //jvmtiEventMonitorContendedEnter,
        MonitorContendedEntered: Some(local_cb_monitor_contended_entered), //jvmtiEventMonitorContendedEntered,
        reserved77: None, //jvmtiEventReserved,
        reserved78: None, //jvmtiEventReserved,
        reserved79: None, //jvmtiEventReserved,
        ResourceExhausted: Some(local_cb_resource_exhausted), //jvmtiEventResourceExhausted,
        GarbageCollectionStart: Some(local_cb_garbage_collection_start), //jvmtiEventGarbageCollectionStart,
        GarbageCollectionFinish: Some(local_cb_garbage_collection_finish), //jvmtiEventGarbageCollectionFinish,
        ObjectFree: Some(local_cb_object_free), //jvmtiEventObjectFree,
        VMObjectAlloc: Some(local_cb_vm_object_alloc) //jvmtiEventVMObjectAlloc,
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_vm_object_alloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> () {
    match CALLBACK_TABLE.vm_object_alloc {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            function(size as u64) },
            */
        },
        None => println!("No dynamic callback method was found for VM object allocation")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_method_entry(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: JavaThread, method: jmethodID) -> () {
    match CALLBACK_TABLE.method_entry {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            let method_id = MethodId { native_id : method };
            let class_id = env.get_method_declaring_class(&method_id).ok().unwrap();
            let class_sig = env.get_class_signature(&class_id).ok().unwrap();

            match env.get_method_name(&method_id) {
                Ok(signature) => function(Method::new(method_id, signature), Class::new(class_id, class_sig), current_thread),
                Err(_) => function(Method::unknown(), Class::unknown(), current_thread)
            }
            */
        },
        None => println!("No dynamic callback method was found for method entry")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_method_exit(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, was_popped_by_exception: jboolean, return_value: jvalue) -> () {
    match CALLBACK_TABLE.method_exit {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let method_id = MethodId { native_id : method };
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            let class_id = env.get_method_declaring_class(&method_id).ok().unwrap();
            let class_sig = env.get_class_signature(&class_id).ok().unwrap();

            match env.get_method_name(&method_id) {
                Ok(signature) => function(Method::new(method_id, signature), Class::new(class_id, class_sig), current_thread),
                Err(_) => function(Method::unknown(), Class::unknown(), current_thread)
            }
            */
        }
        None => println!("No dynamic callback method was found for method exit")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_exception(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation, exception: JavaObject, catch_method: jmethodID, catch_location: jlocation) -> () {
    match CALLBACK_TABLE.exception {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let exception_class: Class = env.get_object_class(exception);

            function(exception_class)
            */
        },
        None => println!("No dynamic callback method was found for exception")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_exception_catch(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation, exception: jobject) -> () {
    match CALLBACK_TABLE.exception_catch {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();

            function()
            */
        },
        None => println!("No dynamic callback method was found for exception catch")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_monitor_wait(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, timeout: jlong) -> () {
    match CALLBACK_TABLE.monitor_wait {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            function(current_thread)
            */
        },
        None => println!("No dynamic callback method was found for monitor wait")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_monitor_waited(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, timed_out: jboolean) -> () {
    match CALLBACK_TABLE.monitor_waited {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            function(current_thread)
            */
        },
        None => println!("No dynamic callback method was found for monitor entered")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_monitor_contended_enter(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject) -> () {
    match CALLBACK_TABLE.monitor_contended_enter {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            function(current_thread)
            */
        },
        None => println!("No dynamic callback method was found for monitor contended enter")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_monitor_contended_entered(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject) -> () {
    match CALLBACK_TABLE.monitor_contended_entered {
        Some(function) => {
            /*
            let env = Environment::new(JVMTIEnvironment::new(jvmti_env), JNIEnvironment::new(jni_env));
            let current_thread = env.get_thread_info(&thread).ok().unwrap();
            function(current_thread)
            */
        },
        None => println!("No dynamic callback method was found for monitor contended entered")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_thread_start(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {
    match CALLBACK_TABLE.thread_start {
        Some(function) => {

        },
        None => println!("No dynamic callback method was found for thread start events")
    }

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_thread_end(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {
    match CALLBACK_TABLE.thread_end {
        Some(function) => {

        },
        None => println!("No dynamic callback method was found for thread end events")
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_garbage_collection_start(jvmti_env: *mut jvmtiEnv) -> () {
    match CALLBACK_TABLE.garbage_collection_start {
        Some(function) => {

        },
        None => println!("No dynamic callback method was found for garbage collection start events")
    }

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_garbage_collection_finish(jvmti_env: *mut jvmtiEnv) -> () {
    match CALLBACK_TABLE.garbage_collection_finish {
        Some(function) => {

        },
        None => println!("No dynamic callback method was found for garbage collection finish events")
    }

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_breakpoint(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_class_file_load_hook(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, class_being_redefined: jclass, loader: jobject,
                                                   name: *const c_char, protection_domain: jobject, class_data_len: jint, class_data: *const c_uchar,
                                                   new_class_data_len: *mut jint, new_class_data: *mut *mut c_uchar) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_class_load(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, klass: jclass) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_class_prepare(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, klass: jclass) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_compiled_method_load(jvmti_env: *mut jvmtiEnv, method: jmethodID, code_size: jint, code_addr: *const c_void, map_length: jint,
                                                   map: *const jvmtiAddrLocationMap, compile_info: *const c_void) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_compiled_method_unload(jvmti_env: *mut jvmtiEnv, method: jmethodID, code_addr: *const c_void) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_data_dump_request(jvmti_env: *mut jvmtiEnv) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_dynamic_code_generated(jvmti_env: *mut jvmtiEnv, name: *const c_char, address: *const c_void, length: jint) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_field_access(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation,
                                                   field_klass: jclass, object: jobject, field: jfieldID) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_field_modification(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation,
                                                   field_klass: jclass, object: jobject, field: jfieldID, signature_type: c_char, new_value: jvalue) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_frame_pop(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, was_popped_by_exception: jboolean) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_native_method_bind(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, address: *mut c_void,
                                                   new_address_ptr: *mut *mut c_void) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_object_free(jvmti_env: *mut jvmtiEnv, tag: jlong) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_resource_exhausted(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, flags: jint, reserved: *const c_void, description: *const c_char) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_single_step(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, location: jlocation) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_vm_death(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_vm_init(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_vm_start(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv) -> () {

}
