mod new_device {
    use rb6::new_device;
    use rb6::get_dev_info_command_buffer;
    use mockb6::setup_mock;
    use rstest::rstest;

    #[test]
    fn returns_error_when_cant_initalise_libusb() {
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
    fn returns_error_when_libusb_cant_open_with_vid_pid() {
        setup_mock(|mock| {
            mock.init_return = 0;
            mock.open_return = 0;
        });
    
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
        setup_mock(|mock| {
            mock.init_return = 0;
            mock.open_return = 0x1;
            mock.detach_return = -5;
            mock.kernel_driver_active = 1;
        });
    
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
        setup_mock(|mock| {
            mock.init_return = 0;
            mock.open_return = 0x1;
            mock.kernel_driver_active = 0;
            mock.claim_return = -3;
        });
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error() == "Could not create device: cannot claim interface 0, err: -3");
            assert!(result.get_device_handle().is_null());
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }

    #[rstest]
    #[case("ABCDEF", 6)]
    #[case("100069", 8)]
    fn returns_device_handle_on_successful_initialization(#[case] core_type: &str, #[case] cell_count: i32) {
        let core_type_array: [u8; 6] = core_type.as_bytes().try_into().unwrap();
        let msg = mockb6::DevInfoMessage {
            core_type: core_type_array,
            upgrade_type: 2,
            is_encrypted: 1,
            customer_id: 513,
            language_id: 5,
            sw_major: 100,
            sw_minor: 42,
            hw_version: 7,
        };
        setup_mock(|mock| {
            mock.init_return = 0;
            mock.open_return = 1;
            mock.kernel_driver_active = 0;
            mock.claim_return = 0;
            mock.reads = vec![Box::new(msg)];
            mock.interrupt_return = 0;
        });
    
        let mut device_handle_result = new_device();
    
        if let Some(result) = device_handle_result.as_mut() {
            assert!(result.get_error().is_empty());
            let device_handle = result.get_device_handle();
            assert!(!device_handle.is_null());
            assert!(device_handle.core_type() == core_type);
            assert!(device_handle.upgrade_type() == 2);
            assert!(device_handle.language_id() == 5);
            assert!(device_handle.customer_id() == 513);
            assert!(device_handle.hw_version() == 7.0);
            assert!(device_handle.sw_version() == (100.0 + 42.0) / 100.0);
            assert!(device_handle.is_encrypted() == true);
            assert!(device_handle.cell_count() == cell_count);
            let writes    = {
                let mock = mockb6::MOCK.lock().unwrap();
                let writes = mock.writes.clone();
                writes
            };
            assert!(writes.len() >= 1, "Expected at least one write, got {}", writes.len());
            let first_write = &writes[0];
            let expected_write = &get_dev_info_command_buffer();
            assert!(first_write == expected_write, "Expected first write to be {:?}, got {:?}", expected_write, first_write);
        } 
        else {
            panic!("Expected DeviceHandleResult, got None");
        }
    }
}