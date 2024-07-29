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

//! Generated file from `ForgeGetQueueDataRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ForgeGetQueueDataRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ForgeGetQueueDataRsp {
    // message fields
    // @@protoc_insertion_point(field:ForgeGetQueueDataRsp.max_queue_num)
    pub max_queue_num: u32,
    // @@protoc_insertion_point(field:ForgeGetQueueDataRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:ForgeGetQueueDataRsp.forge_queue_map)
    pub forge_queue_map: ::std::collections::HashMap<u32, super::ForgeQueueData::ForgeQueueData>,
    // special fields
    // @@protoc_insertion_point(special_field:ForgeGetQueueDataRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ForgeGetQueueDataRsp {
    fn default() -> &'a ForgeGetQueueDataRsp {
        <ForgeGetQueueDataRsp as ::protobuf::Message>::default_instance()
    }
}

impl ForgeGetQueueDataRsp {
    pub fn new() -> ForgeGetQueueDataRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_queue_num",
            |m: &ForgeGetQueueDataRsp| { &m.max_queue_num },
            |m: &mut ForgeGetQueueDataRsp| { &mut m.max_queue_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ForgeGetQueueDataRsp| { &m.retcode },
            |m: &mut ForgeGetQueueDataRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "forge_queue_map",
            |m: &ForgeGetQueueDataRsp| { &m.forge_queue_map },
            |m: &mut ForgeGetQueueDataRsp| { &mut m.forge_queue_map },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ForgeGetQueueDataRsp>(
            "ForgeGetQueueDataRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ForgeGetQueueDataRsp {
    const NAME: &'static str = "ForgeGetQueueDataRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.max_queue_num = is.read_uint32()?;
                },
                88 => {
                    self.retcode = is.read_int32()?;
                },
                98 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.forge_queue_map.insert(key, value);
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
        if self.max_queue_num != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.max_queue_num);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.retcode);
        }
        for (k, v) in &self.forge_queue_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.max_queue_num != 0 {
            os.write_uint32(3, self.max_queue_num)?;
        }
        if self.retcode != 0 {
            os.write_int32(11, self.retcode)?;
        }
        for (k, v) in &self.forge_queue_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(98)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> ForgeGetQueueDataRsp {
        ForgeGetQueueDataRsp::new()
    }

    fn clear(&mut self) {
        self.max_queue_num = 0;
        self.retcode = 0;
        self.forge_queue_map.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ForgeGetQueueDataRsp {
        static instance: ::protobuf::rt::Lazy<ForgeGetQueueDataRsp> = ::protobuf::rt::Lazy::new();
        instance.get(ForgeGetQueueDataRsp::new)
    }
}

impl ::protobuf::MessageFull for ForgeGetQueueDataRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ForgeGetQueueDataRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ForgeGetQueueDataRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ForgeGetQueueDataRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aForgeGetQueueDataRsp.proto\x1a\x14ForgeQueueData.proto\"\xf9\x01\n\
    \x14ForgeGetQueueDataRsp\x12\"\n\rmax_queue_num\x18\x03\x20\x01(\rR\x0bm\
    axQueueNum\x12\x18\n\x07retcode\x18\x0b\x20\x01(\x05R\x07retcode\x12P\n\
    \x0fforge_queue_map\x18\x0c\x20\x03(\x0b2(.ForgeGetQueueDataRsp.ForgeQue\
    ueMapEntryR\rforgeQueueMap\x1aQ\n\x12ForgeQueueMapEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\rR\x03key\x12%\n\x05value\x18\x02\x20\x01(\x0b2\x0f.Fo\
    rgeQueueDataR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::ForgeQueueData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ForgeGetQueueDataRsp::generated_message_descriptor_data());
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
