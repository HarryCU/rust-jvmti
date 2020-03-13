use super::native::JVMTILocalVariableEntryPtr;

pub struct LocalVariableTable {
    pub entry: JVMTILocalVariableEntryPtr,
    pub count: i32,
}
