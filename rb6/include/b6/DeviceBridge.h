#pragma once
#include <memory>
#include <array>
#include <cstdint>
#include "Device.hh"

class DeviceHandle {
public:
  static std::unique_ptr<DeviceHandle> new_device();

  explicit DeviceHandle(std::unique_ptr<b6::Device> d);

private:
  std::unique_ptr<b6::Device> device;
};

inline std::unique_ptr<DeviceHandle> new_device() {
  return DeviceHandle::new_device();
}