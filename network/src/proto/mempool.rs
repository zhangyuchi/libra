// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `mempool.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(PartialEq,Clone,Default)]
pub struct MempoolSyncMsg {
    // message fields
    pub peer_id: ::bytes::Bytes,
    pub transactions: ::protobuf::RepeatedField<super::transaction::SignedTransaction>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MempoolSyncMsg {
    fn default() -> &'a MempoolSyncMsg {
        <MempoolSyncMsg as ::protobuf::Message>::default_instance()
    }
}

impl MempoolSyncMsg {
    pub fn new() -> MempoolSyncMsg {
        ::std::default::Default::default()
    }

    // bytes peer_id = 1;


    pub fn get_peer_id(&self) -> &[u8] {
        &self.peer_id
    }
    pub fn clear_peer_id(&mut self) {
        self.peer_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_peer_id(&mut self, v: ::bytes::Bytes) {
        self.peer_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_id(&mut self) -> &mut ::bytes::Bytes {
        &mut self.peer_id
    }

    // Take field
    pub fn take_peer_id(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.peer_id, ::bytes::Bytes::new())
    }

    // repeated .types.SignedTransaction transactions = 2;


    pub fn get_transactions(&self) -> &[super::transaction::SignedTransaction] {
        &self.transactions
    }
    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<super::transaction::SignedTransaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<super::transaction::SignedTransaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<super::transaction::SignedTransaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MempoolSyncMsg {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.peer_id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        if !self.peer_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.peer_id);
        }
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.peer_id.is_empty() {
            os.write_bytes(1, &self.peer_id)?;
        }
        for v in &self.transactions {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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
        Self::descriptor_static()
    }

    fn new() -> MempoolSyncMsg {
        MempoolSyncMsg::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "peer_id",
                    |m: &MempoolSyncMsg| { &m.peer_id },
                    |m: &mut MempoolSyncMsg| { &mut m.peer_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::transaction::SignedTransaction>>(
                    "transactions",
                    |m: &MempoolSyncMsg| { &m.transactions },
                    |m: &mut MempoolSyncMsg| { &mut m.transactions },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MempoolSyncMsg>(
                    "MempoolSyncMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static MempoolSyncMsg {
        static mut instance: ::protobuf::lazy::Lazy<MempoolSyncMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MempoolSyncMsg,
        };
        unsafe {
            instance.get(MempoolSyncMsg::new)
        }
    }
}

impl ::protobuf::Clear for MempoolSyncMsg {
    fn clear(&mut self) {
        self.peer_id.clear();
        self.transactions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MempoolSyncMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MempoolSyncMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmempool.proto\x12\x07network\x1a\x11transaction.proto\"g\n\x0eMempoo\
    lSyncMsg\x12\x17\n\x07peer_id\x18\x01\x20\x01(\x0cR\x06peerId\x12<\n\x0c\
    transactions\x18\x02\x20\x03(\x0b2\x18.types.SignedTransactionR\x0ctrans\
    actionsb\x06proto3\
";

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
