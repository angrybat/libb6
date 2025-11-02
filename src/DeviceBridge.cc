#include "b6/DeviceBridge.h"
#include <exception>
#include <memory>

DeviceHandle::DeviceHandle(std::unique_ptr<b6::Device> d) : dev(std::move(d)) {}

std::unique_ptr<DeviceHandle> DeviceHandle::new_device() {
  try {
    auto d = std::make_unique<b6::Device>();
    return std::make_unique<DeviceHandle>(std::move(d));
  } catch (const std::exception&) {
    return nullptr;
  } catch (...) {
    return nullptr;
  }
}

std::unique_ptr<SysInfo> DeviceHandle::get_sys_info() const {
  SysInfo out{};
  auto s = dev->getSysInfo();

  out.cycle_time = static_cast<uint8_t>(s.cycleTime);
  out.time_limit_on = static_cast<uint8_t>(s.timeLimitOn);
  out.time_limit = static_cast<uint16_t>(s.timeLimit);
  out.cap_limit_on = static_cast<uint8_t>(s.capLimitOn);
  out.cap_limit = static_cast<uint16_t>(s.capLimit);
  out.key_buzzer = static_cast<uint8_t>(s.keyBuzzer);
  out.system_buzzer = static_cast<uint8_t>(s.systemBuzzer);
  out.low_dc_limit = static_cast<uint16_t>(s.lowDCLimit);
  out.temp_limit = static_cast<uint8_t>(s.tempLimit);
  out.voltage = static_cast<uint16_t>(s.voltage);

  for (size_t i = 0; i < out.cells.size(); ++i) {
    if (i < 8) // raw array has 8 elements
        out.cells[i] = static_cast<uint16_t>(s.cells[i]);
    else
        out.cells[i] = 0;
  }


  return std::make_unique<SysInfo>(out);
}
