/* automatically generated by rust-bindgen 0.59.1 */

pub const WINTUN_MAX_POOL: u32 = 256;
pub const MAX_ADAPTER_NAME: u32 = 128;
pub const WINTUN_MIN_RING_CAPACITY: u32 = 131072;
pub const WINTUN_MAX_RING_CAPACITY: u32 = 67108864;
pub const WINTUN_MAX_IP_PACKET_SIZE: u32 = 65535;
pub type BOOL = ::std::os::raw::c_uchar;
pub type BYTE = ::std::os::raw::c_uchar;
pub type LPARAM = *mut ::std::os::raw::c_void;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type WCHAR = ::std::os::raw::c_ushort;
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type DWORD64 = ::std::os::raw::c_ulonglong;
pub type PDWORD64 = *mut ::std::os::raw::c_ulonglong;
pub type DWORD = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GUID {
    pub __bindgen_anon_1: _GUID__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GUID__bindgen_ty_1 {
    pub __bindgen_anon_1: _GUID__bindgen_ty_1__bindgen_ty_1,
    pub Bytes: [::std::os::raw::c_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID__bindgen_ty_1__bindgen_ty_1 {
    pub a: ::std::os::raw::c_ulonglong,
    pub b: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout__GUID__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_GUID__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_GUID__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_GUID__bindgen_ty_1__bindgen_ty_1>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_GUID__bindgen_ty_1__bindgen_ty_1>())).b as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout__GUID__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_GUID__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_GUID__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID__bindgen_ty_1>())).Bytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID__bindgen_ty_1),
            "::",
            stringify!(Bytes)
        )
    );
}
#[test]
fn bindgen_test_layout__GUID() {
    assert_eq!(
        ::std::mem::size_of::<_GUID>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID>(),
        8usize,
        concat!("Alignment of ", stringify!(_GUID))
    );
}
pub type GUID = _GUID;
pub type NET_LUID = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _WINTUN_ADAPTER {
    _unused: [u8; 0],
}
pub type WINTUN_ADAPTER_HANDLE = *mut _WINTUN_ADAPTER;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TUN_SESSION {
    _unused: [u8; 0],
}
#[doc = " A handle representing Wintun session"]
pub type WINTUN_SESSION_HANDLE = *mut _TUN_SESSION;
pub type WINTUN_ENUM_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(Adapter: WINTUN_ADAPTER_HANDLE, Param: LPARAM) -> BOOL,
>;
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_INFO: WINTUN_LOGGER_LEVEL = 0;
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_WARN: WINTUN_LOGGER_LEVEL = 1;
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_ERR: WINTUN_LOGGER_LEVEL = 2;
pub type WINTUN_LOGGER_LEVEL = ::std::os::raw::c_int;
pub type WINTUN_LOGGER_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(Level: WINTUN_LOGGER_LEVEL, Timestamp: DWORD64, Message: *const WCHAR),
>;
extern crate libloading;
pub struct wintun {
    __library: ::libloading::Library,
    pub WintunCreateAdapter: Result<
        unsafe extern "C" fn(
            Name: LPCWSTR,
            TunnelType: LPCWSTR,
            RequestedGUID: *const GUID,
        ) -> WINTUN_ADAPTER_HANDLE,
        ::libloading::Error,
    >,
    pub WintunOpenAdapter:
        Result<unsafe extern "C" fn(Name: LPCWSTR) -> WINTUN_ADAPTER_HANDLE, ::libloading::Error>,
    pub WintunDeleteDriver: Result<unsafe extern "C" fn() -> BOOL, ::libloading::Error>,
    pub WintunCloseAdapter:
        Result<unsafe extern "C" fn(Adapter: WINTUN_ADAPTER_HANDLE), ::libloading::Error>,
    pub WintunGetAdapterLUID: Result<
        unsafe extern "C" fn(Adapter: WINTUN_ADAPTER_HANDLE, Luid: *mut NET_LUID),
        ::libloading::Error,
    >,
    pub WintunSetLogger:
        Result<unsafe extern "C" fn(NewLogger: WINTUN_LOGGER_CALLBACK), ::libloading::Error>,
    pub WintunGetRunningDriverVersion: Result<unsafe extern "C" fn() -> DWORD, ::libloading::Error>,
    pub WintunStartSession: Result<
        unsafe extern "C" fn(
            Adapter: WINTUN_ADAPTER_HANDLE,
            Capacity: DWORD,
        ) -> WINTUN_SESSION_HANDLE,
        ::libloading::Error,
    >,
    pub WintunEndSession:
        Result<unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE), ::libloading::Error>,
    pub WintunGetReadWaitEvent:
        Result<unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE) -> HANDLE, ::libloading::Error>,
    pub WintunReceivePacket: Result<
        unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE, PacketSize: *mut DWORD) -> *mut BYTE,
        ::libloading::Error,
    >,
    pub WintunReleaseReceivePacket: Result<
        unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE, Packet: *const BYTE),
        ::libloading::Error,
    >,
    pub WintunAllocateSendPacket: Result<
        unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE, PacketSize: DWORD) -> *mut BYTE,
        ::libloading::Error,
    >,
    pub WintunSendPacket: Result<
        unsafe extern "C" fn(Session: WINTUN_SESSION_HANDLE, Packet: *const BYTE),
        ::libloading::Error,
    >,
}
impl wintun {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let WintunCreateAdapter = __library.get(b"WintunCreateAdapter\0").map(|sym| *sym);
        let WintunOpenAdapter = __library.get(b"WintunOpenAdapter\0").map(|sym| *sym);
        let WintunDeleteDriver = __library.get(b"WintunDeleteDriver\0").map(|sym| *sym);
        let WintunCloseAdapter = __library.get(b"WintunCloseAdapter\0").map(|sym| *sym);
        let WintunGetAdapterLUID = __library.get(b"WintunGetAdapterLUID\0").map(|sym| *sym);
        let WintunSetLogger = __library.get(b"WintunSetLogger\0").map(|sym| *sym);
        let WintunGetRunningDriverVersion = __library
            .get(b"WintunGetRunningDriverVersion\0")
            .map(|sym| *sym);
        let WintunStartSession = __library.get(b"WintunStartSession\0").map(|sym| *sym);
        let WintunEndSession = __library.get(b"WintunEndSession\0").map(|sym| *sym);
        let WintunGetReadWaitEvent = __library.get(b"WintunGetReadWaitEvent\0").map(|sym| *sym);
        let WintunReceivePacket = __library.get(b"WintunReceivePacket\0").map(|sym| *sym);
        let WintunReleaseReceivePacket = __library
            .get(b"WintunReleaseReceivePacket\0")
            .map(|sym| *sym);
        let WintunAllocateSendPacket = __library.get(b"WintunAllocateSendPacket\0").map(|sym| *sym);
        let WintunSendPacket = __library.get(b"WintunSendPacket\0").map(|sym| *sym);
        Ok(wintun {
            __library,
            WintunCreateAdapter,
            WintunOpenAdapter,
            WintunDeleteDriver,
            WintunCloseAdapter,
            WintunGetAdapterLUID,
            WintunSetLogger,
            WintunGetRunningDriverVersion,
            WintunStartSession,
            WintunEndSession,
            WintunGetReadWaitEvent,
            WintunReceivePacket,
            WintunReleaseReceivePacket,
            WintunAllocateSendPacket,
            WintunSendPacket,
        })
    }
    pub unsafe fn WintunCreateAdapter(
        &self,
        Name: LPCWSTR,
        TunnelType: LPCWSTR,
        RequestedGUID: *const GUID,
    ) -> WINTUN_ADAPTER_HANDLE {
        (self
            .WintunCreateAdapter
            .as_ref()
            .expect("Expected function, got error."))(Name, TunnelType, RequestedGUID)
    }
    pub unsafe fn WintunOpenAdapter(&self, Name: LPCWSTR) -> WINTUN_ADAPTER_HANDLE {
        (self
            .WintunOpenAdapter
            .as_ref()
            .expect("Expected function, got error."))(Name)
    }
    pub unsafe fn WintunDeleteDriver(&self) -> BOOL {
        (self
            .WintunDeleteDriver
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn WintunCloseAdapter(&self, Adapter: WINTUN_ADAPTER_HANDLE) -> () {
        (self
            .WintunCloseAdapter
            .as_ref()
            .expect("Expected function, got error."))(Adapter)
    }
    pub unsafe fn WintunGetAdapterLUID(
        &self,
        Adapter: WINTUN_ADAPTER_HANDLE,
        Luid: *mut NET_LUID,
    ) -> () {
        (self
            .WintunGetAdapterLUID
            .as_ref()
            .expect("Expected function, got error."))(Adapter, Luid)
    }
    pub unsafe fn WintunSetLogger(&self, NewLogger: WINTUN_LOGGER_CALLBACK) -> () {
        (self
            .WintunSetLogger
            .as_ref()
            .expect("Expected function, got error."))(NewLogger)
    }
    pub unsafe fn WintunGetRunningDriverVersion(&self) -> DWORD {
        (self
            .WintunGetRunningDriverVersion
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn WintunStartSession(
        &self,
        Adapter: WINTUN_ADAPTER_HANDLE,
        Capacity: DWORD,
    ) -> WINTUN_SESSION_HANDLE {
        (self
            .WintunStartSession
            .as_ref()
            .expect("Expected function, got error."))(Adapter, Capacity)
    }
    pub unsafe fn WintunEndSession(&self, Session: WINTUN_SESSION_HANDLE) -> () {
        (self
            .WintunEndSession
            .as_ref()
            .expect("Expected function, got error."))(Session)
    }
    pub unsafe fn WintunGetReadWaitEvent(&self, Session: WINTUN_SESSION_HANDLE) -> HANDLE {
        (self
            .WintunGetReadWaitEvent
            .as_ref()
            .expect("Expected function, got error."))(Session)
    }
    pub unsafe fn WintunReceivePacket(
        &self,
        Session: WINTUN_SESSION_HANDLE,
        PacketSize: *mut DWORD,
    ) -> *mut BYTE {
        (self
            .WintunReceivePacket
            .as_ref()
            .expect("Expected function, got error."))(Session, PacketSize)
    }
    pub unsafe fn WintunReleaseReceivePacket(
        &self,
        Session: WINTUN_SESSION_HANDLE,
        Packet: *const BYTE,
    ) -> () {
        (self
            .WintunReleaseReceivePacket
            .as_ref()
            .expect("Expected function, got error."))(Session, Packet)
    }
    pub unsafe fn WintunAllocateSendPacket(
        &self,
        Session: WINTUN_SESSION_HANDLE,
        PacketSize: DWORD,
    ) -> *mut BYTE {
        (self
            .WintunAllocateSendPacket
            .as_ref()
            .expect("Expected function, got error."))(Session, PacketSize)
    }
    pub unsafe fn WintunSendPacket(
        &self,
        Session: WINTUN_SESSION_HANDLE,
        Packet: *const BYTE,
    ) -> () {
        (self
            .WintunSendPacket
            .as_ref()
            .expect("Expected function, got error."))(Session, Packet)
    }
}
