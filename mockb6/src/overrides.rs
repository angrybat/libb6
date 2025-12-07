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
    MOCK.lock().unwrap().open_return as *mut c_void
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
pub unsafe extern "C" fn libusb_close(_dev: *mut c_void) -> c_int {
    if MOCK.lock().unwrap().print_debug {
        eprintln!("[mock] libusb_close");
    }
    return 0;
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
    let mut mock = MOCK.lock().unwrap();

    if mock.print_debug {
        eprintln!("[mock] libusb_interrupt_transfer: endpoint: {:02X}, len: {}, transferred: {:p}, timeout: {}", _endpoint, _len, transferred, _timeout);
    }

    let is_read = (_endpoint & 0x80) != 0;

    if is_read {
        let bytes = mock.get_next_read(_len as usize);
        if mock.print_debug {
            eprintln!("[mock] read data: {:02X?}", bytes);
        }
        let copy_len = std::cmp::min(_len as usize, bytes.len());
        if mock.print_debug {
            eprintln!("[mock] copying {} bytes to data ptr {:p}", copy_len, data);
        }
        if !data.is_null() && copy_len > 0 {
            if mock.print_debug {
                eprintln!("[mock] performing copy_nonoverlapping");
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, copy_len);
            }
            if mock.print_debug {
                eprintln!("[mock] copy_nonoverlapping complete");
            }
        }
        if !transferred.is_null() {
            if mock.print_debug {
                eprintln!("[mock] setting transferred to {}", copy_len);
            }
            unsafe { *transferred = copy_len as c_int; }
            if mock.print_debug {
                eprintln!("[mock] set transferred to {}", copy_len);
            }
        }
    } else {
        if mock.print_debug {
            eprintln!("[mock] writing data: {:02X?}", data);
        }
        if !data.is_null() {
            let slice: &[u8] = unsafe { std::slice::from_raw_parts(data, _len as usize) };
            mock.record_write(slice);
        } else {
            eprintln!("[mock] WARNING: data pointer is null, cannot record write");
        }
    }

    mock.interrupt_return
}
