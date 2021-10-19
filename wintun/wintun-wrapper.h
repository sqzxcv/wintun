//Manually generated file from wintun documentation to tune what bindgen generates

//Types that we need
typedef unsigned char BOOL;
typedef unsigned char BYTE;
typedef void* LPARAM;
typedef void* HANDLE;
typedef unsigned short WCHAR;
typedef const WCHAR *LPCWSTR, *PCWSTR;
typedef unsigned long long DWORD64, *PDWORD64;
typedef unsigned int DWORD;

typedef struct _GUID {
    union {
        struct {
            unsigned long long a;
            unsigned long long b;
        };
        unsigned char Bytes[16];
    };
} GUID;


#define ULONG64 long long unsigned int

typedef ULONG64 NET_LUID;

#define WINTUN_API __stdcall

//Begin WinTun definitions:

// Maximum pool name length (UTF-16 codepoints) including zero terminator
#define WINTUN_MAX_POOL 256

// Maximum adapter name length (UTF-16 codepoints) including zero terminator
#define MAX_ADAPTER_NAME 128

// Minimum ring capacity.
#define WINTUN_MIN_RING_CAPACITY 0x20000 /* 128kiB */

//Maximum ring capacity.
#define WINTUN_MAX_RING_CAPACITY 0x4000000 /* 64MiB */

//Maximum IP packet size
#define WINTUN_MAX_IP_PACKET_SIZE 0xFFFF

//A handle representing Wintun adapter
typedef struct _WINTUN_ADAPTER *WINTUN_ADAPTER_HANDLE;
/**
 * A handle representing Wintun session
 */
typedef struct _TUN_SESSION *WINTUN_SESSION_HANDLE;


/*
Called by WintunEnumAdapters for each adapter in the pool.

Parameters

Adapter : Adapter handle, which will be freed when this function returns.
Param : An application - defined value passed to the WintunEnumAdapters.

Returns

Non - zero to continue iterating adapters; zero to stop.*/
typedef BOOL(WINTUN_API *WINTUN_ENUM_CALLBACK) (WINTUN_ADAPTER_HANDLE Adapter, LPARAM Param);

//Determines the level of logging, passed to WINTUN_LOGGER_CALLBACK.
enum WINTUN_LOGGER_LEVEL {
    WINTUN_LOG_INFO,
    WINTUN_LOG_WARN,
    WINTUN_LOG_ERR
};

/*
Called by internal logger to report diagnostic messages

Parameters

Level : Message level.
Message : Message text.*/
typedef void(WINTUN_API *WINTUN_LOGGER_CALLBACK) (enum WINTUN_LOGGER_LEVEL Level, DWORD64 Timestamp, const WCHAR* Message);

/**
 * Creates a new Wintun adapter.
 *
 * @param Name          The requested name of the adapter. Zero-terminated string of up to MAX_ADAPTER_NAME-1
 *                      characters.
 *
 * @param TunnelType    Name of the adapter tunnel type. Zero-terminated string of up to MAX_ADAPTER_NAME-1
 *                      characters.
 *
 * @param RequestedGUID The GUID of the created network adapter, which then influences NLA generation deterministically.
 *                      If it is set to NULL, the GUID is chosen by the system at random, and hence a new NLA entry is
 *                      created for each new adapter. It is called "requested" GUID because the API it uses is
 *                      completely undocumented, and so there could be minor interesting complications with its usage.
 *
 * @return If the function succeeds, the return value is the adapter handle. Must be released with
 * WintunCloseAdapter. If the function fails, the return value is NULL. To get extended error information, call
 * GetLastError.
 */
WINTUN_ADAPTER_HANDLE WINTUN_API WintunCreateAdapter(LPCWSTR Name, LPCWSTR TunnelType, const GUID *RequestedGUID);


/*
    Opens an existing Wintun adapter.

    Parameters

    Pool : Name of the adapter pool.Zero - terminated string of up to WINTUN_MAX_POOL - 1 characters.
    Name : Adapter name.Zero - terminated string of up to MAX_ADAPTER_NAME - 1 characters.

    Returns

    If the function succeeds, the return value is adapter handle.Must be released with WintunFreeAdapter.If the function fails, the return value is NULL.To get extended error information, call GetLastError.Possible errors include the following : ERROR_FILE_NOT_FOUND if adapter with given name is not found;
ERROR_ALREADY_EXISTS if adapter is found but not a Wintun - class or
    not a member of the pool WintunDeleteAdapter()
*/
WINTUN_ADAPTER_HANDLE WINTUN_API WintunOpenAdapter(LPCWSTR Name);

void WINTUN_API WintunCloseAdapter(WINTUN_ADAPTER_HANDLE Adapter);

BOOL WINTUN_API WintunDeleteDriver();

void WINTUN_API WintunGetAdapterLUID(WINTUN_ADAPTER_HANDLE Adapter, NET_LUID * Luid);

void WINTUN_API WintunSetLogger(WINTUN_LOGGER_CALLBACK NewLogger);

/*
    Determines the version of the Wintun driver currently loaded.

    Returns

    If the function succeeds, the return value is the version number.If the function fails, the return value is zero.To get extended error information, call GetLastError.Possible errors include the following : ERROR_FILE_NOT_FOUND Wintun not loaded
*/
DWORD WINTUN_API WintunGetRunningDriverVersion(void);

/*
    Starts Wintun session.

    Parameters

    Adapter : Adapter handle obtained with WintunOpenAdapter or WintunCreateAdapter
    Capacity : Rings capacity.Must be between WINTUN_MIN_RING_CAPACITY and WINTUN_MAX_RING_CAPACITY(incl.) Must be a power of two.

    Returns

    Wintun session handle.Must be released with WintunEndSession.If the function fails, the return value is NULL.To get extended error information, call GetLastError.
*/
WINTUN_SESSION_HANDLE WINTUN_API WintunStartSession(WINTUN_ADAPTER_HANDLE Adapter, DWORD Capacity);


/*
    Ends Wintun session.

    Parameters

    Session : Wintun session handle obtained with WintunStartSession
*/
void WINTUN_API WintunEndSession(WINTUN_SESSION_HANDLE Session);


/*
    Gets Wintun session's read-wait event handle.

    Parameters

    Session : Wintun session handle obtained with WintunStartSession

    Returns

    Pointer to receive event handle to wait for available data when reading.Should WintunReceivePackets return ERROR_NO_MORE_ITEMS(after spinning on it for a while under heavy load), wait for this event to become signaled before retrying WintunReceivePackets.Do not call CloseHandle on this event - it is managed by the session.
*/
HANDLE WINTUN_API WintunGetReadWaitEvent(WINTUN_SESSION_HANDLE Session);

/*
    Retrieves one or packet.After the packet content is consumed, call WintunReleaseReceivePacket with Packet returned from this function to release internal buffer.This function is thread - safe.

    Parameters

    Session : Wintun session handle obtained with WintunStartSession
    PacketSize : Pointer to receive packet size.

    Returns

    Pointer to layer 3 IPv4 or IPv6 packet.Client may modify its content at will.If the function fails, the return value is NULL.To get extended error information, call GetLastError.Possible errors include the following : ERROR_HANDLE_EOF Wintun adapter is terminating;
ERROR_NO_MORE_ITEMS Wintun buffer is exhausted; ERROR_INVALID_DATA Wintun buffer is corrupt
*/
BYTE * WINTUN_API WintunReceivePacket(WINTUN_SESSION_HANDLE Session, DWORD * PacketSize);
/*
    Releases internal buffer after the received packet has been processed by the client.This function is thread - safe.

    Parameters

    Session : Wintun session handle obtained with WintunStartSession
    Packet : Packet obtained with WintunReceivePacket
*/
void WINTUN_API WintunReleaseReceivePacket(WINTUN_SESSION_HANDLE Session, const BYTE * Packet);


/*Allocates memory for a packet to send.After the memory is filled with packet data, call WintunSendPacket to sendand release internal buffer.WintunAllocateSendPacket is thread - safe and the WintunAllocateSendPacket order of calls define the packet sending order.

Parameters

Session : Wintun session handle obtained with WintunStartSession
PacketSize : Exact packet size.Must be less or equal to WINTUN_MAX_IP_PACKET_SIZE.

Returns

Returns pointer to memory where to prepare layer 3 IPv4 or IPv6 packet for sending.If the function fails, the return value is NULL.To get extended error information, call GetLastError.Possible errors include the following : ERROR_HANDLE_EOF Wintun adapter is terminating;
ERROR_BUFFER_OVERFLOW Wintun buffer is full;*/
BYTE * WINTUN_API WintunAllocateSendPacket(WINTUN_SESSION_HANDLE Session, DWORD PacketSize);

/*
            Sends the packetand releases internal buffer
                .WintunSendPacket is thread
    - safe,
    but the
    WintunAllocateSendPacket order of calls define the packet sending order
        .This means the packet is
    not guaranteed to be sent in the WintunSendPacket yet.

    Parameters

    Session : Wintun session handle obtained with WintunStartSession Packet
    : Packet obtained with WintunAllocateSendPacket*/
void WINTUN_API WintunSendPacket(WINTUN_SESSION_HANDLE Session, const BYTE *Packet);

