use super::super::capabilities::Capabilities;
use super::super::class::{ClassId, ClassSignature, JavaType};
use super::super::event::{EventCallbacks, VMEvent};
use super::super::event_handler::*;
use super::super::mem::MemoryAllocation;
use super::super::method::{MethodId, MethodSignature};
use super::super::table::LocalVariableTable;
use super::super::thread::{ThreadId, Thread};
use super::super::util::stringify;
use super::super::version::VersionNumber;
use super::super::native::{MutString, MutByteArray, JavaClass, JavaObject, JavaInstance, JavaLong, JavaThread, JVMTIEnvPtr};
use super::super::native::jvmti_native::{Struct__jvmtiThreadInfo, jvmtiCapabilities};
use std::ptr;
use error::{JvmtiErrorTranslator, NativeError};

pub trait JVMTI {
    ///
    /// Return the JVM TI version number, which includes major, minor and micro version numbers.
    ///
    fn get_version_number(&self) -> VersionNumber;
    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError>;
    fn get_capabilities(&self) -> Capabilities;
    /// Set the functions to be called for each event. The callbacks are specified by supplying a
    /// replacement function table. The function table is copied--changes to the local copy of the
    /// table have no effect. This is an atomic action, all callbacks are set at once. No events
    /// are sent before this function is called. When an entry is None no event is sent.
    /// An event must be enabled and have a callback in order to be sent--the order in which this
    /// function and set_event_notification_mode are called does not affect the result.
    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError>;
    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError>;
    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError>;
    fn get_method_declaring_class(&self, method_id: &MethodId) -> Result<ClassId, NativeError>;
    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError>;
    fn get_argument_size(&self, method_id: &MethodId) -> Result<i32, NativeError>;
    fn get_local_variable_table(&self, method_id: &MethodId) -> Result<LocalVariableTable, NativeError>;
    fn get_frame_location(&self, thread_id: &ThreadId, method_id: &MethodId, depth: i32) -> Result<i32, NativeError>;
    fn get_local_object(&self, thread_id: &ThreadId, depth: i32, slot: i32) -> Result<JavaObject, NativeError>;
    fn get_tag(&self, object_id: &JavaObject) -> Result<i32, NativeError>;
    fn get_class_signature(&self, class_id: &ClassId) -> Result<ClassSignature, NativeError>;
    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError>;
    fn deallocate(&self, mem_ptr: MutByteArray) -> Option<NativeError>;
}

pub struct JVMTIEnvironment {
    jvmti: JVMTIEnvPtr
}

impl JVMTIEnvironment {
    pub fn new(env_ptr: JVMTIEnvPtr) -> JVMTIEnvironment {
        JVMTIEnvironment { jvmti: env_ptr }
    }
}

impl JVMTI for JVMTIEnvironment {
    fn get_version_number(&self) -> VersionNumber {
        unsafe {
            let mut version: i32 = 0;
            let version_ptr = &mut version;
            (**self.jvmti).GetVersionNumber.unwrap()(self.jvmti, version_ptr);
            let uversion = *version_ptr as u32;
            VersionNumber::from_u32(&uversion)
        }
    }

    fn add_capabilities(&mut self, new_capabilities: &Capabilities) -> Result<Capabilities, NativeError> {
        let native_caps = new_capabilities.to_native();
        let caps_ptr: *const jvmtiCapabilities = &native_caps;

        unsafe {
            match (**self.jvmti).AddCapabilities.unwrap()(self.jvmti, caps_ptr).translate() {
                NativeError::NoError => Ok(self.get_capabilities()),
                err @ _ => Err(err)
            }
        }
    }

    fn get_capabilities(&self) -> Capabilities {
        unsafe {
            let caps = Capabilities::new();
            let mut native_caps = caps.to_native();
            {
                let cap_ptr = &mut native_caps;
                (**self.jvmti).GetCapabilities.unwrap()(self.jvmti, cap_ptr);
            }
            Capabilities::from_native(&native_caps)
        }
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        register_vm_init_callback(callbacks.vm_init);
        register_vm_start_callback(callbacks.vm_start);
        register_vm_death_callback(callbacks.vm_death);
        register_vm_object_alloc_callback(callbacks.vm_object_alloc);
        register_vm_object_free_callback(callbacks.vm_object_free);
        register_method_entry_callback(callbacks.method_entry);
        register_method_exit_callback(callbacks.method_exit);
        register_thread_start_callback(callbacks.thread_start);
        register_thread_end_callback(callbacks.thread_end);
        register_exception_callback(callbacks.exception);
        register_exception_catch_callback(callbacks.exception_catch);
        register_monitor_wait_callback(callbacks.monitor_wait);
        register_monitor_waited_callback(callbacks.monitor_waited);
        register_monitor_contended_enter_callback(callbacks.monitor_contended_enter);
        register_monitor_contended_endered_callback(callbacks.monitor_contended_entered);
        register_field_access_callback(callbacks.field_access);
        register_field_modification_callback(callbacks.field_modification);
        register_garbage_collection_start(callbacks.garbage_collection_start);
        register_garbage_collection_finish(callbacks.garbage_collection_finish);
        register_class_file_load_hook(callbacks.class_file_load_hook);
        register_compiled_method_load_hook(callbacks.compiled_method_load);
        register_compiled_method_unload_hook(callbacks.compiled_method_unload);
        register_dynamic_code_generated_hook(callbacks.dynamic_code_generated);

        let (native_callbacks, callbacks_size) = registered_callbacks();

        unsafe {
            match (**self.jvmti).SetEventCallbacks.unwrap()(self.jvmti, &native_callbacks, callbacks_size).translate() {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        unsafe {
            let mode_i = match mode {
                true => 1,
                false => 0
            };
            let sptr: JavaObject = ptr::null_mut();

            match (**self.jvmti).SetEventNotificationMode.unwrap()(self.jvmti, mode_i, event as u32, sptr).translate() {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        let mut info = Struct__jvmtiThreadInfo { name: ptr::null_mut(), priority: 0, is_daemon: 0, thread_group: ptr::null_mut(), context_class_loader: ptr::null_mut() };
        let mut info_ptr = &mut info;

        unsafe {
            match (**self.jvmti).GetThreadInfo {
                Some(func) => {
                    match func(self.jvmti, *thread_id, info_ptr).translate() {
                        NativeError::NoError => Ok(Thread {
                            id: ThreadId { native_id: *thread_id },
                            name: stringify((*info_ptr).name),
                            priority: (*info_ptr).priority as u32,
                            is_daemon: if (*info_ptr).is_daemon > 0 { true } else { false },
                        }),
                        err @ _ => Err(err)
                    }
                }
                None => Err(NativeError::NoError)
            }
        }
    }

    fn get_method_declaring_class(&self, method_id: &MethodId) -> Result<ClassId, NativeError> {
        let mut jstruct: JavaInstance = JavaInstance { _hacky_hack_workaround: 0 };
        let mut jclass_instance: JavaClass = &mut jstruct;
        let meta_ptr: *mut JavaClass = &mut jclass_instance;

        unsafe {
            match (**self.jvmti).GetMethodDeclaringClass.unwrap()(self.jvmti, method_id.native_id, meta_ptr).translate() {
                NativeError::NoError => Ok(ClassId { native_id: *meta_ptr }),
                err @ _ => Err(err)
            }
        }
    }

    fn get_method_name(&self, method_id: &MethodId) -> Result<MethodSignature, NativeError> {
        let mut method_name = ptr::null_mut();
        let mut method_ptr = &mut method_name;

        let mut signature: MutString = ptr::null_mut();
        let mut signature_ptr = &mut signature;

        let mut generic_sig: MutString = ptr::null_mut();
        let mut generic_sig_ptr = &mut generic_sig;

        unsafe {
            match (**self.jvmti).GetMethodName.unwrap()(self.jvmti, method_id.native_id, method_ptr, signature_ptr, generic_sig_ptr).translate() {
                NativeError::NoError => Ok(MethodSignature::new(stringify(*method_ptr))),
                err @ _ => Err(err)
            }
        }
    }

    fn get_argument_size(&self, method_id: &MethodId) -> Result<i32, NativeError> {
        let mut size = 0;
        let mut size_ptr = &mut size;
        unsafe {
            match (**self.jvmti).GetArgumentsSize.unwrap()(self.jvmti, method_id.native_id, size_ptr).translate() {
                NativeError::NoError => Ok(*size_ptr),
                err @ _ => Err(err)
            }
        }
    }

    fn get_local_variable_table(&self, method_id: &MethodId) -> Result<LocalVariableTable, NativeError> {
        let mut entry_count = 0;
        let mut entry_count_ptr = &mut entry_count;

        let mut table = ptr::null_mut();
        let mut table_ptr = &mut table;

        unsafe {
            match (**self.jvmti).GetLocalVariableTable.unwrap()(self.jvmti, method_id.native_id, entry_count_ptr, table_ptr).translate() {
                NativeError::NoError => Ok(LocalVariableTable { entry: *table_ptr, count: *entry_count_ptr }),
                err @ _ => Err(err)
            }
        }
    }

    fn get_frame_location(&self, thread_id: &ThreadId, method_id: &MethodId, depth: i32) -> Result<i32, NativeError> {
        let mut method = ptr::null_mut();
        let mut method_ptr = &mut method;
        *method_ptr = method_id.native_id;

        let mut location = 0;
        let mut location_ptr = &mut location;

        unsafe {
            match (**self.jvmti).GetFrameLocation.unwrap()(self.jvmti, thread_id.native_id, depth, method_ptr, location_ptr).translate() {
                NativeError::NoError => Ok(*location_ptr),
                err @ _ => Err(err)
            }
        }
    }

    fn get_local_object(&self, thread_id: &ThreadId, depth: i32, slot: i32) -> Result<JavaObject, NativeError> {
        let mut obj = ptr::null_mut();

        unsafe {
            match (**self.jvmti).GetLocalObject.unwrap()(self.jvmti, thread_id.native_id, depth, slot, obj).translate() {
                NativeError::NoError => Ok(*obj),
                err @ _ => Err(err)
            }
        }
    }

    fn get_tag(&self, object_id: &JavaObject) -> Result<i32, NativeError> {
        let mut tag = 0;
        let mut tag_ptr = &mut tag;

        unsafe {
            match (**self.jvmti).GetTag.unwrap()(self.jvmti, *object_id, tag_ptr).translate() {
                NativeError::NoError => Ok(*tag_ptr),
                err @ _ => Err(err)
            }
        }
    }

    fn get_class_signature(&self, class_id: &ClassId) -> Result<ClassSignature, NativeError> {
        unsafe {
            let mut native_sig: MutString = ptr::null_mut();
            let mut sig: MutString = ptr::null_mut();
            let p1: *mut MutString = &mut sig;
            let p2: *mut MutString = &mut native_sig;

            match (**self.jvmti).GetClassSignature.unwrap()(self.jvmti, class_id.native_id, p1, p2).translate() {
                NativeError::NoError => Ok(ClassSignature::new(&JavaType::parse(&stringify(sig)).unwrap())),
                err @ _ => Err(err)
            }
        }
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        let size: JavaLong = len as JavaLong;
        let mut ptr: MutByteArray = ptr::null_mut();
        let mem_ptr: *mut MutByteArray = &mut ptr;

        unsafe {
            match (**self.jvmti).Allocate.unwrap()(self.jvmti, size, mem_ptr).translate() {
                NativeError::NoError => Ok(MemoryAllocation { ptr: ptr, len: len }),
                err @ _ => Err(err)
            }
        }
    }

    fn deallocate(&self, mem_ptr: MutByteArray) -> Option<NativeError> {
        unsafe {
            match (**self.jvmti).Deallocate.unwrap()(self.jvmti, mem_ptr).translate() {
                NativeError::NoError => None,
                err @ _ => Some(err)
            }
        }
    }
}
