#pragma once
#include <memory>
#include <array>
#include <cstdint>
#include "Device.hh"
#include "rust/cxx.h" 

struct DeviceHandle;

struct DeviceHandleResult {
  std::unique_ptr<DeviceHandle> device_handle;
  std::string error;  

  std::unique_ptr<DeviceHandle> get_device_handle() {
    return std::move(device_handle);
  }

  rust::cxxbridge1::String get_error() const {
    return rust::cxxbridge1::String(error.c_str());
  }
};

class DeviceHandle {
public:
  static std::unique_ptr<DeviceHandleResult> new_device();

  explicit DeviceHandle(std::unique_ptr<b6::Device> d);

private:
  std::unique_ptr<b6::Device> device;
};

inline std::unique_ptr<DeviceHandleResult> new_device() {
  return DeviceHandle::new_device();
}

