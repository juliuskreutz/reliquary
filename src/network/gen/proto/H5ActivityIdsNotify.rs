// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `H5ActivityIdsNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:H5ActivityIdsNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct H5ActivityIdsNotify {
    // message fields
    // @@protoc_insertion_point(field:H5ActivityIdsNotify.client_red_dot_timestamp)
    pub client_red_dot_timestamp: u32,
    // @@protoc_insertion_point(field:H5ActivityIdsNotify.h5_activity_map)
    pub h5_activity_map: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:H5ActivityIdsNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a H5ActivityIdsNotify {
    fn default() -> &'a H5ActivityIdsNotify {
        <H5ActivityIdsNotify as ::protobuf::Message>::default_instance()
    }
}

impl H5ActivityIdsNotify {
    pub fn new() -> H5ActivityIdsNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_red_dot_timestamp",
            |m: &H5ActivityIdsNotify| { &m.client_red_dot_timestamp },
            |m: &mut H5ActivityIdsNotify| { &mut m.client_red_dot_timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "h5_activity_map",
            |m: &H5ActivityIdsNotify| { &m.h5_activity_map },
            |m: &mut H5ActivityIdsNotify| { &mut m.h5_activity_map },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<H5ActivityIdsNotify>(
            "H5ActivityIdsNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for H5ActivityIdsNotify {
    const NAME: &'static str = "H5ActivityIdsNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.client_red_dot_timestamp = is.read_uint32()?;
                },
                98 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.h5_activity_map.insert(key, value);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.client_red_dot_timestamp != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.client_red_dot_timestamp);
        }
        for (k, v) in &self.h5_activity_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.client_red_dot_timestamp != 0 {
            os.write_uint32(14, self.client_red_dot_timestamp)?;
        }
        for (k, v) in &self.h5_activity_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(98)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> H5ActivityIdsNotify {
        H5ActivityIdsNotify::new()
    }

    fn clear(&mut self) {
        self.client_red_dot_timestamp = 0;
        self.h5_activity_map.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static H5ActivityIdsNotify {
        static instance: ::protobuf::rt::Lazy<H5ActivityIdsNotify> = ::protobuf::rt::Lazy::new();
        instance.get(H5ActivityIdsNotify::new)
    }
}

impl ::protobuf::MessageFull for H5ActivityIdsNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("H5ActivityIdsNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for H5ActivityIdsNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for H5ActivityIdsNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19H5ActivityIdsNotify.proto\"\xe1\x01\n\x13H5ActivityIdsNotify\x127\
    \n\x18client_red_dot_timestamp\x18\x0e\x20\x01(\rR\x15clientRedDotTimest\
    amp\x12O\n\x0fh5_activity_map\x18\x0c\x20\x03(\x0b2'.H5ActivityIdsNotify\
    .H5ActivityMapEntryR\rh5ActivityMap\x1a@\n\x12H5ActivityMapEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \rR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(H5ActivityIdsNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}