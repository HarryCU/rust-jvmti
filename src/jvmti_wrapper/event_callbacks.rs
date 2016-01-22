use super::jvmti_native::jvmti_native::*;
use super::jvmti_environment::JvmtiEnvironment;
use super::error::NativeError;
use super::method::Method;

/// The following are function type declaration for wrapped callback methods
pub type FnVMInit = extern fn() -> ();
pub type FnMethodEntry = extern fn(method: Method) -> ();
pub type FnMethodExit = extern fn(method: Method) -> ();
pub type FnVMObjectAlloc = extern fn() -> ();

pub static mut CALLBACK_TABLE: EventCallbacks = EventCallbacks {
    vm_init: None,
    vm_object_alloc: None,
    method_entry: None,
    method_exit: None
};

#[derive(Default)]
pub struct EventCallbacks {
    pub vm_init: Option<FnVMInit>,
    pub vm_object_alloc: Option<FnVMObjectAlloc>,
    pub method_entry: Option<FnMethodEntry>,
    pub method_exit: Option<FnMethodExit>
}

pub enum VMEvent {
    VMObjectAlloc = JVMTI_EVENT_VM_OBJECT_ALLOC as isize,
    VMStart = JVMTI_EVENT_VM_START as isize,
    MethodEntry = JVMTI_EVENT_METHOD_ENTRY as isize,
    MethodExit = JVMTI_EVENT_METHOD_EXIT as isize
}

impl EventCallbacks {

    pub fn new() -> EventCallbacks {
        EventCallbacks {
            ..Default::default()
        }
    }

    pub fn to_native(&self) -> jvmtiEventCallbacks {
        jvmtiEventCallbacks {
            VMInit: None, //jvmtiEventVMInit,
            VMDeath: None, //jvmtiEventVMDeath,
            ThreadStart: None, //jvmtiEventThreadStart,
            ThreadEnd: None, //jvmtiEventThreadEnd,
            ClassFileLoadHook: None, //jvmtiEventClassFileLoadHook,
            ClassLoad: None, //jvmtiEventClassLoad,
            ClassPrepare: None, //jvmtiEventClassPrepare,
            VMStart: None, //jvmtiEventVMStart,
            Exception: None, //jvmtiEventException,
            ExceptionCatch: None, //jvmtiEventExceptionCatch,
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
            MonitorWait: None, //jvmtiEventMonitorWait,
            MonitorWaited: None, //jvmtiEventMonitorWaited,
            MonitorContendedEnter: None, //jvmtiEventMonitorContendedEnter,
            MonitorContendedEntered: None, //jvmtiEventMonitorContendedEntered,
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
}

unsafe extern "C" fn local_cb_vm_object_alloc(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, object: jobject, object_klass: jclass, size: jlong) -> () {
    match CALLBACK_TABLE.vm_object_alloc {
        Some(function) => function(),
        None => println!("No dynamic callback method was found for VM object allocation")
    }
}

unsafe extern "C" fn local_cb_method_entry(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID) -> () {
    match CALLBACK_TABLE.method_entry {
        Some(function) => function(Method::new(&JvmtiEnvironment::new(jvmti_env), method)),
        None => println!("No dynamic callback method was found for method entry")
    }
}

unsafe extern "C" fn local_cb_method_exit(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread, method: jmethodID, was_popped_by_exception: jboolean, return_value: jvalue) -> () {
    match CALLBACK_TABLE.method_exit {
        Some(function) => function(Method::new(&JvmtiEnvironment::new(jvmti_env), method)),
        None => println!("No dynamic callback method was found for method exit")
    }
}
