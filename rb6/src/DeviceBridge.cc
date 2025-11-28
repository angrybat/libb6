#include "b6/DeviceBridge.h"
#include <exception>
#include <memory>
#include <iostream>

DeviceHandle::DeviceHandle(std::unique_ptr<b6::Device> d) : device(std::move(d)) {}

std::unique_ptr<DeviceHandle> DeviceHandle::new_device() {
  try {
    auto d = std::make_unique<b6::Device>();
    return std::make_unique<DeviceHandle>(std::move(d));
  } catch (const std::exception& e) {
    std::cerr << "Could not create device: " << e.what() << std::endl;
    return nullptr;
  } catch (...) {
    std::cerr << "Could not create device due to unknown exception" << std::endl;
    return nullptr;
  }
}