mod new_device {
    use rb6::new_device;
    use mockb6;

    #[test]
    fn returns_error_when_libusb_init_does_not_return_0() {
        {
            let mut mock = mockb6::MOCK.lock().unwrap();
            mock.init_return = -1;
        }
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error() == "Could not create device: libusb err: -1");
            assert!(result.get_device_handle().is_null());
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }
    
    #[test]
    fn returns_error_when_libusb_open_with_vid_pid_returns_nullptr() {
        {
            let mut mock = mockb6::MOCK.lock().unwrap();
            mock.init_return = 0;
            mock.open_return = 0;
        }
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error() == "Could not create device: cannot find/open b6 device");
            assert!(result.get_device_handle().is_null());
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }

    #[test]
    fn returns_error_when_libusb_cant_detach_kernel_driver() {
        {
            let mut mock = mockb6::MOCK.lock().unwrap();
            mock.init_return = 0;
            mock.open_return = 0x1;
            mock.detach_return = -5;
            mock.kernel_driver_active = 1;
        }
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error() == "Could not create device: cannot detach kernel driver, err: -5");
            assert!(result.get_device_handle().is_null());
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }

    #[test]
    fn returns_error_when_libusb_cant_claim_interface() {
        {
            let mut mock = mockb6::MOCK.lock().unwrap();
            mock.init_return = 0;
            mock.open_return = 0x1;
            mock.kernel_driver_active = 0;
            mock.claim_return = -3;
        }
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error() == "Could not create device: cannot claim interface 0, err: -3");
            assert!(result.get_device_handle().is_null());
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }
}