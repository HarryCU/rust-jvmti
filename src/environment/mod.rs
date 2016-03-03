use self::jvmti::{JVMTI, JVMTIEnvironment};
use self::jni::{JNI, JNIEnvironment};
use super::capabilities::Capabilities;
use super::class::ClassId;
use super::native::JavaObject;
use super::version::VersionNumber;


pub mod jni;
pub mod jvm;
pub mod jvmti;

pub struct Environment {

    jvmti: JVMTIEnvironment,
    jni: JNIEnvironment
}

impl JVMTI for Environment {

    fn get_version_number(&self) -> VersionNumber {
        self.jvmti.get_version_number()
    }

    fn get_capabilities(&self) -> Capabilities {
        self.jvmti.get_capabilities()
    }
}

impl JNI for Environment {

    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        self.jni.get_object_class(object_id)
    }
}
