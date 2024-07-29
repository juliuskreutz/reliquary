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

//! Generated file from `SyncTeamEntityNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SyncTeamEntityNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SyncTeamEntityNotify {
    // message fields
    // @@protoc_insertion_point(field:SyncTeamEntityNotify.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:SyncTeamEntityNotify.team_entity_info_list)
    pub team_entity_info_list: ::std::vec::Vec<super::TeamEntityInfo::TeamEntityInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SyncTeamEntityNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SyncTeamEntityNotify {
    fn default() -> &'a SyncTeamEntityNotify {
        <SyncTeamEntityNotify as ::protobuf::Message>::default_instance()
    }
}

impl SyncTeamEntityNotify {
    pub fn new() -> SyncTeamEntityNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &SyncTeamEntityNotify| { &m.scene_id },
            |m: &mut SyncTeamEntityNotify| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "team_entity_info_list",
            |m: &SyncTeamEntityNotify| { &m.team_entity_info_list },
            |m: &mut SyncTeamEntityNotify| { &mut m.team_entity_info_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SyncTeamEntityNotify>(
            "SyncTeamEntityNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SyncTeamEntityNotify {
    const NAME: &'static str = "SyncTeamEntityNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.scene_id = is.read_uint32()?;
                },
                114 => {
                    self.team_entity_info_list.push(is.read_message()?);
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
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.scene_id);
        }
        for value in &self.team_entity_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.scene_id != 0 {
            os.write_uint32(1, self.scene_id)?;
        }
        for v in &self.team_entity_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> SyncTeamEntityNotify {
        SyncTeamEntityNotify::new()
    }

    fn clear(&mut self) {
        self.scene_id = 0;
        self.team_entity_info_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SyncTeamEntityNotify {
        static instance: SyncTeamEntityNotify = SyncTeamEntityNotify {
            scene_id: 0,
            team_entity_info_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SyncTeamEntityNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SyncTeamEntityNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SyncTeamEntityNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncTeamEntityNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSyncTeamEntityNotify.proto\x1a\x14TeamEntityInfo.proto\"u\n\x14Syn\
    cTeamEntityNotify\x12\x19\n\x08scene_id\x18\x01\x20\x01(\rR\x07sceneId\
    \x12B\n\x15team_entity_info_list\x18\x0e\x20\x03(\x0b2\x0f.TeamEntityInf\
    oR\x12teamEntityInfoListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::TeamEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SyncTeamEntityNotify::generated_message_descriptor_data());
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
