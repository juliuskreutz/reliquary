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

//! Generated file from `TeamEnterSceneInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TeamEnterSceneInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TeamEnterSceneInfo {
    // message fields
    // @@protoc_insertion_point(field:TeamEnterSceneInfo.ability_control_block)
    pub ability_control_block: ::protobuf::MessageField<super::AbilityControlBlock::AbilityControlBlock>,
    // @@protoc_insertion_point(field:TeamEnterSceneInfo.team_ability_info)
    pub team_ability_info: ::protobuf::MessageField<super::AbilitySyncStateInfo::AbilitySyncStateInfo>,
    // @@protoc_insertion_point(field:TeamEnterSceneInfo.team_entity_id)
    pub team_entity_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TeamEnterSceneInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TeamEnterSceneInfo {
    fn default() -> &'a TeamEnterSceneInfo {
        <TeamEnterSceneInfo as ::protobuf::Message>::default_instance()
    }
}

impl TeamEnterSceneInfo {
    pub fn new() -> TeamEnterSceneInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityControlBlock::AbilityControlBlock>(
            "ability_control_block",
            |m: &TeamEnterSceneInfo| { &m.ability_control_block },
            |m: &mut TeamEnterSceneInfo| { &mut m.ability_control_block },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilitySyncStateInfo::AbilitySyncStateInfo>(
            "team_ability_info",
            |m: &TeamEnterSceneInfo| { &m.team_ability_info },
            |m: &mut TeamEnterSceneInfo| { &mut m.team_ability_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "team_entity_id",
            |m: &TeamEnterSceneInfo| { &m.team_entity_id },
            |m: &mut TeamEnterSceneInfo| { &mut m.team_entity_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TeamEnterSceneInfo>(
            "TeamEnterSceneInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TeamEnterSceneInfo {
    const NAME: &'static str = "TeamEnterSceneInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ability_control_block)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.team_ability_info)?;
                },
                24 => {
                    self.team_entity_id = is.read_uint32()?;
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
        if let Some(v) = self.ability_control_block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.team_ability_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.team_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.team_entity_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ability_control_block.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.team_ability_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.team_entity_id != 0 {
            os.write_uint32(3, self.team_entity_id)?;
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

    fn new() -> TeamEnterSceneInfo {
        TeamEnterSceneInfo::new()
    }

    fn clear(&mut self) {
        self.ability_control_block.clear();
        self.team_ability_info.clear();
        self.team_entity_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TeamEnterSceneInfo {
        static instance: TeamEnterSceneInfo = TeamEnterSceneInfo {
            ability_control_block: ::protobuf::MessageField::none(),
            team_ability_info: ::protobuf::MessageField::none(),
            team_entity_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TeamEnterSceneInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TeamEnterSceneInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TeamEnterSceneInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TeamEnterSceneInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18TeamEnterSceneInfo.proto\x1a\x19AbilityControlBlock.proto\x1a\x1aA\
    bilitySyncStateInfo.proto\"\xc7\x01\n\x12TeamEnterSceneInfo\x12H\n\x15ab\
    ility_control_block\x18\x0b\x20\x01(\x0b2\x14.AbilityControlBlockR\x13ab\
    ilityControlBlock\x12A\n\x11team_ability_info\x18\x04\x20\x01(\x0b2\x15.\
    AbilitySyncStateInfoR\x0fteamAbilityInfo\x12$\n\x0eteam_entity_id\x18\
    \x03\x20\x01(\rR\x0cteamEntityIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AbilityControlBlock::file_descriptor().clone());
            deps.push(super::AbilitySyncStateInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TeamEnterSceneInfo::generated_message_descriptor_data());
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