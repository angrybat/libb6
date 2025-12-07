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
  rust::String core_type() const {
    return rust::String(device->getCoreType());
  }
  int upgrade_type() const {
    return device->getUpgradeType();
  }
  int language_id() const {
    return device->getLanguageID();
  }
  int customer_id() const {
    return device->getCustomerID();
  }
  double hw_version() const {
    return device->getHWVersion();
  }
  double sw_version() const {
    return device->getSWVersion();
  }
  bool is_encrypted() const {
    return device->isEncrypted();
  }
  int cell_count() const {
    return device->getCellCount();
  }

  explicit DeviceHandle(std::unique_ptr<b6::Device> d);

private:
  std::unique_ptr<b6::Device> device;
};

inline std::unique_ptr<DeviceHandleResult> new_device() {
  return DeviceHandle::new_device();
}