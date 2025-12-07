#include "b6/DeviceBridge.h"
#include <exception>
#include <memory>
#include <iostream>

DeviceHandle::DeviceHandle(std::unique_ptr<b6::Device> d) : device(std::move(d)) {}

std::unique_ptr<DeviceHandleResult> DeviceHandle::new_device() {
  auto device_handle_result = std::make_unique<DeviceHandleResult>();
  try {
    auto device = std::make_unique<b6::Device>();
    device_handle_result -> device_handle = std::make_unique<DeviceHandle>(std::move(device));
  } catch (const std::exception& e) {
    device_handle_result -> device_handle = nullptr;
    device_handle_result -> error = std::string("Could not create device: ") + e.what();
  } catch (...) {
    device_handle_result -> device_handle = nullptr;
    device_handle_result -> error = "Could not create device due to unknown exception";
  }
  return device_handle_result;
}