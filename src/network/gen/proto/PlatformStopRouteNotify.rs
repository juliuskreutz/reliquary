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

//! Generated file from `PlatformStopRouteNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlatformStopRouteNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlatformStopRouteNotify {
    // message fields
    // @@protoc_insertion_point(field:PlatformStopRouteNotify.scene_time)
    pub scene_time: u32,
    // @@protoc_insertion_point(field:PlatformStopRouteNotify.platform)
    pub platform: ::protobuf::MessageField<super::PlatformInfo::PlatformInfo>,
    // @@protoc_insertion_point(field:PlatformStopRouteNotify.entity_id)
    pub entity_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlatformStopRouteNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlatformStopRouteNotify {
    fn default() -> &'a PlatformStopRouteNotify {
        <PlatformStopRouteNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlatformStopRouteNotify {
    pub fn new() -> PlatformStopRouteNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_time",
            |m: &PlatformStopRouteNotify| { &m.scene_time },
            |m: &mut PlatformStopRouteNotify| { &mut m.scene_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PlatformInfo::PlatformInfo>(
            "platform",
            |m: &PlatformStopRouteNotify| { &m.platform },
            |m: &mut PlatformStopRouteNotify| { &mut m.platform },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &PlatformStopRouteNotify| { &m.entity_id },
            |m: &mut PlatformStopRouteNotify| { &mut m.entity_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlatformStopRouteNotify>(
            "PlatformStopRouteNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlatformStopRouteNotify {
    const NAME: &'static str = "PlatformStopRouteNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.scene_time = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.platform)?;
                },
                40 => {
                    self.entity_id = is.read_uint32()?;
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
        if self.scene_time != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.scene_time);
        }
        if let Some(v) = self.platform.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.entity_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.scene_time != 0 {
            os.write_uint32(1, self.scene_time)?;
        }
        if let Some(v) = self.platform.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(5, self.entity_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PlatformStopRouteNotify {
        PlatformStopRouteNotify::new()
    }

    fn clear(&mut self) {
        self.scene_time = 0;
        self.platform.clear();
        self.entity_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlatformStopRouteNotify {
        static instance: PlatformStopRouteNotify = PlatformStopRouteNotify {
            scene_time: 0,
            platform: ::protobuf::MessageField::none(),
            entity_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlatformStopRouteNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlatformStopRouteNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlatformStopRouteNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlatformStopRouteNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dPlatformStopRouteNotify.proto\x1a\x12PlatformInfo.proto\"\x80\x01\
    \n\x17PlatformStopRouteNotify\x12\x1d\n\nscene_time\x18\x01\x20\x01(\rR\
    \tsceneTime\x12)\n\x08platform\x18\x08\x20\x01(\x0b2\r.PlatformInfoR\x08\
    platform\x12\x1b\n\tentity_id\x18\x05\x20\x01(\rR\x08entityIdB\x1b\n\x19\
    emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::PlatformInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlatformStopRouteNotify::generated_message_descriptor_data());
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