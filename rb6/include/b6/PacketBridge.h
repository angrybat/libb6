
# include "Packet.hh"

inline int32_t get_dev_info_command() {
  return static_cast<int32_t>(b6::CMD::GET_DEV_INFO);
}

inline rust::Vec<uint8_t> get_dev_info_command_buffer() {
  b6::Packet cmdPacket({ 0x0f, 0x03, static_cast<uint8_t>(b6::CMD::GET_DEV_INFO), 0x00 });
  cmdPacket.writeChecksum();
  
  const uint8_t* buf = cmdPacket.getBuffer();
  size_t len = cmdPacket.getSize();

  rust::Vec<uint8_t> v;
  v.reserve(len);
  for (size_t i = 0; i < len; ++i) {
      v.push_back(buf[i]);
  }
  return v;
} 