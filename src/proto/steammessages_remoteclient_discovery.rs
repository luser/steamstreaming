// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteClientBroadcastHeader {
    // message fields
    client_id: ::std::option::Option<u64>,
    msg_type: ::std::option::Option<ERemoteClientBroadcastMsg>,
    instance_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteClientBroadcastHeader {}

impl CMsgRemoteClientBroadcastHeader {
    pub fn new() -> CMsgRemoteClientBroadcastHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastHeader,
        };
        unsafe {
            instance.get(CMsgRemoteClientBroadcastHeader::new)
        }
    }

    // optional uint64 client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id = ::std::option::Option::None;
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: u64) {
        self.client_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_id(&self) -> u64 {
        self.client_id.unwrap_or(0)
    }

    fn get_client_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_id
    }

    // optional .ERemoteClientBroadcastMsg msg_type = 2;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: ERemoteClientBroadcastMsg) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> ERemoteClientBroadcastMsg {
        self.msg_type.unwrap_or(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<ERemoteClientBroadcastMsg> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<ERemoteClientBroadcastMsg> {
        &mut self.msg_type
    }

    // optional uint64 instance_id = 3;

    pub fn clear_instance_id(&mut self) {
        self.instance_id = ::std::option::Option::None;
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: u64) {
        self.instance_id = ::std::option::Option::Some(v);
    }

    pub fn get_instance_id(&self) -> u64 {
        self.instance_id.unwrap_or(0)
    }

    fn get_instance_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.instance_id
    }

    fn mut_instance_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.instance_id
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.client_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.instance_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.msg_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.instance_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.msg_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.instance_id {
            os.write_uint64(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastHeader {
    fn new() -> CMsgRemoteClientBroadcastHeader {
        CMsgRemoteClientBroadcastHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "client_id",
                    CMsgRemoteClientBroadcastHeader::get_client_id_for_reflect,
                    CMsgRemoteClientBroadcastHeader::mut_client_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ERemoteClientBroadcastMsg>>(
                    "msg_type",
                    CMsgRemoteClientBroadcastHeader::get_msg_type_for_reflect,
                    CMsgRemoteClientBroadcastHeader::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "instance_id",
                    CMsgRemoteClientBroadcastHeader::get_instance_id_for_reflect,
                    CMsgRemoteClientBroadcastHeader::mut_instance_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastHeader>(
                    "CMsgRemoteClientBroadcastHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastHeader {
    fn clear(&mut self) {
        self.clear_client_id();
        self.clear_msg_type();
        self.clear_instance_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteClientBroadcastHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteClientBroadcastHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteClientBroadcastStatus {
    // message fields
    version: ::std::option::Option<i32>,
    min_version: ::std::option::Option<i32>,
    connect_port: ::std::option::Option<u32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    enabled_services: ::std::option::Option<u32>,
    ostype: ::std::option::Option<i32>,
    is64bit: ::std::option::Option<bool>,
    users: ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User>,
    euniverse: ::std::option::Option<i32>,
    timestamp: ::std::option::Option<u32>,
    screen_locked: ::std::option::Option<bool>,
    games_running: ::std::option::Option<bool>,
    mac_addresses: ::protobuf::RepeatedField<::std::string::String>,
    download_lan_peer_group: ::std::option::Option<u32>,
    broadcasting_active: ::std::option::Option<bool>,
    vr_active: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteClientBroadcastStatus {}

impl CMsgRemoteClientBroadcastStatus {
    pub fn new() -> CMsgRemoteClientBroadcastStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastStatus,
        };
        unsafe {
            instance.get(CMsgRemoteClientBroadcastStatus::new)
        }
    }

    // optional int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }

    // optional int32 min_version = 2;

    pub fn clear_min_version(&mut self) {
        self.min_version = ::std::option::Option::None;
    }

    pub fn has_min_version(&self) -> bool {
        self.min_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_version(&mut self, v: i32) {
        self.min_version = ::std::option::Option::Some(v);
    }

    pub fn get_min_version(&self) -> i32 {
        self.min_version.unwrap_or(0)
    }

    fn get_min_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.min_version
    }

    fn mut_min_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.min_version
    }

    // optional uint32 connect_port = 3;

    pub fn clear_connect_port(&mut self) {
        self.connect_port = ::std::option::Option::None;
    }

    pub fn has_connect_port(&self) -> bool {
        self.connect_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect_port(&mut self, v: u32) {
        self.connect_port = ::std::option::Option::Some(v);
    }

    pub fn get_connect_port(&self) -> u32 {
        self.connect_port.unwrap_or(0)
    }

    fn get_connect_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connect_port
    }

    fn mut_connect_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connect_port
    }

    // optional string hostname = 4;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // optional uint32 enabled_services = 6;

    pub fn clear_enabled_services(&mut self) {
        self.enabled_services = ::std::option::Option::None;
    }

    pub fn has_enabled_services(&self) -> bool {
        self.enabled_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled_services(&mut self, v: u32) {
        self.enabled_services = ::std::option::Option::Some(v);
    }

    pub fn get_enabled_services(&self) -> u32 {
        self.enabled_services.unwrap_or(0)
    }

    fn get_enabled_services_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.enabled_services
    }

    fn mut_enabled_services_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.enabled_services
    }

    // optional int32 ostype = 7;

    pub fn clear_ostype(&mut self) {
        self.ostype = ::std::option::Option::None;
    }

    pub fn has_ostype(&self) -> bool {
        self.ostype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ostype(&mut self, v: i32) {
        self.ostype = ::std::option::Option::Some(v);
    }

    pub fn get_ostype(&self) -> i32 {
        self.ostype.unwrap_or(0i32)
    }

    fn get_ostype_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ostype
    }

    fn mut_ostype_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ostype
    }

    // optional bool is64bit = 8;

    pub fn clear_is64bit(&mut self) {
        self.is64bit = ::std::option::Option::None;
    }

    pub fn has_is64bit(&self) -> bool {
        self.is64bit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is64bit(&mut self, v: bool) {
        self.is64bit = ::std::option::Option::Some(v);
    }

    pub fn get_is64bit(&self) -> bool {
        self.is64bit.unwrap_or(false)
    }

    fn get_is64bit_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is64bit
    }

    fn mut_is64bit_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is64bit
    }

    // repeated .CMsgRemoteClientBroadcastStatus.User users = 9;

    pub fn clear_users(&mut self) {
        self.users.clear();
    }

    // Param is passed by value, moved
    pub fn set_users(&mut self, v: ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User>) {
        self.users = v;
    }

    // Mutable pointer to the field.
    pub fn mut_users(&mut self) -> &mut ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        &mut self.users
    }

    // Take field
    pub fn take_users(&mut self) -> ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        ::std::mem::replace(&mut self.users, ::protobuf::RepeatedField::new())
    }

    pub fn get_users(&self) -> &[CMsgRemoteClientBroadcastStatus_User] {
        &self.users
    }

    fn get_users_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        &self.users
    }

    fn mut_users_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgRemoteClientBroadcastStatus_User> {
        &mut self.users
    }

    // optional int32 euniverse = 11;

    pub fn clear_euniverse(&mut self) {
        self.euniverse = ::std::option::Option::None;
    }

    pub fn has_euniverse(&self) -> bool {
        self.euniverse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_euniverse(&mut self, v: i32) {
        self.euniverse = ::std::option::Option::Some(v);
    }

    pub fn get_euniverse(&self) -> i32 {
        self.euniverse.unwrap_or(0)
    }

    fn get_euniverse_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.euniverse
    }

    fn mut_euniverse_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.euniverse
    }

    // optional uint32 timestamp = 12;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timestamp
    }

    // optional bool screen_locked = 13;

    pub fn clear_screen_locked(&mut self) {
        self.screen_locked = ::std::option::Option::None;
    }

    pub fn has_screen_locked(&self) -> bool {
        self.screen_locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_screen_locked(&mut self, v: bool) {
        self.screen_locked = ::std::option::Option::Some(v);
    }

    pub fn get_screen_locked(&self) -> bool {
        self.screen_locked.unwrap_or(false)
    }

    fn get_screen_locked_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.screen_locked
    }

    fn mut_screen_locked_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.screen_locked
    }

    // optional bool games_running = 14;

    pub fn clear_games_running(&mut self) {
        self.games_running = ::std::option::Option::None;
    }

    pub fn has_games_running(&self) -> bool {
        self.games_running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_games_running(&mut self, v: bool) {
        self.games_running = ::std::option::Option::Some(v);
    }

    pub fn get_games_running(&self) -> bool {
        self.games_running.unwrap_or(false)
    }

    fn get_games_running_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.games_running
    }

    fn mut_games_running_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.games_running
    }

    // repeated string mac_addresses = 15;

    pub fn clear_mac_addresses(&mut self) {
        self.mac_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_mac_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.mac_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mac_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mac_addresses
    }

    // Take field
    pub fn take_mac_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.mac_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_mac_addresses(&self) -> &[::std::string::String] {
        &self.mac_addresses
    }

    fn get_mac_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.mac_addresses
    }

    fn mut_mac_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mac_addresses
    }

    // optional uint32 download_lan_peer_group = 16;

    pub fn clear_download_lan_peer_group(&mut self) {
        self.download_lan_peer_group = ::std::option::Option::None;
    }

    pub fn has_download_lan_peer_group(&self) -> bool {
        self.download_lan_peer_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download_lan_peer_group(&mut self, v: u32) {
        self.download_lan_peer_group = ::std::option::Option::Some(v);
    }

    pub fn get_download_lan_peer_group(&self) -> u32 {
        self.download_lan_peer_group.unwrap_or(0)
    }

    fn get_download_lan_peer_group_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.download_lan_peer_group
    }

    fn mut_download_lan_peer_group_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.download_lan_peer_group
    }

    // optional bool broadcasting_active = 17;

    pub fn clear_broadcasting_active(&mut self) {
        self.broadcasting_active = ::std::option::Option::None;
    }

    pub fn has_broadcasting_active(&self) -> bool {
        self.broadcasting_active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcasting_active(&mut self, v: bool) {
        self.broadcasting_active = ::std::option::Option::Some(v);
    }

    pub fn get_broadcasting_active(&self) -> bool {
        self.broadcasting_active.unwrap_or(false)
    }

    fn get_broadcasting_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.broadcasting_active
    }

    fn mut_broadcasting_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.broadcasting_active
    }

    // optional bool vr_active = 18;

    pub fn clear_vr_active(&mut self) {
        self.vr_active = ::std::option::Option::None;
    }

    pub fn has_vr_active(&self) -> bool {
        self.vr_active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vr_active(&mut self, v: bool) {
        self.vr_active = ::std::option::Option::Some(v);
    }

    pub fn get_vr_active(&self) -> bool {
        self.vr_active.unwrap_or(false)
    }

    fn get_vr_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.vr_active
    }

    fn mut_vr_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.vr_active
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.min_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.connect_port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.enabled_services = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ostype = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is64bit = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.users)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.euniverse = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.screen_locked = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.games_running = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.mac_addresses)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.download_lan_peer_group = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.broadcasting_active = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.vr_active = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.min_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.connect_port {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.enabled_services {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ostype {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is64bit {
            my_size += 2;
        };
        for value in &self.users {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.euniverse {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.screen_locked {
            my_size += 2;
        };
        if let Some(v) = self.games_running {
            my_size += 2;
        };
        for value in &self.mac_addresses {
            my_size += ::protobuf::rt::string_size(15, &value);
        };
        if let Some(v) = self.download_lan_peer_group {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.broadcasting_active {
            my_size += 3;
        };
        if let Some(v) = self.vr_active {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.min_version {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.connect_port {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.hostname.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.enabled_services {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.ostype {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.is64bit {
            os.write_bool(8, v)?;
        };
        for v in &self.users {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.euniverse {
            os.write_int32(11, v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.screen_locked {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.games_running {
            os.write_bool(14, v)?;
        };
        for v in &self.mac_addresses {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.download_lan_peer_group {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.broadcasting_active {
            os.write_bool(17, v)?;
        };
        if let Some(v) = self.vr_active {
            os.write_bool(18, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastStatus {
    fn new() -> CMsgRemoteClientBroadcastStatus {
        CMsgRemoteClientBroadcastStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CMsgRemoteClientBroadcastStatus::get_version_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "min_version",
                    CMsgRemoteClientBroadcastStatus::get_min_version_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_min_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "connect_port",
                    CMsgRemoteClientBroadcastStatus::get_connect_port_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_connect_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    CMsgRemoteClientBroadcastStatus::get_hostname_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "enabled_services",
                    CMsgRemoteClientBroadcastStatus::get_enabled_services_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_enabled_services_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ostype",
                    CMsgRemoteClientBroadcastStatus::get_ostype_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_ostype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is64bit",
                    CMsgRemoteClientBroadcastStatus::get_is64bit_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_is64bit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgRemoteClientBroadcastStatus_User>>(
                    "users",
                    CMsgRemoteClientBroadcastStatus::get_users_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_users_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "euniverse",
                    CMsgRemoteClientBroadcastStatus::get_euniverse_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_euniverse_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CMsgRemoteClientBroadcastStatus::get_timestamp_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "screen_locked",
                    CMsgRemoteClientBroadcastStatus::get_screen_locked_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_screen_locked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "games_running",
                    CMsgRemoteClientBroadcastStatus::get_games_running_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_games_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mac_addresses",
                    CMsgRemoteClientBroadcastStatus::get_mac_addresses_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_mac_addresses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "download_lan_peer_group",
                    CMsgRemoteClientBroadcastStatus::get_download_lan_peer_group_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_download_lan_peer_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "broadcasting_active",
                    CMsgRemoteClientBroadcastStatus::get_broadcasting_active_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_broadcasting_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "vr_active",
                    CMsgRemoteClientBroadcastStatus::get_vr_active_for_reflect,
                    CMsgRemoteClientBroadcastStatus::mut_vr_active_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastStatus>(
                    "CMsgRemoteClientBroadcastStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastStatus {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_min_version();
        self.clear_connect_port();
        self.clear_hostname();
        self.clear_enabled_services();
        self.clear_ostype();
        self.clear_is64bit();
        self.clear_users();
        self.clear_euniverse();
        self.clear_timestamp();
        self.clear_screen_locked();
        self.clear_games_running();
        self.clear_mac_addresses();
        self.clear_download_lan_peer_group();
        self.clear_broadcasting_active();
        self.clear_vr_active();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteClientBroadcastStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteClientBroadcastStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteClientBroadcastStatus_User {
    // message fields
    steamid: ::std::option::Option<u64>,
    auth_key_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteClientBroadcastStatus_User {}

impl CMsgRemoteClientBroadcastStatus_User {
    pub fn new() -> CMsgRemoteClientBroadcastStatus_User {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastStatus_User {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastStatus_User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastStatus_User,
        };
        unsafe {
            instance.get(CMsgRemoteClientBroadcastStatus_User::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional uint32 auth_key_id = 2;

    pub fn clear_auth_key_id(&mut self) {
        self.auth_key_id = ::std::option::Option::None;
    }

    pub fn has_auth_key_id(&self) -> bool {
        self.auth_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_key_id(&mut self, v: u32) {
        self.auth_key_id = ::std::option::Option::Some(v);
    }

    pub fn get_auth_key_id(&self) -> u32 {
        self.auth_key_id.unwrap_or(0)
    }

    fn get_auth_key_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.auth_key_id
    }

    fn mut_auth_key_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.auth_key_id
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastStatus_User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.auth_key_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        };
        if let Some(v) = self.auth_key_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.auth_key_id {
            os.write_uint32(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastStatus_User {
    fn new() -> CMsgRemoteClientBroadcastStatus_User {
        CMsgRemoteClientBroadcastStatus_User::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastStatus_User>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgRemoteClientBroadcastStatus_User::get_steamid_for_reflect,
                    CMsgRemoteClientBroadcastStatus_User::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "auth_key_id",
                    CMsgRemoteClientBroadcastStatus_User::get_auth_key_id_for_reflect,
                    CMsgRemoteClientBroadcastStatus_User::mut_auth_key_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastStatus_User>(
                    "CMsgRemoteClientBroadcastStatus_User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastStatus_User {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_auth_key_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteClientBroadcastStatus_User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteClientBroadcastStatus_User {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteClientBroadcastDiscovery {
    // message fields
    seq_num: ::std::option::Option<u32>,
    client_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteClientBroadcastDiscovery {}

impl CMsgRemoteClientBroadcastDiscovery {
    pub fn new() -> CMsgRemoteClientBroadcastDiscovery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteClientBroadcastDiscovery {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteClientBroadcastDiscovery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteClientBroadcastDiscovery,
        };
        unsafe {
            instance.get(CMsgRemoteClientBroadcastDiscovery::new)
        }
    }

    // optional uint32 seq_num = 1;

    pub fn clear_seq_num(&mut self) {
        self.seq_num = ::std::option::Option::None;
    }

    pub fn has_seq_num(&self) -> bool {
        self.seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num(&mut self, v: u32) {
        self.seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num(&self) -> u32 {
        self.seq_num.unwrap_or(0)
    }

    fn get_seq_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num
    }

    fn mut_seq_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num
    }

    // repeated uint64 client_ids = 2;

    pub fn clear_client_ids(&mut self) {
        self.client_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.client_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.client_ids
    }

    // Take field
    pub fn take_client_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.client_ids, ::std::vec::Vec::new())
    }

    pub fn get_client_ids(&self) -> &[u64] {
        &self.client_ids
    }

    fn get_client_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.client_ids
    }

    fn mut_client_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.client_ids
    }
}

impl ::protobuf::Message for CMsgRemoteClientBroadcastDiscovery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.client_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.seq_num {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.client_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seq_num {
            os.write_uint32(1, v)?;
        };
        for v in &self.client_ids {
            os.write_uint64(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteClientBroadcastDiscovery {
    fn new() -> CMsgRemoteClientBroadcastDiscovery {
        CMsgRemoteClientBroadcastDiscovery::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteClientBroadcastDiscovery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num",
                    CMsgRemoteClientBroadcastDiscovery::get_seq_num_for_reflect,
                    CMsgRemoteClientBroadcastDiscovery::mut_seq_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "client_ids",
                    CMsgRemoteClientBroadcastDiscovery::get_client_ids_for_reflect,
                    CMsgRemoteClientBroadcastDiscovery::mut_client_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteClientBroadcastDiscovery>(
                    "CMsgRemoteClientBroadcastDiscovery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteClientBroadcastDiscovery {
    fn clear(&mut self) {
        self.clear_seq_num();
        self.clear_client_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteClientBroadcastDiscovery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteClientBroadcastDiscovery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationRequest {
    // message fields
    device_token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    device_name: ::protobuf::SingularField<::std::string::String>,
    encrypted_request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceAuthorizationRequest {}

impl CMsgRemoteDeviceAuthorizationRequest {
    pub fn new() -> CMsgRemoteDeviceAuthorizationRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationRequest,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceAuthorizationRequest::new)
        }
    }

    // required bytes device_token = 1;

    pub fn clear_device_token(&mut self) {
        self.device_token.clear();
    }

    pub fn has_device_token(&self) -> bool {
        self.device_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.device_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_token(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.device_token.is_none() {
            self.device_token.set_default();
        };
        self.device_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_token(&mut self) -> ::std::vec::Vec<u8> {
        self.device_token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_device_token(&self) -> &[u8] {
        match self.device_token.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_device_token_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.device_token
    }

    fn mut_device_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.device_token
    }

    // optional string device_name = 2;

    pub fn clear_device_name(&mut self) {
        self.device_name.clear();
    }

    pub fn has_device_name(&self) -> bool {
        self.device_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_name(&mut self, v: ::std::string::String) {
        self.device_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_name(&mut self) -> &mut ::std::string::String {
        if self.device_name.is_none() {
            self.device_name.set_default();
        };
        self.device_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_name(&mut self) -> ::std::string::String {
        self.device_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_name(&self) -> &str {
        match self.device_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_name
    }

    fn mut_device_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_name
    }

    // required bytes encrypted_request = 3;

    pub fn clear_encrypted_request(&mut self) {
        self.encrypted_request.clear();
    }

    pub fn has_encrypted_request(&self) -> bool {
        self.encrypted_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_request(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_request.is_none() {
            self.encrypted_request.set_default();
        };
        self.encrypted_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_request(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_request(&self) -> &[u8] {
        match self.encrypted_request.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_request_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted_request
    }

    fn mut_encrypted_request_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted_request
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationRequest {
    fn is_initialized(&self) -> bool {
        if self.device_token.is_none() {
            return false;
        };
        if self.encrypted_request.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.device_token)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_request)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.device_token.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.device_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.encrypted_request.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.device_token.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.device_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.encrypted_request.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationRequest {
    fn new() -> CMsgRemoteDeviceAuthorizationRequest {
        CMsgRemoteDeviceAuthorizationRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "device_token",
                    CMsgRemoteDeviceAuthorizationRequest::get_device_token_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest::mut_device_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_name",
                    CMsgRemoteDeviceAuthorizationRequest::get_device_name_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest::mut_device_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_request",
                    CMsgRemoteDeviceAuthorizationRequest::get_encrypted_request_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest::mut_encrypted_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationRequest>(
                    "CMsgRemoteDeviceAuthorizationRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationRequest {
    fn clear(&mut self) {
        self.clear_device_token();
        self.clear_device_name();
        self.clear_encrypted_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceAuthorizationRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceAuthorizationRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    // message fields
    password: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    identifier: ::std::option::Option<u64>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u32>,
    usage: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage>,
    device_name: ::protobuf::SingularField<::std::string::String>,
    device_model: ::protobuf::SingularField<::std::string::String>,
    device_serial: ::protobuf::SingularField<::std::string::String>,
    device_provisioning_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {}

impl CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    pub fn new() -> CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::new)
        }
    }

    // optional bytes password = 1;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.password.is_none() {
            self.password.set_default();
        };
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        self.password.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        match self.password.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_password_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.password
    }

    // optional uint64 identifier = 2;

    pub fn clear_identifier(&mut self) {
        self.identifier = ::std::option::Option::None;
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: u64) {
        self.identifier = ::std::option::Option::Some(v);
    }

    pub fn get_identifier(&self) -> u64 {
        self.identifier.unwrap_or(0)
    }

    fn get_identifier_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.identifier
    }

    fn mut_identifier_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.identifier
    }

    // optional bytes payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // optional uint32 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timestamp
    }

    // optional .CMsgRemoteDeviceAuthorizationRequest.EKeyEscrowUsage usage = 5;

    pub fn clear_usage(&mut self) {
        self.usage = ::std::option::Option::None;
    }

    pub fn has_usage(&self) -> bool {
        self.usage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_usage(&mut self, v: CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage) {
        self.usage = ::std::option::Option::Some(v);
    }

    pub fn get_usage(&self) -> CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
        self.usage.unwrap_or(CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage::k_EKeyEscrowUsageStreamingDevice)
    }

    fn get_usage_for_reflect(&self) -> &::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage> {
        &self.usage
    }

    fn mut_usage_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage> {
        &mut self.usage
    }

    // optional string device_name = 6;

    pub fn clear_device_name(&mut self) {
        self.device_name.clear();
    }

    pub fn has_device_name(&self) -> bool {
        self.device_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_name(&mut self, v: ::std::string::String) {
        self.device_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_name(&mut self) -> &mut ::std::string::String {
        if self.device_name.is_none() {
            self.device_name.set_default();
        };
        self.device_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_name(&mut self) -> ::std::string::String {
        self.device_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_name(&self) -> &str {
        match self.device_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_name
    }

    fn mut_device_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_name
    }

    // optional string device_model = 7;

    pub fn clear_device_model(&mut self) {
        self.device_model.clear();
    }

    pub fn has_device_model(&self) -> bool {
        self.device_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_model(&mut self, v: ::std::string::String) {
        self.device_model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_model(&mut self) -> &mut ::std::string::String {
        if self.device_model.is_none() {
            self.device_model.set_default();
        };
        self.device_model.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_model(&mut self) -> ::std::string::String {
        self.device_model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_model(&self) -> &str {
        match self.device_model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_model_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_model
    }

    fn mut_device_model_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_model
    }

    // optional string device_serial = 8;

    pub fn clear_device_serial(&mut self) {
        self.device_serial.clear();
    }

    pub fn has_device_serial(&self) -> bool {
        self.device_serial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_serial(&mut self, v: ::std::string::String) {
        self.device_serial = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_serial(&mut self) -> &mut ::std::string::String {
        if self.device_serial.is_none() {
            self.device_serial.set_default();
        };
        self.device_serial.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_serial(&mut self) -> ::std::string::String {
        self.device_serial.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_serial(&self) -> &str {
        match self.device_serial.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_serial_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_serial
    }

    fn mut_device_serial_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_serial
    }

    // optional uint32 device_provisioning_id = 9;

    pub fn clear_device_provisioning_id(&mut self) {
        self.device_provisioning_id = ::std::option::Option::None;
    }

    pub fn has_device_provisioning_id(&self) -> bool {
        self.device_provisioning_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_provisioning_id(&mut self, v: u32) {
        self.device_provisioning_id = ::std::option::Option::Some(v);
    }

    pub fn get_device_provisioning_id(&self) -> u32 {
        self.device_provisioning_id.unwrap_or(0)
    }

    fn get_device_provisioning_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.device_provisioning_id
    }

    fn mut_device_provisioning_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.device_provisioning_id
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.password)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.identifier = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.usage = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_name)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_serial)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.device_provisioning_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.password.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.identifier {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.usage {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.device_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.device_model.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.device_serial.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.device_provisioning_id {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.password.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.identifier {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.payload.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.usage {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.device_name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.device_model.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.device_serial.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.device_provisioning_id {
            os.write_uint32(9, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn new() -> CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
        CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_password_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "identifier",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_identifier_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_payload_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_timestamp_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage>>(
                    "usage",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_usage_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_usage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_name",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_device_name_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_device_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_model",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_device_model_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_device_model_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_serial",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_device_serial_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_device_serial_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "device_provisioning_id",
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::get_device_provisioning_id_for_reflect,
                    CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket::mut_device_provisioning_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket>(
                    "CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn clear(&mut self) {
        self.clear_password();
        self.clear_identifier();
        self.clear_payload();
        self.clear_timestamp();
        self.clear_usage();
        self.clear_device_name();
        self.clear_device_model();
        self.clear_device_serial();
        self.clear_device_provisioning_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceAuthorizationRequest_CKeyEscrow_Ticket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
    k_EKeyEscrowUsageStreamingDevice = 0,
}

impl ::protobuf::ProtobufEnum for CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage> {
        match value {
            0 => ::std::option::Option::Some(CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage::k_EKeyEscrowUsageStreamingDevice),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage] = &[
            CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage::k_EKeyEscrowUsageStreamingDevice,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceAuthorizationRequest_EKeyEscrowUsage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationCancelRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceAuthorizationCancelRequest {}

impl CMsgRemoteDeviceAuthorizationCancelRequest {
    pub fn new() -> CMsgRemoteDeviceAuthorizationCancelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationCancelRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationCancelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationCancelRequest,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceAuthorizationCancelRequest::new)
        }
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationCancelRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationCancelRequest {
    fn new() -> CMsgRemoteDeviceAuthorizationCancelRequest {
        CMsgRemoteDeviceAuthorizationCancelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationCancelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationCancelRequest>(
                    "CMsgRemoteDeviceAuthorizationCancelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationCancelRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceAuthorizationCancelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceAuthorizationCancelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceAuthorizationResponse {
    // message fields
    result: ::std::option::Option<ERemoteDeviceAuthorizationResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceAuthorizationResponse {}

impl CMsgRemoteDeviceAuthorizationResponse {
    pub fn new() -> CMsgRemoteDeviceAuthorizationResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceAuthorizationResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceAuthorizationResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceAuthorizationResponse,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceAuthorizationResponse::new)
        }
    }

    // required .ERemoteDeviceAuthorizationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ERemoteDeviceAuthorizationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ERemoteDeviceAuthorizationResult {
        self.result.unwrap_or(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationSuccess)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ERemoteDeviceAuthorizationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ERemoteDeviceAuthorizationResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceAuthorizationResponse {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceAuthorizationResponse {
    fn new() -> CMsgRemoteDeviceAuthorizationResponse {
        CMsgRemoteDeviceAuthorizationResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceAuthorizationResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ERemoteDeviceAuthorizationResult>>(
                    "result",
                    CMsgRemoteDeviceAuthorizationResponse::get_result_for_reflect,
                    CMsgRemoteDeviceAuthorizationResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceAuthorizationResponse>(
                    "CMsgRemoteDeviceAuthorizationResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceAuthorizationResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceAuthorizationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceAuthorizationResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceStreamingRequest {
    // message fields
    request_id: ::std::option::Option<u32>,
    maximum_resolution_x: ::std::option::Option<i32>,
    maximum_resolution_y: ::std::option::Option<i32>,
    audio_channel_count: ::std::option::Option<i32>,
    device_version: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceStreamingRequest {}

impl CMsgRemoteDeviceStreamingRequest {
    pub fn new() -> CMsgRemoteDeviceStreamingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceStreamingRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceStreamingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceStreamingRequest,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceStreamingRequest::new)
        }
    }

    // required uint32 request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u32) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id(&self) -> u32 {
        self.request_id.unwrap_or(0)
    }

    fn get_request_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.request_id
    }

    fn mut_request_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.request_id
    }

    // optional int32 maximum_resolution_x = 2;

    pub fn clear_maximum_resolution_x(&mut self) {
        self.maximum_resolution_x = ::std::option::Option::None;
    }

    pub fn has_maximum_resolution_x(&self) -> bool {
        self.maximum_resolution_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum_resolution_x(&mut self, v: i32) {
        self.maximum_resolution_x = ::std::option::Option::Some(v);
    }

    pub fn get_maximum_resolution_x(&self) -> i32 {
        self.maximum_resolution_x.unwrap_or(0)
    }

    fn get_maximum_resolution_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.maximum_resolution_x
    }

    fn mut_maximum_resolution_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.maximum_resolution_x
    }

    // optional int32 maximum_resolution_y = 3;

    pub fn clear_maximum_resolution_y(&mut self) {
        self.maximum_resolution_y = ::std::option::Option::None;
    }

    pub fn has_maximum_resolution_y(&self) -> bool {
        self.maximum_resolution_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum_resolution_y(&mut self, v: i32) {
        self.maximum_resolution_y = ::std::option::Option::Some(v);
    }

    pub fn get_maximum_resolution_y(&self) -> i32 {
        self.maximum_resolution_y.unwrap_or(0)
    }

    fn get_maximum_resolution_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.maximum_resolution_y
    }

    fn mut_maximum_resolution_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.maximum_resolution_y
    }

    // optional int32 audio_channel_count = 4;

    pub fn clear_audio_channel_count(&mut self) {
        self.audio_channel_count = ::std::option::Option::None;
    }

    pub fn has_audio_channel_count(&self) -> bool {
        self.audio_channel_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audio_channel_count(&mut self, v: i32) {
        self.audio_channel_count = ::std::option::Option::Some(v);
    }

    pub fn get_audio_channel_count(&self) -> i32 {
        self.audio_channel_count.unwrap_or(2i32)
    }

    fn get_audio_channel_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.audio_channel_count
    }

    fn mut_audio_channel_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.audio_channel_count
    }

    // optional string device_version = 5;

    pub fn clear_device_version(&mut self) {
        self.device_version.clear();
    }

    pub fn has_device_version(&self) -> bool {
        self.device_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_version(&mut self, v: ::std::string::String) {
        self.device_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_version(&mut self) -> &mut ::std::string::String {
        if self.device_version.is_none() {
            self.device_version.set_default();
        };
        self.device_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_version(&mut self) -> ::std::string::String {
        self.device_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_version(&self) -> &str {
        match self.device_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_version
    }

    fn mut_device_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_version
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceStreamingRequest {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.maximum_resolution_x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.maximum_resolution_y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.audio_channel_count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.request_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.maximum_resolution_x {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.maximum_resolution_y {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.audio_channel_count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.device_version.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.maximum_resolution_x {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.maximum_resolution_y {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.audio_channel_count {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.device_version.as_ref() {
            os.write_string(5, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceStreamingRequest {
    fn new() -> CMsgRemoteDeviceStreamingRequest {
        CMsgRemoteDeviceStreamingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceStreamingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "request_id",
                    CMsgRemoteDeviceStreamingRequest::get_request_id_for_reflect,
                    CMsgRemoteDeviceStreamingRequest::mut_request_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "maximum_resolution_x",
                    CMsgRemoteDeviceStreamingRequest::get_maximum_resolution_x_for_reflect,
                    CMsgRemoteDeviceStreamingRequest::mut_maximum_resolution_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "maximum_resolution_y",
                    CMsgRemoteDeviceStreamingRequest::get_maximum_resolution_y_for_reflect,
                    CMsgRemoteDeviceStreamingRequest::mut_maximum_resolution_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "audio_channel_count",
                    CMsgRemoteDeviceStreamingRequest::get_audio_channel_count_for_reflect,
                    CMsgRemoteDeviceStreamingRequest::mut_audio_channel_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_version",
                    CMsgRemoteDeviceStreamingRequest::get_device_version_for_reflect,
                    CMsgRemoteDeviceStreamingRequest::mut_device_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceStreamingRequest>(
                    "CMsgRemoteDeviceStreamingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceStreamingRequest {
    fn clear(&mut self) {
        self.clear_request_id();
        self.clear_maximum_resolution_x();
        self.clear_maximum_resolution_y();
        self.clear_audio_channel_count();
        self.clear_device_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceStreamingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceStreamingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceStreamingCancelRequest {
    // message fields
    request_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceStreamingCancelRequest {}

impl CMsgRemoteDeviceStreamingCancelRequest {
    pub fn new() -> CMsgRemoteDeviceStreamingCancelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceStreamingCancelRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceStreamingCancelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceStreamingCancelRequest,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceStreamingCancelRequest::new)
        }
    }

    // required uint32 request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u32) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id(&self) -> u32 {
        self.request_id.unwrap_or(0)
    }

    fn get_request_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.request_id
    }

    fn mut_request_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.request_id
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceStreamingCancelRequest {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.request_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id {
            os.write_uint32(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceStreamingCancelRequest {
    fn new() -> CMsgRemoteDeviceStreamingCancelRequest {
        CMsgRemoteDeviceStreamingCancelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceStreamingCancelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "request_id",
                    CMsgRemoteDeviceStreamingCancelRequest::get_request_id_for_reflect,
                    CMsgRemoteDeviceStreamingCancelRequest::mut_request_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceStreamingCancelRequest>(
                    "CMsgRemoteDeviceStreamingCancelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceStreamingCancelRequest {
    fn clear(&mut self) {
        self.clear_request_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceStreamingCancelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceStreamingCancelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceStreamingResponse {
    // message fields
    request_id: ::std::option::Option<u32>,
    result: ::std::option::Option<ERemoteDeviceStreamingResult>,
    port: ::std::option::Option<u32>,
    encrypted_session_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    virtualhere_licensed_device_count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceStreamingResponse {}

impl CMsgRemoteDeviceStreamingResponse {
    pub fn new() -> CMsgRemoteDeviceStreamingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceStreamingResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceStreamingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceStreamingResponse,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceStreamingResponse::new)
        }
    }

    // required uint32 request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u32) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id(&self) -> u32 {
        self.request_id.unwrap_or(0)
    }

    fn get_request_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.request_id
    }

    fn mut_request_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.request_id
    }

    // required .ERemoteDeviceStreamingResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ERemoteDeviceStreamingResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ERemoteDeviceStreamingResult {
        self.result.unwrap_or(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingSuccess)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ERemoteDeviceStreamingResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ERemoteDeviceStreamingResult> {
        &mut self.result
    }

    // optional uint32 port = 3;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.port
    }

    // optional bytes encrypted_session_key = 4;

    pub fn clear_encrypted_session_key(&mut self) {
        self.encrypted_session_key.clear();
    }

    pub fn has_encrypted_session_key(&self) -> bool {
        self.encrypted_session_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_session_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_session_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_session_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_session_key.is_none() {
            self.encrypted_session_key.set_default();
        };
        self.encrypted_session_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_session_key(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_session_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_session_key(&self) -> &[u8] {
        match self.encrypted_session_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_session_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted_session_key
    }

    fn mut_encrypted_session_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted_session_key
    }

    // optional int32 virtualhere_licensed_device_count = 5;

    pub fn clear_virtualhere_licensed_device_count(&mut self) {
        self.virtualhere_licensed_device_count = ::std::option::Option::None;
    }

    pub fn has_virtualhere_licensed_device_count(&self) -> bool {
        self.virtualhere_licensed_device_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_virtualhere_licensed_device_count(&mut self, v: i32) {
        self.virtualhere_licensed_device_count = ::std::option::Option::Some(v);
    }

    pub fn get_virtualhere_licensed_device_count(&self) -> i32 {
        self.virtualhere_licensed_device_count.unwrap_or(0)
    }

    fn get_virtualhere_licensed_device_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.virtualhere_licensed_device_count
    }

    fn mut_virtualhere_licensed_device_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.virtualhere_licensed_device_count
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceStreamingResponse {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        if self.result.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_session_key)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.virtualhere_licensed_device_count = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.request_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.encrypted_session_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.virtualhere_licensed_device_count {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.port {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.encrypted_session_key.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.virtualhere_licensed_device_count {
            os.write_int32(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceStreamingResponse {
    fn new() -> CMsgRemoteDeviceStreamingResponse {
        CMsgRemoteDeviceStreamingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceStreamingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "request_id",
                    CMsgRemoteDeviceStreamingResponse::get_request_id_for_reflect,
                    CMsgRemoteDeviceStreamingResponse::mut_request_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ERemoteDeviceStreamingResult>>(
                    "result",
                    CMsgRemoteDeviceStreamingResponse::get_result_for_reflect,
                    CMsgRemoteDeviceStreamingResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    CMsgRemoteDeviceStreamingResponse::get_port_for_reflect,
                    CMsgRemoteDeviceStreamingResponse::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_session_key",
                    CMsgRemoteDeviceStreamingResponse::get_encrypted_session_key_for_reflect,
                    CMsgRemoteDeviceStreamingResponse::mut_encrypted_session_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "virtualhere_licensed_device_count",
                    CMsgRemoteDeviceStreamingResponse::get_virtualhere_licensed_device_count_for_reflect,
                    CMsgRemoteDeviceStreamingResponse::mut_virtualhere_licensed_device_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceStreamingResponse>(
                    "CMsgRemoteDeviceStreamingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceStreamingResponse {
    fn clear(&mut self) {
        self.clear_request_id();
        self.clear_result();
        self.clear_port();
        self.clear_encrypted_session_key();
        self.clear_virtualhere_licensed_device_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceStreamingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceStreamingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceProofRequest {
    // message fields
    challenge: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceProofRequest {}

impl CMsgRemoteDeviceProofRequest {
    pub fn new() -> CMsgRemoteDeviceProofRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceProofRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceProofRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceProofRequest,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceProofRequest::new)
        }
    }

    // required bytes challenge = 1;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: ::std::vec::Vec<u8>) {
        self.challenge = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.challenge.is_none() {
            self.challenge.set_default();
        };
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> ::std::vec::Vec<u8> {
        self.challenge.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_challenge(&self) -> &[u8] {
        match self.challenge.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.challenge
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceProofRequest {
    fn is_initialized(&self) -> bool {
        if self.challenge.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.challenge)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.challenge.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.challenge.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceProofRequest {
    fn new() -> CMsgRemoteDeviceProofRequest {
        CMsgRemoteDeviceProofRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceProofRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "challenge",
                    CMsgRemoteDeviceProofRequest::get_challenge_for_reflect,
                    CMsgRemoteDeviceProofRequest::mut_challenge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceProofRequest>(
                    "CMsgRemoteDeviceProofRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceProofRequest {
    fn clear(&mut self) {
        self.clear_challenge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceProofRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceProofRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRemoteDeviceProofResponse {
    // message fields
    response: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRemoteDeviceProofResponse {}

impl CMsgRemoteDeviceProofResponse {
    pub fn new() -> CMsgRemoteDeviceProofResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRemoteDeviceProofResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRemoteDeviceProofResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRemoteDeviceProofResponse,
        };
        unsafe {
            instance.get(CMsgRemoteDeviceProofResponse::new)
        }
    }

    // required bytes response = 1;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::vec::Vec<u8>) {
        self.response = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::vec::Vec<u8> {
        self.response.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_response(&self) -> &[u8] {
        match self.response.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.response
    }
}

impl ::protobuf::Message for CMsgRemoteDeviceProofResponse {
    fn is_initialized(&self) -> bool {
        if self.response.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.response)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.response.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRemoteDeviceProofResponse {
    fn new() -> CMsgRemoteDeviceProofResponse {
        CMsgRemoteDeviceProofResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRemoteDeviceProofResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "response",
                    CMsgRemoteDeviceProofResponse::get_response_for_reflect,
                    CMsgRemoteDeviceProofResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRemoteDeviceProofResponse>(
                    "CMsgRemoteDeviceProofResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRemoteDeviceProofResponse {
    fn clear(&mut self) {
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRemoteDeviceProofResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRemoteDeviceProofResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ERemoteClientBroadcastMsg {
    k_ERemoteClientBroadcastMsgDiscovery = 0,
    k_ERemoteClientBroadcastMsgStatus = 1,
    k_ERemoteClientBroadcastMsgOffline = 2,
    k_ERemoteDeviceAuthorizationRequest = 3,
    k_ERemoteDeviceAuthorizationResponse = 4,
    k_ERemoteDeviceStreamingRequest = 5,
    k_ERemoteDeviceStreamingResponse = 6,
    k_ERemoteDeviceProofRequest = 7,
    k_ERemoteDeviceProofResponse = 8,
    k_ERemoteDeviceAuthorizationCancelRequest = 9,
    k_ERemoteDeviceStreamingCancelRequest = 10,
}

impl ::protobuf::ProtobufEnum for ERemoteClientBroadcastMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteClientBroadcastMsg> {
        match value {
            0 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery),
            1 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgStatus),
            2 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgOffline),
            3 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationRequest),
            4 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationResponse),
            5 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingRequest),
            6 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingResponse),
            7 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceProofRequest),
            8 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceProofResponse),
            9 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationCancelRequest),
            10 => ::std::option::Option::Some(ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingCancelRequest),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ERemoteClientBroadcastMsg] = &[
            ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery,
            ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgStatus,
            ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgOffline,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationRequest,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationResponse,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingRequest,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingResponse,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceProofRequest,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceProofResponse,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceAuthorizationCancelRequest,
            ERemoteClientBroadcastMsg::k_ERemoteDeviceStreamingCancelRequest,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ERemoteClientBroadcastMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteClientBroadcastMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ERemoteClientBroadcastMsg {
}

impl ::protobuf::reflect::ProtobufValue for ERemoteClientBroadcastMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ERemoteClientService {
    k_ERemoteClientServiceNone = 0,
    k_ERemoteClientServiceRemoteControl = 1,
    k_ERemoteClientServiceGameStreaming = 2,
}

impl ::protobuf::ProtobufEnum for ERemoteClientService {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteClientService> {
        match value {
            0 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceNone),
            1 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceRemoteControl),
            2 => ::std::option::Option::Some(ERemoteClientService::k_ERemoteClientServiceGameStreaming),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ERemoteClientService] = &[
            ERemoteClientService::k_ERemoteClientServiceNone,
            ERemoteClientService::k_ERemoteClientServiceRemoteControl,
            ERemoteClientService::k_ERemoteClientServiceGameStreaming,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ERemoteClientService>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteClientService", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ERemoteClientService {
}

impl ::protobuf::reflect::ProtobufValue for ERemoteClientService {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ERemoteDeviceAuthorizationResult {
    k_ERemoteDeviceAuthorizationSuccess = 0,
    k_ERemoteDeviceAuthorizationDenied = 1,
    k_ERemoteDeviceAuthorizationNotLoggedIn = 2,
    k_ERemoteDeviceAuthorizationOffline = 3,
    k_ERemoteDeviceAuthorizationBusy = 4,
    k_ERemoteDeviceAuthorizationInProgress = 5,
    k_ERemoteDeviceAuthorizationTimedOut = 6,
    k_ERemoteDeviceAuthorizationFailed = 7,
    k_ERemoteDeviceAuthorizationCanceled = 8,
}

impl ::protobuf::ProtobufEnum for ERemoteDeviceAuthorizationResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteDeviceAuthorizationResult> {
        match value {
            0 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationSuccess),
            1 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationDenied),
            2 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationNotLoggedIn),
            3 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationOffline),
            4 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationBusy),
            5 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationInProgress),
            6 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationTimedOut),
            7 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationFailed),
            8 => ::std::option::Option::Some(ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationCanceled),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ERemoteDeviceAuthorizationResult] = &[
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationSuccess,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationDenied,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationNotLoggedIn,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationOffline,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationBusy,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationInProgress,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationTimedOut,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationFailed,
            ERemoteDeviceAuthorizationResult::k_ERemoteDeviceAuthorizationCanceled,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ERemoteDeviceAuthorizationResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteDeviceAuthorizationResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ERemoteDeviceAuthorizationResult {
}

impl ::protobuf::reflect::ProtobufValue for ERemoteDeviceAuthorizationResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ERemoteDeviceStreamingResult {
    k_ERemoteDeviceStreamingSuccess = 0,
    k_ERemoteDeviceStreamingUnauthorized = 1,
    k_ERemoteDeviceStreamingScreenLocked = 2,
    k_ERemoteDeviceStreamingFailed = 3,
    k_ERemoteDeviceStreamingBusy = 4,
    k_ERemoteDeviceStreamingInProgress = 5,
    k_ERemoteDeviceStreamingCanceled = 6,
    k_ERemoteDeviceStreamingDriversNotInstalled = 7,
    k_ERemoteDeviceStreamingDisabled = 8,
    k_ERemoteDeviceStreamingBroadcastingActive = 9,
    k_ERemoteDeviceStreamingVRActive = 10,
}

impl ::protobuf::ProtobufEnum for ERemoteDeviceStreamingResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ERemoteDeviceStreamingResult> {
        match value {
            0 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingSuccess),
            1 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingUnauthorized),
            2 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingScreenLocked),
            3 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingFailed),
            4 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingBusy),
            5 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingInProgress),
            6 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingCanceled),
            7 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingDriversNotInstalled),
            8 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingDisabled),
            9 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingBroadcastingActive),
            10 => ::std::option::Option::Some(ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingVRActive),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ERemoteDeviceStreamingResult] = &[
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingSuccess,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingUnauthorized,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingScreenLocked,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingFailed,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingBusy,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingInProgress,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingCanceled,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingDriversNotInstalled,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingDisabled,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingBroadcastingActive,
            ERemoteDeviceStreamingResult::k_ERemoteDeviceStreamingVRActive,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ERemoteDeviceStreamingResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ERemoteDeviceStreamingResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ERemoteDeviceStreamingResult {
}

impl ::protobuf::reflect::ProtobufValue for ERemoteDeviceStreamingResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x34, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x2f, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbc, 0x01, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64,
    0x63, 0x61, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x5b, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x4d, 0x73, 0x67, 0x3a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d,
    0x73, 0x67, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x07, 0x6d, 0x73, 0x67,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6e, 0x63, 0x65, 0x49, 0x64, 0x22, 0xb1, 0x05, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63,
    0x61, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x69, 0x6e, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x69, 0x6e, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x5f,
    0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x65,
    0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x12, 0x19,
    0x0a, 0x06, 0x6f, 0x73, 0x74, 0x79, 0x70, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01,
    0x30, 0x52, 0x06, 0x6f, 0x73, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x07, 0x69, 0x73, 0x36,
    0x34, 0x62, 0x69, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73,
    0x65, 0x52, 0x07, 0x69, 0x73, 0x36, 0x34, 0x62, 0x69, 0x74, 0x12, 0x3b, 0x0a, 0x05, 0x75, 0x73,
    0x65, 0x72, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61,
    0x64, 0x63, 0x61, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x72,
    0x52, 0x05, 0x75, 0x73, 0x65, 0x72, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x65, 0x75, 0x6e, 0x69, 0x76,
    0x65, 0x72, 0x73, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x65, 0x75, 0x6e, 0x69,
    0x76, 0x65, 0x72, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x5f, 0x6c, 0x6f,
    0x63, 0x6b, 0x65, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x73, 0x63, 0x72, 0x65,
    0x65, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x67, 0x61, 0x6d, 0x65,
    0x73, 0x5f, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0c, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x23, 0x0a,
    0x0d, 0x6d, 0x61, 0x63, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x0f,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x6d, 0x61, 0x63, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x12, 0x35, 0x0a, 0x17, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6c,
    0x61, 0x6e, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x10, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x14, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x4c, 0x61, 0x6e,
    0x50, 0x65, 0x65, 0x72, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x2f, 0x0a, 0x13, 0x62, 0x72, 0x6f,
    0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x76, 0x72,
    0x5f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x12, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x76,
    0x72, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x1a, 0x40, 0x0a, 0x04, 0x55, 0x73, 0x65, 0x72, 0x12,
    0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x75, 0x74,
    0x68, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09,
    0x61, 0x75, 0x74, 0x68, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x22, 0x5c, 0x0a, 0x22, 0x43, 0x4d, 0x73,
    0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f,
    0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x12,
    0x17, 0x0a, 0x07, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x06, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x04, 0x52, 0x09, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x22, 0xe8, 0x04, 0x0a, 0x24, 0x43, 0x4d, 0x73, 0x67,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68,
    0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x54, 0x6f,
    0x6b, 0x65, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2b, 0x0a, 0x11, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65,
    0x64, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x52,
    0x10, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x1a, 0x95, 0x03, 0x0a, 0x11, 0x43, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77,
    0x5f, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x1c, 0x0a,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x6d, 0x0a, 0x05, 0x75,
    0x73, 0x61, 0x67, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x35, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74,
    0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x2e, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67,
    0x65, 0x3a, 0x20, 0x6b, 0x5f, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55,
    0x73, 0x61, 0x67, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x52, 0x05, 0x75, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0a, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x23,
    0x0a, 0x0d, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x12, 0x34, 0x0a, 0x16, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x14, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x76, 0x69,
    0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x49, 0x64, 0x22, 0x37, 0x0a, 0x0f, 0x45, 0x4b, 0x65,
    0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67, 0x65, 0x12, 0x24, 0x0a, 0x20,
    0x6b, 0x5f, 0x45, 0x4b, 0x65, 0x79, 0x45, 0x73, 0x63, 0x72, 0x6f, 0x77, 0x55, 0x73, 0x61, 0x67,
    0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x10, 0x00, 0x22, 0x2c, 0x0a, 0x2a, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x22, 0x87, 0x01, 0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5e, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x23, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xff, 0x01, 0x0a, 0x20, 0x43,
    0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0d, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0x12, 0x30,
    0x0a, 0x14, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x12, 0x6d, 0x61,
    0x78, 0x69, 0x6d, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x58,
    0x12, 0x30, 0x0a, 0x14, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x12,
    0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x59, 0x12, 0x31, 0x0a, 0x13, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x5f, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x3a,
    0x01, 0x32, 0x52, 0x11, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x47, 0x0a, 0x26,
    0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x49, 0x64, 0x22, 0xad, 0x02, 0x0a, 0x21, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52,
    0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x1f, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x32, 0x0a, 0x15, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x65, 0x64, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64,
    0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x4b, 0x65, 0x79, 0x12, 0x49, 0x0a, 0x21, 0x76, 0x69,
    0x72, 0x74, 0x75, 0x61, 0x6c, 0x68, 0x65, 0x72, 0x65, 0x5f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x64, 0x5f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x1e, 0x76, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6c, 0x68, 0x65,
    0x72, 0x65, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x3c, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e,
    0x67, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65,
    0x6e, 0x67, 0x65, 0x22, 0x3b, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x6d, 0x6f, 0x74,
    0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2a, 0xcf, 0x03, 0x0a, 0x19, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x28,
    0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x44, 0x69, 0x73,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x10, 0x00, 0x12, 0x25, 0x0a, 0x21, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64,
    0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x10, 0x01, 0x12,
    0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x4f, 0x66,
    0x66, 0x6c, 0x69, 0x6e, 0x65, 0x10, 0x02, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x03,
    0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x04, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f,
    0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x05, 0x12,
    0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x10, 0x06, 0x12, 0x1f, 0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x10, 0x07, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x08, 0x12, 0x2d, 0x0a, 0x29, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x09, 0x12, 0x29, 0x0a, 0x25, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x10, 0x0a, 0x2a, 0x88, 0x01, 0x0a, 0x14, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x1e, 0x0a, 0x1a, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12, 0x27, 0x0a, 0x23, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x10, 0x01, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74,
    0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x47, 0x61,
    0x6d, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x10, 0x02, 0x2a, 0x97, 0x03,
    0x0a, 0x20, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41,
    0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x10, 0x00, 0x12, 0x26, 0x0a, 0x22, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x6e, 0x69, 0x65,
    0x64, 0x10, 0x01, 0x12, 0x2b, 0x0a, 0x27, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x74, 0x4c, 0x6f, 0x67, 0x67, 0x65, 0x64, 0x49, 0x6e, 0x10, 0x02,
    0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4f, 0x66, 0x66, 0x6c, 0x69, 0x6e, 0x65, 0x10, 0x03, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68,
    0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x75, 0x73, 0x79, 0x10, 0x04, 0x12,
    0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x10, 0x05, 0x12, 0x28, 0x0a, 0x24, 0x6b,
    0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x64,
    0x4f, 0x75, 0x74, 0x10, 0x06, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x10, 0x07, 0x12, 0x28, 0x0a,
    0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x61, 0x6e,
    0x63, 0x65, 0x6c, 0x65, 0x64, 0x10, 0x08, 0x2a, 0xd8, 0x03, 0x0a, 0x1c, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x69, 0x6e, 0x67, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x10, 0x00, 0x12, 0x28, 0x0a,
    0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x55, 0x6e, 0x61, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x65, 0x64, 0x10, 0x01, 0x12, 0x28, 0x0a, 0x24, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x10,
    0x02, 0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x46, 0x61, 0x69,
    0x6c, 0x65, 0x64, 0x10, 0x03, 0x12, 0x20, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e,
    0x67, 0x42, 0x75, 0x73, 0x79, 0x10, 0x04, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x52, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x69, 0x6e, 0x67, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x10, 0x05, 0x12,
    0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x43, 0x61, 0x6e, 0x63, 0x65,
    0x6c, 0x65, 0x64, 0x10, 0x06, 0x12, 0x2f, 0x0a, 0x2b, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e,
    0x67, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x73, 0x4e, 0x6f, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61,
    0x6c, 0x6c, 0x65, 0x64, 0x10, 0x07, 0x12, 0x24, 0x0a, 0x20, 0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69,
    0x6e, 0x67, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x10, 0x08, 0x12, 0x2e, 0x0a, 0x2a,
    0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x10, 0x09, 0x12, 0x24, 0x0a, 0x20,
    0x6b, 0x5f, 0x45, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x56, 0x52, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x10, 0x0a, 0x42, 0x02, 0x48, 0x01, 0x4a, 0xc1, 0x2c, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x8a,
    0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x00, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x00, 0x16, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x03, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x2c, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x05, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x06, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06,
    0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x07, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x08, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x08, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x08,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x09, 0x08, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x09, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x09, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x0a, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x0a, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x0a, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0b, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0c, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x0c, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x0d, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x0d, 0x08,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x0d, 0x30, 0x32, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x10, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x01, 0x01, 0x12, 0x03, 0x10, 0x05, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x11, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x13, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13,
    0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x16, 0x00, 0x20, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x16, 0x05, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x17, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x17, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x18, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x19, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x19, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a,
    0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1a, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x1b, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1c, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x1c, 0x31, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x07, 0x12, 0x03, 0x1e, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x1e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1e,
    0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1f, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x08, 0x02, 0x12, 0x03, 0x1f, 0x2f, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x22, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x03,
    0x22, 0x05, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x03, 0x23, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03, 0x23, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x24, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x24, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x02, 0x12, 0x03, 0x25,
    0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x25, 0x2f, 0x30, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x03, 0x02, 0x03, 0x12, 0x03, 0x26, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x26, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x04, 0x12,
    0x03, 0x27, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x27,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x02, 0x12, 0x03, 0x27, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x05, 0x12, 0x03, 0x28, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x28, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x05, 0x02, 0x12, 0x03, 0x28, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02,
    0x06, 0x12, 0x03, 0x29, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x29, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x06, 0x02, 0x12, 0x03, 0x29,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x07, 0x12, 0x03, 0x2a, 0x08, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x07, 0x02, 0x12, 0x03, 0x2a, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x03, 0x02, 0x08, 0x12, 0x03, 0x2b, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x2b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x08, 0x02, 0x12,
    0x03, 0x2b, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x09, 0x12, 0x03, 0x2c, 0x08,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x09, 0x02, 0x12, 0x03, 0x2c, 0x35, 0x36, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x03, 0x02, 0x0a, 0x12, 0x03, 0x2d, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0a,
    0x02, 0x12, 0x03, 0x2d, 0x2b, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x30, 0x00,
    0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x30, 0x08, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x31, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x31, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x31, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x32, 0x08, 0x6a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x32, 0x11, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x2c, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x08, 0x12, 0x03, 0x32, 0x39, 0x69, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07, 0x12,
    0x03, 0x32, 0x44, 0x68, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x33, 0x08,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x33, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x36, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x36, 0x08,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x37, 0x08, 0x3a, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x37, 0x10, 0x14, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x38, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x21, 0x28, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x39, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c,
    0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x3d, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x3d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x3e,
    0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x3f, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x3f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x18,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3f, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x40, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x40, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x40, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x40, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x41, 0x08,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x41, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x41, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x41, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x41, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x08, 0x12, 0x03, 0x41, 0x22, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x07,
    0x12, 0x03, 0x41, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x42,
    0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x42, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x42, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x42, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x42, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x08, 0x12, 0x03, 0x42, 0x22, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x07, 0x12, 0x03, 0x42, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03,
    0x43, 0x08, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x43, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x06, 0x12, 0x03, 0x43, 0x11, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x43, 0x37, 0x3c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x43, 0x3f, 0x40, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x08, 0x12, 0x03, 0x44, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x44, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x44, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x44,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x44, 0x23, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x45, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x45, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x45, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x45, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x46,
    0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x46, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x46, 0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x46, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x0b, 0x12, 0x03, 0x47, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x47, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x47, 0x16,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x47, 0x26, 0x28, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x48, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x48, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x48, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x48, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x49, 0x08,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x49, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x49, 0x18, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x49, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x0e, 0x12, 0x03, 0x4a, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x04, 0x12,
    0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x4a,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x4a, 0x16, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x4a, 0x2c, 0x2e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x4b, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0f, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x01,
    0x12, 0x03, 0x4b, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x4b, 0x22, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x4e, 0x00, 0x51, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x4f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f,
    0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x50, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x50, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x50, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x50, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x53, 0x00, 0x67,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x53, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x54, 0x08, 0x5e, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x54, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x55, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x1f, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x56, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x56, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x56, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x56, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x56, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x57, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x57, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x57, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x57, 0x1f, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x57, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x58, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x58, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x58, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x58, 0x20, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x58, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x12,
    0x04, 0x59, 0x10, 0x86, 0x01, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x59, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x59, 0x19, 0x4e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x59, 0x4f, 0x54, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x59, 0x57, 0x58, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x59, 0x59, 0x85, 0x01, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x59, 0x64, 0x84, 0x01, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x5a, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x5a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x5a, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x5a, 0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x5a, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x5b, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x5b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x5b, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x5b, 0x20, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x5b, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x5c, 0x10, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x5c, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x5c, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x5c, 0x20, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x5c, 0x30, 0x31, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x5d, 0x10, 0x3b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03,
    0x5d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x5d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x5d, 0x20, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x5d, 0x39, 0x3a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x60, 0x08, 0x62,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x60, 0x0d, 0x1c, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x61, 0x10, 0x35, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x10, 0x30, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x61, 0x33, 0x34, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x64, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x64, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x64, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x64, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x65, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x65, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02,
    0x12, 0x03, 0x66, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x66, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x66, 0x17, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x66, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x69, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x69, 0x08, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x6c, 0x00, 0x6e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x08, 0x6e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x6d, 0x11, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6d, 0x33, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d,
    0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x08, 0x12, 0x03, 0x6d, 0x3e, 0x6d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x07, 0x12, 0x03, 0x6d, 0x49, 0x6c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x70, 0x00, 0x76, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x70, 0x08, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x71, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x71, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x71, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x71, 0x18, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x71, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x01, 0x12, 0x03, 0x72, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x72, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72,
    0x17, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x73, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x73, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x73, 0x17, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x73, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x74,
    0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x74, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x74, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x74, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x74, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x08, 0x12, 0x03, 0x74, 0x2f, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x07, 0x12, 0x03, 0x74, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03,
    0x75, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x75, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x75, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x75, 0x18, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x75, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x78, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03,
    0x78, 0x08, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x79, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x79, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x05,
    0x7c, 0x00, 0x82, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x7c, 0x08,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x7d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03,
    0x7e, 0x08, 0x66, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7e, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x7e, 0x11, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7e, 0x2f, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7e, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x08, 0x12, 0x03, 0x7e, 0x3a, 0x65, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x07, 0x12, 0x03, 0x7e, 0x45, 0x64, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12,
    0x03, 0x7f, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7f,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7f, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x18, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7f, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x80, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x05, 0x12, 0x04, 0x80, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x80, 0x01, 0x17, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03,
    0x12, 0x04, 0x80, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x04,
    0x81, 0x01, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0x81,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x04, 0x81, 0x01,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0x81, 0x01, 0x17,
    0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0x81, 0x01, 0x3b, 0x3c,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0x84, 0x01, 0x00, 0x86, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0x84, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x04, 0x85, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x85, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x85, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x85, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x85, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x88, 0x01, 0x00,
    0x8a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x88, 0x01, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0x89, 0x01, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x89, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x89, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x89, 0x01, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x89, 0x01, 0x22, 0x23,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
