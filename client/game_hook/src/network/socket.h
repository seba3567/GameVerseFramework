// Network Socket - QUIC/WebSocket transport
// socket.h

#pragma once

#include <winsock2.h>
#include <windows.h>
#include <string>
#include <cstdint>
#include <vector>
#include <functional>
#include <memory>
#include <thread>
#include <atomic>

// Forward declarations
struct quic_conn;
struct ws_conn;

namespace Network {

// Connection status
enum class Status {
    Disconnected,
    Connecting,
    Connected,
    Error
};

// Packet types
enum PacketType : uint8_t {
    Hello = 0,
    Accepted = 1,
    Rejected = 2,
    Event = 3,
    RPC = 4,
    ResourceList = 5,
    ResourceData = 6,
    Ping = 7,
    Pong = 8
};

// Network packet
struct Packet {
    PacketType type;
    std::vector<uint8_t> data;
};

// Base socket interface
class Socket {
public:
    Socket();
    ~Socket();
    
    // Connect to server
    bool Connect(const std::string& host, int port);
    
    // Disconnect
    void Disconnect();
    
    // Send raw data
    bool Send(const uint8_t* data, size_t len);
    
    // Send packet
    bool SendPacket(PacketType type, const void* data, size_t len);
    
    // Receive data
    void Receive();
    
    // Send event to server
    void SendEvent(const char* event, const char* data);
    
    // Get status
    Status GetStatus() const { return status_.load(); }
    
    // Set callbacks
    void OnPacket(std::function<void(Packet&)> callback);
    void OnEvent(std::function<void(const std::string&, const std::string&)> callback);

private:
    // Try QUIC first, fallback to WebSocket
    bool connect_quic(const std::string& host, int port);
    bool connect_websocket(const std::string& host, int port);
    
    Status status_ = Status::Disconnected;
    std::string host_;
    int port_ = 0;
    
    // Connections
    quic_conn* quic_ = nullptr;
    ws_conn* ws_ = nullptr;
    
    // Callbacks
    std::function<void(Packet&)> packet_cb_;
    std::function<void(const std::string&, const std::string&)> event_cb_;
    
    // Receive thread
    std::thread receive_thread_;
    std::atomic<bool> running_{false};
    
    // Authentication
    std::string auth_token_;
};

// QUIC connection
struct quic_conn {
    void* connection; // quinn connection handle
    void* endpoint;  // quinn endpoint
    bool connected = false;
};

// WebSocket connection
struct ws_conn {
    void* socket;     // websocket handle
    bool connected = false;
};

} // namespace Network
