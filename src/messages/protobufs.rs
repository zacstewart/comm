// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct UdpTransport {
    // message fields
    ip_address: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    port: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UdpTransport {}

impl UdpTransport {
    pub fn new() -> UdpTransport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UdpTransport {
        static mut instance: ::protobuf::lazy::Lazy<UdpTransport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UdpTransport,
        };
        unsafe {
            instance.get(|| {
                UdpTransport {
                    ip_address: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes ip_address = 1;

    pub fn clear_ip_address(&mut self) {
        self.ip_address.clear();
    }

    pub fn has_ip_address(&self) -> bool {
        self.ip_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.ip_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ip_address.is_none() {
            self.ip_address.set_default();
        };
        self.ip_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip_address(&mut self) -> ::std::vec::Vec<u8> {
        self.ip_address.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ip_address(&self) -> &[u8] {
        match self.ip_address.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 port = 2;

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
}

impl ::protobuf::Message for UdpTransport {
    fn is_initialized(&self) -> bool {
        if self.ip_address.is_none() {
            return false;
        };
        if self.port.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ip_address));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ip_address.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.port.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip_address.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UdpTransport>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UdpTransport {
    fn new() -> UdpTransport {
        UdpTransport::new()
    }

    fn descriptor_static(_: ::std::option::Option<UdpTransport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ip_address",
                    UdpTransport::has_ip_address,
                    UdpTransport::get_ip_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    UdpTransport::has_port,
                    UdpTransport::get_port,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UdpTransport>(
                    "UdpTransport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UdpTransport {
    fn clear(&mut self) {
        self.clear_ip_address();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UdpTransport {
    fn eq(&self, other: &UdpTransport) -> bool {
        self.ip_address == other.ip_address &&
        self.port == other.port &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UdpTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Transport {
    // message fields
    transport_type: ::std::option::Option<Transport_Type>,
    udp_transport: ::protobuf::SingularPtrField<UdpTransport>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transport {}

impl Transport {
    pub fn new() -> Transport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transport {
        static mut instance: ::protobuf::lazy::Lazy<Transport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transport,
        };
        unsafe {
            instance.get(|| {
                Transport {
                    transport_type: ::std::option::Option::None,
                    udp_transport: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Transport.Type transport_type = 1;

    pub fn clear_transport_type(&mut self) {
        self.transport_type = ::std::option::Option::None;
    }

    pub fn has_transport_type(&self) -> bool {
        self.transport_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_type(&mut self, v: Transport_Type) {
        self.transport_type = ::std::option::Option::Some(v);
    }

    pub fn get_transport_type(&self) -> Transport_Type {
        self.transport_type.unwrap_or(Transport_Type::UDP)
    }

    // optional .UdpTransport udp_transport = 2;

    pub fn clear_udp_transport(&mut self) {
        self.udp_transport.clear();
    }

    pub fn has_udp_transport(&self) -> bool {
        self.udp_transport.is_some()
    }

    // Param is passed by value, moved
    pub fn set_udp_transport(&mut self, v: UdpTransport) {
        self.udp_transport = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_udp_transport(&mut self) -> &mut UdpTransport {
        if self.udp_transport.is_none() {
            self.udp_transport.set_default();
        };
        self.udp_transport.as_mut().unwrap()
    }

    // Take field
    pub fn take_udp_transport(&mut self) -> UdpTransport {
        self.udp_transport.take().unwrap_or_else(|| UdpTransport::new())
    }

    pub fn get_udp_transport(&self) -> &UdpTransport {
        self.udp_transport.as_ref().unwrap_or_else(|| UdpTransport::default_instance())
    }
}

impl ::protobuf::Message for Transport {
    fn is_initialized(&self) -> bool {
        if self.transport_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.transport_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.udp_transport));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.transport_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.udp_transport.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transport_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.udp_transport.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Transport>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Transport {
    fn new() -> Transport {
        Transport::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "transport_type",
                    Transport::has_transport_type,
                    Transport::get_transport_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "udp_transport",
                    Transport::has_udp_transport,
                    Transport::get_udp_transport,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transport>(
                    "Transport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transport {
    fn clear(&mut self) {
        self.clear_transport_type();
        self.clear_udp_transport();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Transport {
    fn eq(&self, other: &Transport) -> bool {
        self.transport_type == other.transport_type &&
        self.udp_transport == other.udp_transport &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Transport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Transport_Type {
    UDP = 1,
}

impl ::protobuf::ProtobufEnum for Transport_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Transport_Type> {
        match value {
            1 => ::std::option::Option::Some(Transport_Type::UDP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Transport_Type] = &[
            Transport_Type::UDP,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Transport_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Transport_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Transport_Type {
}

#[derive(Clone,Default)]
pub struct Node {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    transports: ::protobuf::RepeatedField<Transport>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(|| {
                Node {
                    id: ::protobuf::SingularField::none(),
                    transports: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Transport transports = 2;

    pub fn clear_transports(&mut self) {
        self.transports.clear();
    }

    // Param is passed by value, moved
    pub fn set_transports(&mut self, v: ::protobuf::RepeatedField<Transport>) {
        self.transports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transports(&mut self) -> &mut ::protobuf::RepeatedField<Transport> {
        &mut self.transports
    }

    // Take field
    pub fn take_transports(&mut self) -> ::protobuf::RepeatedField<Transport> {
        ::std::mem::replace(&mut self.transports, ::protobuf::RepeatedField::new())
    }

    pub fn get_transports(&self) -> &[Transport] {
        &self.transports
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transports));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.transports.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.transports.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Node>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    Node::has_id,
                    Node::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "transports",
                    Node::get_transports,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_transports();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id &&
        self.transports == other.transports &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FindNodeQuery {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FindNodeQuery {}

impl FindNodeQuery {
    pub fn new() -> FindNodeQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FindNodeQuery {
        static mut instance: ::protobuf::lazy::Lazy<FindNodeQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FindNodeQuery,
        };
        unsafe {
            instance.get(|| {
                FindNodeQuery {
                    origin: ::protobuf::SingularPtrField::none(),
                    target: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    // required string target = 2;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        };
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for FindNodeQuery {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        if self.target.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.target.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.target.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FindNodeQuery>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FindNodeQuery {
    fn new() -> FindNodeQuery {
        FindNodeQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<FindNodeQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    FindNodeQuery::has_origin,
                    FindNodeQuery::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "target",
                    FindNodeQuery::has_target,
                    FindNodeQuery::get_target,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FindNodeQuery>(
                    "FindNodeQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FindNodeQuery {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FindNodeQuery {
    fn eq(&self, other: &FindNodeQuery) -> bool {
        self.origin == other.origin &&
        self.target == other.target &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FindNodeQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FindNodeResponse {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    nodes: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FindNodeResponse {}

impl FindNodeResponse {
    pub fn new() -> FindNodeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FindNodeResponse {
        static mut instance: ::protobuf::lazy::Lazy<FindNodeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FindNodeResponse,
        };
        unsafe {
            instance.get(|| {
                FindNodeResponse {
                    origin: ::protobuf::SingularPtrField::none(),
                    nodes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    // repeated .Node nodes = 2;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }
}

impl ::protobuf::Message for FindNodeResponse {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.nodes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.nodes.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FindNodeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FindNodeResponse {
    fn new() -> FindNodeResponse {
        FindNodeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FindNodeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    FindNodeResponse::has_origin,
                    FindNodeResponse::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "nodes",
                    FindNodeResponse::get_nodes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FindNodeResponse>(
                    "FindNodeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FindNodeResponse {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FindNodeResponse {
    fn eq(&self, other: &FindNodeResponse) -> bool {
        self.origin == other.origin &&
        self.nodes == other.nodes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FindNodeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingQuery {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingQuery {}

impl PingQuery {
    pub fn new() -> PingQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingQuery {
        static mut instance: ::protobuf::lazy::Lazy<PingQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingQuery,
        };
        unsafe {
            instance.get(|| {
                PingQuery {
                    origin: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }
}

impl ::protobuf::Message for PingQuery {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingQuery>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingQuery {
    fn new() -> PingQuery {
        PingQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    PingQuery::has_origin,
                    PingQuery::get_origin,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingQuery>(
                    "PingQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingQuery {
    fn clear(&mut self) {
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingQuery {
    fn eq(&self, other: &PingQuery) -> bool {
        self.origin == other.origin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingResponse {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResponse {}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResponse {
        static mut instance: ::protobuf::lazy::Lazy<PingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResponse,
        };
        unsafe {
            instance.get(|| {
                PingResponse {
                    origin: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingResponse {
    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    PingResponse::has_origin,
                    PingResponse::get_origin,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingResponse>(
                    "PingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingResponse {
    fn eq(&self, other: &PingResponse) -> bool {
        self.origin == other.origin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PacketQuery {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PacketQuery {}

impl PacketQuery {
    pub fn new() -> PacketQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PacketQuery {
        static mut instance: ::protobuf::lazy::Lazy<PacketQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PacketQuery,
        };
        unsafe {
            instance.get(|| {
                PacketQuery {
                    origin: ::protobuf::SingularPtrField::none(),
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    // required bytes payload = 2;

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
}

impl ::protobuf::Message for PacketQuery {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        if self.payload.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PacketQuery>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PacketQuery {
    fn new() -> PacketQuery {
        PacketQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<PacketQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    PacketQuery::has_origin,
                    PacketQuery::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    PacketQuery::has_payload,
                    PacketQuery::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PacketQuery>(
                    "PacketQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PacketQuery {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PacketQuery {
    fn eq(&self, other: &PacketQuery) -> bool {
        self.origin == other.origin &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PacketQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PacketResponse {
    // message fields
    origin: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PacketResponse {}

impl PacketResponse {
    pub fn new() -> PacketResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PacketResponse {
        static mut instance: ::protobuf::lazy::Lazy<PacketResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PacketResponse,
        };
        unsafe {
            instance.get(|| {
                PacketResponse {
                    origin: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: Node) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut Node {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> Node {
        self.origin.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_origin(&self) -> &Node {
        self.origin.as_ref().unwrap_or_else(|| Node::default_instance())
    }
}

impl ::protobuf::Message for PacketResponse {
    fn is_initialized(&self) -> bool {
        if self.origin.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.origin.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PacketResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PacketResponse {
    fn new() -> PacketResponse {
        PacketResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PacketResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    PacketResponse::has_origin,
                    PacketResponse::get_origin,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PacketResponse>(
                    "PacketResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PacketResponse {
    fn clear(&mut self) {
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PacketResponse {
    fn eq(&self, other: &PacketResponse) -> bool {
        self.origin == other.origin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PacketResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Envelope {
    // message fields
    message_type: ::std::option::Option<Envelope_Type>,
    transaction_id: ::std::option::Option<u32>,
    find_node_query: ::protobuf::SingularPtrField<FindNodeQuery>,
    find_node_response: ::protobuf::SingularPtrField<FindNodeResponse>,
    ping_query: ::protobuf::SingularPtrField<PingQuery>,
    ping_response: ::protobuf::SingularPtrField<PingResponse>,
    packet_query: ::protobuf::SingularPtrField<PacketQuery>,
    packet_response: ::protobuf::SingularPtrField<PacketResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Envelope {}

impl Envelope {
    pub fn new() -> Envelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Envelope {
        static mut instance: ::protobuf::lazy::Lazy<Envelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Envelope,
        };
        unsafe {
            instance.get(|| {
                Envelope {
                    message_type: ::std::option::Option::None,
                    transaction_id: ::std::option::Option::None,
                    find_node_query: ::protobuf::SingularPtrField::none(),
                    find_node_response: ::protobuf::SingularPtrField::none(),
                    ping_query: ::protobuf::SingularPtrField::none(),
                    ping_response: ::protobuf::SingularPtrField::none(),
                    packet_query: ::protobuf::SingularPtrField::none(),
                    packet_response: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Envelope.Type message_type = 1;

    pub fn clear_message_type(&mut self) {
        self.message_type = ::std::option::Option::None;
    }

    pub fn has_message_type(&self) -> bool {
        self.message_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: Envelope_Type) {
        self.message_type = ::std::option::Option::Some(v);
    }

    pub fn get_message_type(&self) -> Envelope_Type {
        self.message_type.unwrap_or(Envelope_Type::FIND_NODE_QUERY)
    }

    // required uint32 transaction_id = 2;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: u32) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> u32 {
        self.transaction_id.unwrap_or(0)
    }

    // optional .FindNodeQuery find_node_query = 3;

    pub fn clear_find_node_query(&mut self) {
        self.find_node_query.clear();
    }

    pub fn has_find_node_query(&self) -> bool {
        self.find_node_query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_find_node_query(&mut self, v: FindNodeQuery) {
        self.find_node_query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_find_node_query(&mut self) -> &mut FindNodeQuery {
        if self.find_node_query.is_none() {
            self.find_node_query.set_default();
        };
        self.find_node_query.as_mut().unwrap()
    }

    // Take field
    pub fn take_find_node_query(&mut self) -> FindNodeQuery {
        self.find_node_query.take().unwrap_or_else(|| FindNodeQuery::new())
    }

    pub fn get_find_node_query(&self) -> &FindNodeQuery {
        self.find_node_query.as_ref().unwrap_or_else(|| FindNodeQuery::default_instance())
    }

    // optional .FindNodeResponse find_node_response = 4;

    pub fn clear_find_node_response(&mut self) {
        self.find_node_response.clear();
    }

    pub fn has_find_node_response(&self) -> bool {
        self.find_node_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_find_node_response(&mut self, v: FindNodeResponse) {
        self.find_node_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_find_node_response(&mut self) -> &mut FindNodeResponse {
        if self.find_node_response.is_none() {
            self.find_node_response.set_default();
        };
        self.find_node_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_find_node_response(&mut self) -> FindNodeResponse {
        self.find_node_response.take().unwrap_or_else(|| FindNodeResponse::new())
    }

    pub fn get_find_node_response(&self) -> &FindNodeResponse {
        self.find_node_response.as_ref().unwrap_or_else(|| FindNodeResponse::default_instance())
    }

    // optional .PingQuery ping_query = 5;

    pub fn clear_ping_query(&mut self) {
        self.ping_query.clear();
    }

    pub fn has_ping_query(&self) -> bool {
        self.ping_query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_query(&mut self, v: PingQuery) {
        self.ping_query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_query(&mut self) -> &mut PingQuery {
        if self.ping_query.is_none() {
            self.ping_query.set_default();
        };
        self.ping_query.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_query(&mut self) -> PingQuery {
        self.ping_query.take().unwrap_or_else(|| PingQuery::new())
    }

    pub fn get_ping_query(&self) -> &PingQuery {
        self.ping_query.as_ref().unwrap_or_else(|| PingQuery::default_instance())
    }

    // optional .PingResponse ping_response = 6;

    pub fn clear_ping_response(&mut self) {
        self.ping_response.clear();
    }

    pub fn has_ping_response(&self) -> bool {
        self.ping_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_response(&mut self, v: PingResponse) {
        self.ping_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_response(&mut self) -> &mut PingResponse {
        if self.ping_response.is_none() {
            self.ping_response.set_default();
        };
        self.ping_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_response(&mut self) -> PingResponse {
        self.ping_response.take().unwrap_or_else(|| PingResponse::new())
    }

    pub fn get_ping_response(&self) -> &PingResponse {
        self.ping_response.as_ref().unwrap_or_else(|| PingResponse::default_instance())
    }

    // optional .PacketQuery packet_query = 7;

    pub fn clear_packet_query(&mut self) {
        self.packet_query.clear();
    }

    pub fn has_packet_query(&self) -> bool {
        self.packet_query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_query(&mut self, v: PacketQuery) {
        self.packet_query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet_query(&mut self) -> &mut PacketQuery {
        if self.packet_query.is_none() {
            self.packet_query.set_default();
        };
        self.packet_query.as_mut().unwrap()
    }

    // Take field
    pub fn take_packet_query(&mut self) -> PacketQuery {
        self.packet_query.take().unwrap_or_else(|| PacketQuery::new())
    }

    pub fn get_packet_query(&self) -> &PacketQuery {
        self.packet_query.as_ref().unwrap_or_else(|| PacketQuery::default_instance())
    }

    // optional .PacketResponse packet_response = 8;

    pub fn clear_packet_response(&mut self) {
        self.packet_response.clear();
    }

    pub fn has_packet_response(&self) -> bool {
        self.packet_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_response(&mut self, v: PacketResponse) {
        self.packet_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet_response(&mut self) -> &mut PacketResponse {
        if self.packet_response.is_none() {
            self.packet_response.set_default();
        };
        self.packet_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_packet_response(&mut self) -> PacketResponse {
        self.packet_response.take().unwrap_or_else(|| PacketResponse::new())
    }

    pub fn get_packet_response(&self) -> &PacketResponse {
        self.packet_response.as_ref().unwrap_or_else(|| PacketResponse::default_instance())
    }
}

impl ::protobuf::Message for Envelope {
    fn is_initialized(&self) -> bool {
        if self.message_type.is_none() {
            return false;
        };
        if self.transaction_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.message_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.find_node_query));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.find_node_response));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_query));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_response));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.packet_query));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.packet_response));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.transaction_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.find_node_query.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.find_node_response.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ping_query.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ping_response.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.packet_query.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.packet_response.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.transaction_id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.find_node_query.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.find_node_response.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ping_query.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ping_response.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.packet_query.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.packet_response.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Envelope>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Envelope {
    fn new() -> Envelope {
        Envelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<Envelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "message_type",
                    Envelope::has_message_type,
                    Envelope::get_message_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "transaction_id",
                    Envelope::has_transaction_id,
                    Envelope::get_transaction_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "find_node_query",
                    Envelope::has_find_node_query,
                    Envelope::get_find_node_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "find_node_response",
                    Envelope::has_find_node_response,
                    Envelope::get_find_node_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_query",
                    Envelope::has_ping_query,
                    Envelope::get_ping_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_response",
                    Envelope::has_ping_response,
                    Envelope::get_ping_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "packet_query",
                    Envelope::has_packet_query,
                    Envelope::get_packet_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "packet_response",
                    Envelope::has_packet_response,
                    Envelope::get_packet_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Envelope>(
                    "Envelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Envelope {
    fn clear(&mut self) {
        self.clear_message_type();
        self.clear_transaction_id();
        self.clear_find_node_query();
        self.clear_find_node_response();
        self.clear_ping_query();
        self.clear_ping_response();
        self.clear_packet_query();
        self.clear_packet_response();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Envelope {
    fn eq(&self, other: &Envelope) -> bool {
        self.message_type == other.message_type &&
        self.transaction_id == other.transaction_id &&
        self.find_node_query == other.find_node_query &&
        self.find_node_response == other.find_node_response &&
        self.ping_query == other.ping_query &&
        self.ping_response == other.ping_response &&
        self.packet_query == other.packet_query &&
        self.packet_response == other.packet_response &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Envelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Envelope_Type {
    FIND_NODE_QUERY = 1,
    FIND_NODE_RESPONSE = 2,
    PING_QUERY = 3,
    PING_RESPONSE = 4,
    PACKET_QUERY = 5,
    PACKET_RESPONSE = 6,
}

impl ::protobuf::ProtobufEnum for Envelope_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Envelope_Type> {
        match value {
            1 => ::std::option::Option::Some(Envelope_Type::FIND_NODE_QUERY),
            2 => ::std::option::Option::Some(Envelope_Type::FIND_NODE_RESPONSE),
            3 => ::std::option::Option::Some(Envelope_Type::PING_QUERY),
            4 => ::std::option::Option::Some(Envelope_Type::PING_RESPONSE),
            5 => ::std::option::Option::Some(Envelope_Type::PACKET_QUERY),
            6 => ::std::option::Option::Some(Envelope_Type::PACKET_RESPONSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Envelope_Type] = &[
            Envelope_Type::FIND_NODE_QUERY,
            Envelope_Type::FIND_NODE_RESPONSE,
            Envelope_Type::PING_QUERY,
            Envelope_Type::PING_RESPONSE,
            Envelope_Type::PACKET_QUERY,
            Envelope_Type::PACKET_RESPONSE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Envelope_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Envelope_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Envelope_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x73, 0x72, 0x63, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x41,
    0x0a, 0x0c, 0x55, 0x64, 0x70, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x1d,
    0x0a, 0x0a, 0x69, 0x70, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0c, 0x52, 0x09, 0x69, 0x70, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x12, 0x0a,
    0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72,
    0x74, 0x22, 0x88, 0x01, 0x0a, 0x09, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x12,
    0x36, 0x0a, 0x0e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70,
    0x6f, 0x72, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70,
    0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x32, 0x0a, 0x0d, 0x75, 0x64, 0x70, 0x5f, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x55, 0x64, 0x70, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x52, 0x0c, 0x75,
    0x64, 0x70, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x22, 0x0f, 0x0a, 0x04, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x55, 0x44, 0x50, 0x10, 0x01, 0x22, 0x42, 0x0a, 0x04,
    0x4e, 0x6f, 0x64, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x02, 0x69, 0x64, 0x12, 0x2a, 0x0a, 0x0a, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72,
    0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x70, 0x6f, 0x72, 0x74, 0x52, 0x0a, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x73,
    0x22, 0x46, 0x0a, 0x0d, 0x46, 0x69, 0x6e, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x51, 0x75, 0x65, 0x72,
    0x79, 0x12, 0x1d, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x12, 0x16, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x22, 0x4e, 0x0a, 0x10, 0x46, 0x69, 0x6e, 0x64,
    0x4e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x06,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e,
    0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x1b, 0x0a, 0x05, 0x6e,
    0x6f, 0x64, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64,
    0x65, 0x52, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x2a, 0x0a, 0x09, 0x50, 0x69, 0x6e, 0x67,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x1d, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x22, 0x2d, 0x0a, 0x0c, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x22, 0x46, 0x0a, 0x0b, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x51, 0x75, 0x65,
    0x72, 0x79, 0x12, 0x1d, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0c, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x2f, 0x0a, 0x0e, 0x50,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a,
    0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x05, 0x2e,
    0x4e, 0x6f, 0x64, 0x65, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x22, 0xa6, 0x04, 0x0a,
    0x08, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x12, 0x31, 0x0a, 0x0c, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x0e, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x25, 0x0a, 0x0e,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0d, 0x52, 0x0d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x12, 0x36, 0x0a, 0x0f, 0x66, 0x69, 0x6e, 0x64, 0x5f, 0x6e, 0x6f, 0x64, 0x65,
    0x5f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x46,
    0x69, 0x6e, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x0d, 0x66, 0x69,
    0x6e, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x3f, 0x0a, 0x12, 0x66,
    0x69, 0x6e, 0x64, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x46, 0x69, 0x6e, 0x64, 0x4e, 0x6f,
    0x64, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x66, 0x69, 0x6e, 0x64,
    0x4e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x0a,
    0x70, 0x69, 0x6e, 0x67, 0x5f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0a, 0x2e, 0x50, 0x69, 0x6e, 0x67, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x09, 0x70, 0x69,
    0x6e, 0x67, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x32, 0x0a, 0x0d, 0x70, 0x69, 0x6e, 0x67, 0x5f,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0c, 0x70,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x0c, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52,
    0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x38, 0x0a, 0x0f,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x7d, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x13,
    0x0a, 0x0f, 0x46, 0x49, 0x4e, 0x44, 0x5f, 0x4e, 0x4f, 0x44, 0x45, 0x5f, 0x51, 0x55, 0x45, 0x52,
    0x59, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x46, 0x49, 0x4e, 0x44, 0x5f, 0x4e, 0x4f, 0x44, 0x45,
    0x5f, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x10, 0x02, 0x12, 0x0e, 0x0a, 0x0a, 0x50,
    0x49, 0x4e, 0x47, 0x5f, 0x51, 0x55, 0x45, 0x52, 0x59, 0x10, 0x03, 0x12, 0x11, 0x0a, 0x0d, 0x50,
    0x49, 0x4e, 0x47, 0x5f, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x10, 0x04, 0x12, 0x10,
    0x0a, 0x0c, 0x50, 0x41, 0x43, 0x4b, 0x45, 0x54, 0x5f, 0x51, 0x55, 0x45, 0x52, 0x59, 0x10, 0x05,
    0x12, 0x13, 0x0a, 0x0f, 0x50, 0x41, 0x43, 0x4b, 0x45, 0x54, 0x5f, 0x52, 0x45, 0x53, 0x50, 0x4f,
    0x4e, 0x53, 0x45, 0x10, 0x06, 0x4a, 0xb3, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x02, 0x00, 0x46, 0x01,
    0x0a, 0x13, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x32, 0x07, 0x20, 0x54,
    0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x03, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x04, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x14, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x07, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x07, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x08, 0x04, 0x0a,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x09, 0x0d, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x09, 0x0e, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x0f, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x04, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x10, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x10, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11,
    0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x11, 0x0d, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x24, 0x25, 0x0a, 0x16, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x32, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16, 0x08, 0x15, 0x0a, 0x1c,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17, 0x04, 0x1d, 0x22, 0x0f, 0x20, 0x51, 0x75,
    0x65, 0x72, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x17, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x17, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x17, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x18, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08,
    0x18, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x04, 0x1d, 0x22, 0x11,
    0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x6f, 0x64, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1d,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x20, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x20, 0x08, 0x11, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x21, 0x04, 0x1d, 0x22, 0x0f, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x6e,
    0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x24, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x24, 0x08, 0x14, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x25, 0x04,
    0x1d, 0x22, 0x11, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6e,
    0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x28, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x29, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x01, 0x12, 0x03, 0x2a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x13, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x2e, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x1b, 0x1c, 0x0a, 0x15, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x33, 0x00, 0x46, 0x01, 0x32, 0x09, 0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65,
    0x72, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x09, 0x04, 0x00, 0x12, 0x04, 0x34, 0x04, 0x3b, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x04, 0x00, 0x01, 0x12, 0x03, 0x34, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x35, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x08, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x35, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x36, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x37, 0x08, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x37, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x37, 0x15, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x38, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x38, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x38, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x39, 0x08, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x39, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x39, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x3a, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x3a, 0x08, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x3a, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x04,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x01, 0x12, 0x03, 0x3d, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x14, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x25, 0x26, 0x0a, 0x2a,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x40, 0x04, 0x2f, 0x1a, 0x1d, 0x20, 0x52, 0x65,
    0x70, 0x6c, 0x61, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x41, 0x6e, 0x79, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x40, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x40, 0x1b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40,
    0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x41, 0x04, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x03, 0x41, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x41, 0x1e, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x41, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12,
    0x03, 0x42, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x03, 0x42,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x06, 0x12, 0x03, 0x42, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x17, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x03, 0x42, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x05, 0x12, 0x03, 0x43, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x43, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x43, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x03, 0x12, 0x03, 0x43, 0x2a,
    0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x06, 0x12, 0x03, 0x44, 0x04, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x06, 0x06, 0x12, 0x03, 0x44, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x44, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x44, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x07, 0x12, 0x03,
    0x45, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x04, 0x12, 0x03, 0x45, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x06, 0x12, 0x03, 0x45, 0x0d, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x01, 0x12, 0x03, 0x45, 0x1c, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x07, 0x03, 0x12, 0x03, 0x45, 0x2e, 0x2f,
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
