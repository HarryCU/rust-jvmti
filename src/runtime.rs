use super::class::{ClassId, ClassSignature};
use super::method::{MethodId, MethodSignature};
use super::thread::Thread;

pub trait RuntimeEvent {
}

pub struct ObjectAllocationEvent {
    pub class_id: ClassId,
    pub thread: Thread,
    pub size: i64
}

pub struct MethodInvocationEvent {
    pub method_id: MethodId,
    pub method_sig: MethodSignature,
    pub class_sig: ClassSignature,
    pub thread: Thread
}

impl RuntimeEvent for ObjectAllocationEvent {}
impl RuntimeEvent for MethodInvocationEvent {}
