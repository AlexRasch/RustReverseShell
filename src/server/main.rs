#![windows_subsystem = "windows"]
#![no_main]
#![no_std]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[link(name = "ws2_32")]
unsafe extern "system" {
    fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut c_void) -> i32;
    fn WSACleanup() -> i32;
    fn socket(af: i32, type_: i32, protocol: i32) -> usize; // SOCKET är en usize på x64
    fn connect(s: usize, addr: *const SOCKADDR_IN, namelen: i32) -> i32;
    fn send(s: usize, buf: *const u8, len: i32, flags: i32) -> i32;
    fn recv(s: usize, buf: *mut u8, len: i32, flags: i32) -> i32;
    fn closesocket(s: usize) -> i32;
    fn WSAGetLastError() -> i32;
}

// Strukturer för Windows API
#[repr(C)]
struct SOCKADDR_IN {
    sin_family: i16,
    sin_port: u16,
    sin_addr: u32, // 127.0.0.1 som u32
    sin_zero: [u8; 8],
}

#[unsafe(no_mangle)]
pub extern "system" fn WinMain(
    _hInstance: *mut c_void,
    _hPrevInstance: *mut c_void,
    _lpCmdLine: *const u8,
    _nCmdShow: i32,
) -> i32 {
    unsafe {

        // Initiera Winsock
        let mut wsa_data = core::mem::zeroed();
        if WSAStartup(0x0202, &mut wsa_data) != 0 {
            return 10;
        }

        // Skapa socket
        let sock = socket(2, 1, 6); // AF_INET, SOCK_STREAM, IPPROTO_TCP
        if sock == usize::MAX {
            WSACleanup();
            return 20;
        }

        // Anslut till 127.0.0.1:4444
        let mut addr = SOCKADDR_IN {
            sin_family: 2, // AF_INET
            sin_port: (4444u16.to_be()), // 4444 i network byte order (big-endian)
            sin_addr: u32::from_ne_bytes([127, 0, 0, 1]), // 127.0.0.1 i little-endian (Winsock-kompatibelt)
            sin_zero: [0; 8],
        };
        if connect(sock, &addr, core::mem::size_of::<SOCKADDR_IN>() as i32) != 0 {
            let error_code = WSAGetLastError();
            closesocket(sock);
            WSACleanup();
            return error_code;
        }

        let mut buffer = [0u8; 1024];
        loop {
            // Ta emot kommando
            let bytes_read = recv(sock, buffer.as_mut_ptr(), 1024, 0);
            if bytes_read <= 0 {
                break; // Klienten stängde eller fel
            }

            // Nöjer oss med att skicka tillbaka
            let sent_bytes = send(sock, buffer.as_ptr(), bytes_read, 0);
            if sent_bytes == -1 {
                break;
            }


        }

        closesocket(sock);
        WSACleanup();
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}