use self::jvmti::{JVMTI, JVMTIEnvironment};
use self::jni::{JNI, JNIEnvironment};
use super::capabilities::Capabilities;
use super::class::{ClassId, ClassSignature};
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::mem::MemoryAllocation;
use super::method::{MethodId, MethodSignature};
use super::native::{JavaObject, JavaThread};
use super::thread::Thread;
use super::version::VersionNumber;
use native::MutByteArray;
use table::LocalVariableTable;
use thread::ThreadId;

pub mod jni;
pub mod jvm;
pub mod jvmti;

/// `Environment` combines the functionality of both `JNI` and `JVMTI` by wrapping an instance of
/// both and delegating the method calls to their corresponding recipients.
pub struct Environment {
    jvmti: JVMTIEnvironment,
    jni: JNIEnvironment,
}

impl Environment {
    pub fn new(jvmti: JVMTIEnvironment, jni: JNIEnvironment) -> Environment {
        Environment { jvmti: jvmti, jni: jni }
    }
}

impl JVMTI for Environment {
    fn get_version_number(&self) -> VersionNumber {
        self.jvmti.get_version_number()
    }

    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError> {
        self.jvmti.add_capabilities(new_capabilities)
    }

    fn get_capabilities(&self) -> Capabilities {
        self.jvmti.get_capabilities()
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.jvmti.set_event_callbacks(callbacks)
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.jvmti.set_event_notification_mode(event, mode)
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        self.jvmti.get_thread_info(thread_id)
    }

    fn get_method_declaring_class(&self, method_id: &MethodId) -> Result<ClassId, NativeError> {
        self.jvmti.get_method_declaring_class(method_id)
    }

    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError> {
        self.jvmti.get_method_name(method_id)
    }

    fn get_argument_size(&self, method_id: &MethodId) -> Result<i32, NativeError> {
        self.jvmti.get_argument_size(method_id)
    }

    fn get_local_variable_table(&self, method_id: &MethodId) -> Result<LocalVariableTable, NativeError> {
        self.jvmti.get_local_variable_table(method_id)
    }

    fn get_frame_location(&self, thread_id: &ThreadId, method_id: &MethodId, depth: i32) -> Result<i32, NativeError> {
        self.jvmti.get_frame_location(thread_id, method_id, depth)
    }

    fn get_local_object(&self, thread_id: &ThreadId, depth: i32, slot: i32) -> Result<JavaObject, NativeError> {
        self.jvmti.get_local_object(thread_id, depth, slot)
    }

    fn get_tag(&self, object_id: &JavaObject) -> Result<i32, NativeError> {
        self.jvmti.get_tag(object_id)
    }

    fn get_class_signature(&self, class_id: &ClassId) -> Result<ClassSignature, NativeError> {
        self.jvmti.get_class_signature(class_id)
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        self.jvmti.allocate(len)
    }

    fn deallocate(&self, mem_ptr: MutByteArray) -> Option<NativeError> {
        self.jvmti.deallocate(mem_ptr)
    }
}

impl JNI for Environment {
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        self.jni.get_object_class(object_id)
    }

    fn delete_local_ref(&self, object_id: &JavaObject) {
        self.jni.delete_local_ref(object_id)
    }
}
