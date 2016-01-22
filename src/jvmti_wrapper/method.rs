use super::jvmti_native::jvmti_native::*;
use super::class::Class;
use super::error::NativeError;
use super::jvmti_environment::JvmtiEnvironment;

pub struct Method<'a> {
    id: jmethodID,
    env: &'a JvmtiEnvironment
}

impl<'a> Method<'a> {
    pub fn new(env: &JvmtiEnvironment, method_id: jmethodID) -> Method {
        Method {
            id: method_id,
            env: env
        }
    }

    pub fn id(&self) -> jmethodID {
        self.id
    }

    pub fn get_class(&self) -> Result<Class, NativeError> {
        self.env.get_method_declaring_class(self)
    }

    pub fn to_string(&self) -> String {
        "Method".to_string()
    }
}
