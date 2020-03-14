/// A type-safe representation of possible errors
#[derive(Debug)]
pub enum NativeError {
    NoError = 0,
    InvalidThread = 10,
    InvalidThreadGroup = 11,
    InvalidPriority = 12,
    ThreadNotSuspended = 13,
    ThreadSuspended = 14,
    ThreadNotAlive = 15,
    InvalidObject = 20,
    InvalidClass = 21,
    ClassNotPrepared = 22,
    InvalidMethodId = 23,
    InvalidLocation = 24,
    InvalidFieldId = 25,
    NoMoreFrames = 31,
    OpaqueFrame = 32,
    TypeMismatch = 34,
    InvalidSlot = 35,
    Duplicate = 40,
    NotFound = 41,
    InvalidMonitor = 50,
    NotMonitorOwner = 51,
    Interrupt = 52,
    InvalidClassFormat = 60,
    CircularClassDefinition = 61,
    FailsVerification = 62,
    UnsupportedRedefinitionMethodAdded = 63,
    UnsupportedRedefinitionSchemaChanged = 64,
    InvalidTypestate = 65,
    UnsupportedRedefinitionHierarchyChanged = 66,
    UnsupportedRedefinitionMethodDeleted = 67,
    UnsupportedVersion = 68,
    NamesDontMatch = 69,
    UnsupportedRedefinitionClassModifiersChanged = 70,
    UnsupportedRedefinitionMethodModifiersChanged = 71,
    UnmodifiableClass = 79,
    NotAvailable = 98,
    MustPossessCapability = 99,
    NullPointer = 100,
    AbsentInformation = 101,
    InvalidEventType = 102,
    IllegalArgument = 103,
    NativeMethod = 104,
    ClassLoaderUnsupported = 106,
    OutOfMemory = 110,
    NotEnabled = 111,
    WrongPhase = 112,
    UnexpectedInternalError = 113,
    ThreadNotAttached = 115,
    Disconnected = 116,
    NotImplemented = 999999,
    // <- now this is a "temporary" hack until the library is under heavy development
    UnknownError,
}

/// Turn a native error code into a type-safe error
fn wrap_error(code: u32) -> NativeError {
    match code {
        0 => NativeError::NoError,
        10 => NativeError::InvalidThread,
        11 => NativeError::InvalidThreadGroup,
        12 => NativeError::InvalidPriority,
        13 => NativeError::ThreadNotSuspended,
        14 => NativeError::ThreadSuspended,
        15 => NativeError::ThreadNotAlive,
        20 => NativeError::InvalidObject,
        21 => NativeError::InvalidClass,
        22 => NativeError::ClassNotPrepared,
        23 => NativeError::InvalidMethodId,
        24 => NativeError::InvalidLocation,
        25 => NativeError::InvalidFieldId,
        31 => NativeError::NoMoreFrames,
        32 => NativeError::OpaqueFrame,
        34 => NativeError::TypeMismatch,
        35 => NativeError::InvalidSlot,
        40 => NativeError::Duplicate,
        41 => NativeError::NotFound,
        50 => NativeError::InvalidMonitor,
        51 => NativeError::NotMonitorOwner,
        52 => NativeError::Interrupt,
        60 => NativeError::InvalidClassFormat,
        61 => NativeError::CircularClassDefinition,
        62 => NativeError::FailsVerification,
        63 => NativeError::UnsupportedRedefinitionMethodAdded,
        64 => NativeError::UnsupportedRedefinitionSchemaChanged,
        65 => NativeError::InvalidTypestate,
        66 => NativeError::UnsupportedRedefinitionHierarchyChanged,
        67 => NativeError::UnsupportedRedefinitionMethodDeleted,
        68 => NativeError::UnsupportedVersion,
        69 => NativeError::NamesDontMatch,
        70 => NativeError::UnsupportedRedefinitionClassModifiersChanged,
        71 => NativeError::UnsupportedRedefinitionMethodModifiersChanged,
        79 => NativeError::UnmodifiableClass,
        98 => NativeError::NotAvailable,
        99 => NativeError::MustPossessCapability,
        100 => NativeError::NullPointer,
        101 => NativeError::AbsentInformation,
        102 => NativeError::InvalidEventType,
        103 => NativeError::IllegalArgument,
        104 => NativeError::NativeMethod,
        106 => NativeError::ClassLoaderUnsupported,
        110 => NativeError::OutOfMemory,
        111 => NativeError::NotEnabled,
        112 => NativeError::WrongPhase,
        113 => NativeError::UnexpectedInternalError,
        115 => NativeError::ThreadNotAttached,
        116 => NativeError::Disconnected,
        999999 => NativeError::NotImplemented,
        _ => {
            println!("Unknown error code was detected: {}", code);
            NativeError::UnknownError
        }
    }
}

/// Turn native error codes into meaningful and user-readable strings
fn translate_error(code: &NativeError) -> String {
    match code {
        /* 0   */ &NativeError::NoError => "No error has occurred.",
        /* 10  */ &NativeError::InvalidThread => "The passed thread is not a valid thread.",
        /* 11  */ &NativeError::InvalidThreadGroup => "Thread group invalid.",
        /* 12  */ &NativeError::InvalidPriority => "Invalid priority.",
        /* 13  */ &NativeError::ThreadNotSuspended => "Thread was not suspended.",
        /* 14  */ &NativeError::ThreadSuspended => "Thread already suspended.",
        /* 15  */ &NativeError::ThreadNotAlive => "This operation requires the thread to be alive--that is, it must be started and not yet have died.",
        /* 20  */ &NativeError::InvalidObject => "Invalid object.",
        /* 21  */ &NativeError::InvalidClass => "Invalid class.",
        /* 22  */ &NativeError::ClassNotPrepared => "The class has been loaded but not yet prepared.",
        /* 23  */ &NativeError::InvalidMethodId => "Invalid method.",
        /* 24  */ &NativeError::InvalidLocation => "Invalid location.",
        /* 25  */ &NativeError::InvalidFieldId => "Invalid field.",
        /* 31  */ &NativeError::NoMoreFrames => "There are no Java programming language or JNI stack frames at the specified depth.",
        /* 32  */ &NativeError::OpaqueFrame => "Information about the frame is not available (e.g. for native frames).",
        /* 34  */ &NativeError::TypeMismatch => "The variable is not an appropriate type for the function used.",
        /* 35  */ &NativeError::InvalidSlot => "Invalid slot.",
        /* 40  */ &NativeError::Duplicate => "Item already set.",
        /* 41  */ &NativeError::NotFound => "Desired element (e.g. field or breakpoint) not found.",
        /* 50  */ &NativeError::InvalidMonitor => "Invalid raw monitor.",
        /* 51  */ &NativeError::NotMonitorOwner => "This thread doesn't own the raw monitor.",
        /* 52  */ &NativeError::Interrupt => "The call has been interrupted before completion.",
        /* 60  */ &NativeError::InvalidClassFormat => "A new class file is malformed (the VM would return a ClassFormatError).",
        /* 61  */ &NativeError::CircularClassDefinition => "The new class file definitions would lead to a circular definition (the VM would return a ClassCircularityError).",
        /* 62  */ &NativeError::FailsVerification => "The class bytes fail verification.",
        /* 63  */ &NativeError::UnsupportedRedefinitionMethodAdded => "A new class file would require adding a method.",
        /* 64  */ &NativeError::UnsupportedRedefinitionSchemaChanged => "A new class version changes a field.",
        /* 65  */ &NativeError::InvalidTypestate => "The state of the thread has been modified, and is now inconsistent.",
        /* 66  */ &NativeError::UnsupportedRedefinitionHierarchyChanged => "A direct superclass is different for the new class version, or the set of directly implemented interfaces is different.",
        /* 67  */ &NativeError::UnsupportedRedefinitionMethodDeleted => "A new class version does not declare a method declared in the old class version.",
        /* 68  */ &NativeError::UnsupportedVersion => "A new class file has a version number not supported by this VM.",
        /* 69  */ &NativeError::NamesDontMatch => "The class name defined in the new class file is different from the name in the old class object.",
        /* 70  */ &NativeError::UnsupportedRedefinitionClassModifiersChanged => "A new class version has different modifiers.",
        /* 71  */ &NativeError::UnsupportedRedefinitionMethodModifiersChanged => "A method in the new class version has different modifiers than its counterpart in the old class version.",
        /* 79  */ &NativeError::UnmodifiableClass => "The class cannot be modified.",
        /* 98  */ &NativeError::NotAvailable => "The functionality is not available in this virtual machine.",
        /* 99  */ &NativeError::MustPossessCapability => "The capability being used is false in this environment.",
        /* 100 */ &NativeError::NullPointer => "Pointer is unexpectedly NULL.",
        /* 101 */ &NativeError::AbsentInformation => "The requested information is not available.",
        /* 102 */ &NativeError::InvalidEventType => "The specified event type ID is not recognized.",
        /* 103 */ &NativeError::IllegalArgument => "Illegal argument.",
        /* 104 */ &NativeError::NativeMethod => "The requested information is not available for native method.",
        /* 106 */ &NativeError::ClassLoaderUnsupported => "The class loader does not support this operation.",
        /* 110 */ &NativeError::OutOfMemory => "The function attempted to allocate memory and no more memory was available for allocation.",
        /* 111 */ &NativeError::NotEnabled => "The desired functionality has not been enabled in this virtual machine.",
        /* 112 */ &NativeError::WrongPhase => "The desired functionality is not available in the current phase. Always returned if the virtual machine has completed running.",
        /* 113 */ &NativeError::UnexpectedInternalError => "An unexpected internal error has occurred.",
        /* 115 */ &NativeError::ThreadNotAttached => "The thread being used to call this function is not attached to the virtual machine. Calls must be made from attached threads.",
        /* 116 */ &NativeError::Disconnected => "The JVM TI environment provided is no longer connected or is not an environment.",
        /* 999999 */ &NativeError::NotImplemented => "This function is not implemented yet",
        /*  */ &NativeError::UnknownError => "Unknown error."
    }.to_string()
}

pub trait JvmtiErrorTranslator {
    fn translate(&self) -> NativeError;
}

pub trait NativeErrorTranslator {
    fn translate(&self) -> String;
}

impl JvmtiErrorTranslator for u32 {
    fn translate(&self) -> NativeError {
        wrap_error(*self as u32)
    }
}

impl NativeErrorTranslator for NativeError {
    fn translate(&self) -> String {
        translate_error(self)
    }
}