use super::event::*;
use super::native::*;
use super::native::jvmti_native::*;

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

pub fn local_event_callbacks() -> jvmtiEventCallbacks {
    jvmtiEventCallbacks {
        VMInit: None, //jvmtiEventVMInit,
        VMDeath: None, //jvmtiEventVMDeath,
        ThreadStart: Some(local_cb_thread_start), //jvmtiEventThreadStart,
        ThreadEnd: Some(local_cb_thread_end), //jvmtiEventThreadEnd,
        ClassFileLoadHook: None, //jvmtiEventClassFileLoadHook,
        ClassLoad: None, //jvmtiEventClassLoad,
        ClassPrepare: None, //jvmtiEventClassPrepare,
        VMStart: None, //jvmtiEventVMStart,
        Exception: Some(local_cb_exception), //jvmtiEventException,
        ExceptionCatch: Some(local_cb_exception_catch), //jvmtiEventExceptionCatch,
        SingleStep: None, //jvmtiEventSingleStep,
        FramePop: None, //jvmtiEventFramePop,
        Breakpoint: None, //jvmtiEventBreakpoint,
        FieldAccess: None, //jvmtiEventFieldAccess,
        FieldModification: None, //jvmtiEventFieldModification,
        MethodEntry: Some(local_cb_method_entry), //jvmtiEventMethodEntry,
        MethodExit: Some(local_cb_method_exit), //jvmtiEventMethodExit,
        NativeMethodBind: None, //jvmtiEventNativeMethodBind,
        CompiledMethodLoad: None, //jvmtiEventCompiledMethodLoad,
        CompiledMethodUnload: None, //jvmtiEventCompiledMethodUnload,
        DynamicCodeGenerated: None, //jvmtiEventDynamicCodeGenerated,
        DataDumpRequest: None, //jvmtiEventDataDumpRequest,
        reserved72: None, //jvmtiEventReserved,
        MonitorWait: Some(local_cb_monitor_wait), //jvmtiEventMonitorWait,
        MonitorWaited: Some(local_cb_monitor_waited), //jvmtiEventMonitorWaited,
        MonitorContendedEnter: Some(local_cb_monitor_contended_enter), //jvmtiEventMonitorContendedEnter,
        MonitorContendedEntered: Some(local_cb_monitor_contended_entered), //jvmtiEventMonitorContendedEntered,
        reserved77: None, //jvmtiEventReserved,
        reserved78: None, //jvmtiEventReserved,
        reserved79: None, //jvmtiEventReserved,
        ResourceExhausted: None, //jvmtiEventResourceExhausted,
        GarbageCollectionStart: None, //jvmtiEventGarbageCollectionStart,
        GarbageCollectionFinish: None, //jvmtiEventGarbageCollectionFinish,
        ObjectFree: None, //jvmtiEventObjectFree,
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
    match CALLBACK_TABLE.monitor_wait {
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
unsafe extern "C" fn local_cb_thread_end(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {

}

#[allow(unused_variables)]
unsafe extern "C" fn local_cb_thread_start(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> () {

}
