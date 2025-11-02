#pragma once
#include <memory>
#include <array>
#include <cstdint>
#include "Device.hh" // must define Device

struct SysInfo {
  uint8_t cycle_time;
  uint8_t time_limit_on;
  uint16_t time_limit;
  uint8_t cap_limit_on;
  uint16_t cap_limit;
  uint8_t key_buzzer;
  uint8_t system_buzzer;
  uint16_t low_dc_limit;
  uint8_t temp_limit;
  uint16_t voltage;
  std::array<uint16_t, 8> cells;
};

class DeviceHandle {
public:
  // factory: returns nullptr on error
  static std::unique_ptr<DeviceHandle> new_device();

  // get system info
  std::unique_ptr<SysInfo> get_sys_info() const;

  explicit DeviceHandle(std::unique_ptr<b6::Device> d);

private:
  std::unique_ptr<b6::Device> dev;
};

// convenience
inline std::unique_ptr<DeviceHandle> new_device() {
  return DeviceHandle::new_device();
}
