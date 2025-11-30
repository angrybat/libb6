use libc::*;
use crate::MOCK;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_init(_ctx: *mut *mut c_void) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_init");
    }
    MOCK.lock().unwrap().init_return
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_open_device_with_vid_pid(
    _ctx: *mut c_void,
    _vid: u16,
    _pid: u16,
) -> *mut c_void {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_open_device_with_vid_pid");
    }
    MOCK.lock().unwrap().open_return as *mut libc::c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_kernel_driver_active(
    _dev: *mut c_void,
    _interface: c_int,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_kernel_driver_active");
    }
    MOCK.lock().unwrap().kernel_driver_active
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_detach_kernel_driver(
    _dev: *mut c_void,
    _interface: c_int,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_detach_kernel_driver");
    }
    MOCK.lock().unwrap().detach_return
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_claim_interface(
    _dev: *mut c_void,
    _interface: c_int,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_claim_interface");
    }
    MOCK.lock().unwrap().claim_return
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_release_interface(
    _dev: *mut c_void,
    _interface: c_int,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_release_interface");
    }
    MOCK.lock().unwrap().release_return
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_attach_kernel_driver(
    _dev: *mut c_void,
    _interface: c_int,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_attach_kernel_driver");
    }
    MOCK.lock().unwrap().attach_return
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_close(_dev: *mut c_void) {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_close");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_exit(_ctx: *mut c_void) {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_exit");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libusb_interrupt_transfer(
    _dev: *mut c_void,
    _endpoint: u8,
    data: *mut u8,
    _len: c_int,
    transferred: *mut c_int,
    _timeout: c_uint,
) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_interrupt_transfer");
    }

    let mock = MOCK.lock().unwrap();

    // Write mock data if buffer is valid
    if !data.is_null() && mock.interrupt_return > 0 {
        unsafe {
            *data = 0xAB;
        }
    }

    if !transferred.is_null() {
        unsafe {
            *transferred = mock.interrupt_return;
        }
    }

    mock.interrupt_return
}
