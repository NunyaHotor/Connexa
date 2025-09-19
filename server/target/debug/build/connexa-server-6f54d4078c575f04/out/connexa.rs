/// -----------------------------
/// Core Envelope
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageEnvelope {
    /// Unique message ID (UUID)
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Encrypted sender identity (sealed sender)
    #[prost(string, tag = "2")]
    pub sender_id: ::prost::alloc::string::String,
    /// Recipient (user, group, or device)
    #[prost(string, tag = "3")]
    pub recipient_id: ::prost::alloc::string::String,
    /// Unix time (ms)
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    /// Encrypted MessagePayload
    #[prost(bytes = "vec", tag = "5")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    /// Message type
    #[prost(enumeration = "MessageType", tag = "6")]
    pub r#type: i32,
}
/// -----------------------------
/// Payload (decrypted content)
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePayload {
    /// Device ID of sender
    #[prost(string, tag = "1")]
    pub sender_device: ::prost::alloc::string::String,
    #[prost(oneof = "message_payload::Content", tags = "2, 3, 4, 5")]
    pub content: ::core::option::Option<message_payload::Content>,
}
/// Nested message and enum types in `MessagePayload`.
pub mod message_payload {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "2")]
        Text(super::TextMessage),
        #[prost(message, tag = "3")]
        Media(super::MediaMessage),
        #[prost(message, tag = "4")]
        Call(super::CallMessage),
        #[prost(message, tag = "5")]
        Control(super::ControlMessage),
    }
}
/// -----------------------------
/// Text
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextMessage {
    #[prost(string, tag = "1")]
    pub body: ::prost::alloc::string::String,
}
/// -----------------------------
/// Media
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaMessage {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub file_size: i64,
    /// Symmetric key for decryption
    #[prost(bytes = "vec", tag = "4")]
    pub media_key: ::prost::alloc::vec::Vec<u8>,
    /// Location (encrypted blob in S3/IPFS/etc.)
    #[prost(string, tag = "5")]
    pub media_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub thumbnail: ::core::option::Option<Thumbnail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thumbnail {
    /// Small preview image (encrypted)
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "2")]
    pub width: i32,
    #[prost(int32, tag = "3")]
    pub height: i32,
}
/// -----------------------------
/// Calls (WebRTC signaling)
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallMessage {
    #[prost(string, tag = "1")]
    pub call_id: ::prost::alloc::string::String,
    #[prost(enumeration = "CallType", tag = "2")]
    pub call_type: i32,
    /// Session description (offer/answer)
    #[prost(string, tag = "3")]
    pub sdp: ::prost::alloc::string::String,
    /// ICE candidates
    #[prost(string, repeated, tag = "4")]
    pub candidates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// -----------------------------
/// Control Messages
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlMessage {
    #[prost(oneof = "control_message::Action", tags = "1, 2")]
    pub action: ::core::option::Option<control_message::Action>,
}
/// Nested message and enum types in `ControlMessage`.
pub mod control_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        Group(super::GroupControl),
        #[prost(message, tag = "2")]
        Device(super::DeviceControl),
    }
}
/// Group management (create, invite, remove, leave)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupControl {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(enumeration = "GroupAction", tag = "2")]
    pub action: i32,
    /// Affected members
    #[prost(string, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Device linking (multi-device support)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceControl {
    #[prost(string, tag = "1")]
    pub device_id: ::prost::alloc::string::String,
    #[prost(enumeration = "DeviceAction", tag = "2")]
    pub action: i32,
}
/// -----------------------------
/// Encrypted Message
/// -----------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub sender_blind: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub dh_ratchet_pub: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub message_number: u32,
    #[prost(int64, tag = "5")]
    pub timestamp: i64,
    #[prost(string, tag = "6")]
    pub target_device_id: ::prost::alloc::string::String,
    /// NEW: time-to-live in seconds (optional)
    #[prost(int64, tag = "7")]
    pub ttl: i64,
}
/// Supported top-level message types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    Text = 0,
    Media = 1,
    Call = 2,
    /// e.g., group/device management
    Control = 3,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::Text => "TEXT",
            MessageType::Media => "MEDIA",
            MessageType::Call => "CALL",
            MessageType::Control => "CONTROL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEXT" => Some(Self::Text),
            "MEDIA" => Some(Self::Media),
            "CALL" => Some(Self::Call),
            "CONTROL" => Some(Self::Control),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CallType {
    Voice = 0,
    Video = 1,
    Group = 2,
}
impl CallType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CallType::Voice => "VOICE",
            CallType::Video => "VIDEO",
            CallType::Group => "GROUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICE" => Some(Self::Voice),
            "VIDEO" => Some(Self::Video),
            "GROUP" => Some(Self::Group),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupAction {
    Create = 0,
    Invite = 1,
    Remove = 2,
    Leave = 3,
    /// e.g., name, avatar
    Update = 4,
}
impl GroupAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupAction::Create => "CREATE",
            GroupAction::Invite => "INVITE",
            GroupAction::Remove => "REMOVE",
            GroupAction::Leave => "LEAVE",
            GroupAction::Update => "UPDATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATE" => Some(Self::Create),
            "INVITE" => Some(Self::Invite),
            "REMOVE" => Some(Self::Remove),
            "LEAVE" => Some(Self::Leave),
            "UPDATE" => Some(Self::Update),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceAction {
    Link = 0,
    Unlink = 1,
    /// Sync keys/messages
    Sync = 2,
}
impl DeviceAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceAction::Link => "LINK",
            DeviceAction::Unlink => "UNLINK",
            DeviceAction::Sync => "SYNC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINK" => Some(Self::Link),
            "UNLINK" => Some(Self::Unlink),
            "SYNC" => Some(Self::Sync),
            _ => None,
        }
    }
}
