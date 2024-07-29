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

//! Generated file from `AvatarLifeStateChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AvatarLifeStateChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarLifeStateChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.server_buff_list)
    pub server_buff_list: ::std::vec::Vec<super::ServerBuff::ServerBuff>,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.die_type)
    pub die_type: ::protobuf::EnumOrUnknown<super::PlayerDieType::PlayerDieType>,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.source_entity_id)
    pub source_entity_id: u32,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.attack_tag)
    pub attack_tag: ::std::string::String,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.avatar_guid)
    pub avatar_guid: u64,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.move_reliable_seq)
    pub move_reliable_seq: u32,
    // @@protoc_insertion_point(field:AvatarLifeStateChangeNotify.life_state)
    pub life_state: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarLifeStateChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarLifeStateChangeNotify {
    fn default() -> &'a AvatarLifeStateChangeNotify {
        <AvatarLifeStateChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl AvatarLifeStateChangeNotify {
    pub fn new() -> AvatarLifeStateChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "server_buff_list",
            |m: &AvatarLifeStateChangeNotify| { &m.server_buff_list },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.server_buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "die_type",
            |m: &AvatarLifeStateChangeNotify| { &m.die_type },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.die_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "source_entity_id",
            |m: &AvatarLifeStateChangeNotify| { &m.source_entity_id },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.source_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attack_tag",
            |m: &AvatarLifeStateChangeNotify| { &m.attack_tag },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.attack_tag },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &AvatarLifeStateChangeNotify| { &m.avatar_guid },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.avatar_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "move_reliable_seq",
            |m: &AvatarLifeStateChangeNotify| { &m.move_reliable_seq },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.move_reliable_seq },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "life_state",
            |m: &AvatarLifeStateChangeNotify| { &m.life_state },
            |m: &mut AvatarLifeStateChangeNotify| { &mut m.life_state },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarLifeStateChangeNotify>(
            "AvatarLifeStateChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarLifeStateChangeNotify {
    const NAME: &'static str = "AvatarLifeStateChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.server_buff_list.push(is.read_message()?);
                },
                24 => {
                    self.die_type = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.source_entity_id = is.read_uint32()?;
                },
                58 => {
                    self.attack_tag = is.read_string()?;
                },
                72 => {
                    self.avatar_guid = is.read_uint64()?;
                },
                80 => {
                    self.move_reliable_seq = is.read_uint32()?;
                },
                120 => {
                    self.life_state = is.read_uint32()?;
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
        for value in &self.server_buff_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.die_type.value());
        }
        if self.source_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.source_entity_id);
        }
        if !self.attack_tag.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.attack_tag);
        }
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.avatar_guid);
        }
        if self.move_reliable_seq != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.move_reliable_seq);
        }
        if self.life_state != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.life_state);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.server_buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.die_type))?;
        }
        if self.source_entity_id != 0 {
            os.write_uint32(6, self.source_entity_id)?;
        }
        if !self.attack_tag.is_empty() {
            os.write_string(7, &self.attack_tag)?;
        }
        if self.avatar_guid != 0 {
            os.write_uint64(9, self.avatar_guid)?;
        }
        if self.move_reliable_seq != 0 {
            os.write_uint32(10, self.move_reliable_seq)?;
        }
        if self.life_state != 0 {
            os.write_uint32(15, self.life_state)?;
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

    fn new() -> AvatarLifeStateChangeNotify {
        AvatarLifeStateChangeNotify::new()
    }

    fn clear(&mut self) {
        self.server_buff_list.clear();
        self.die_type = ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE);
        self.source_entity_id = 0;
        self.attack_tag.clear();
        self.avatar_guid = 0;
        self.move_reliable_seq = 0;
        self.life_state = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarLifeStateChangeNotify {
        static instance: AvatarLifeStateChangeNotify = AvatarLifeStateChangeNotify {
            server_buff_list: ::std::vec::Vec::new(),
            die_type: ::protobuf::EnumOrUnknown::from_i32(0),
            source_entity_id: 0,
            attack_tag: ::std::string::String::new(),
            avatar_guid: 0,
            move_reliable_seq: 0,
            life_state: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarLifeStateChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarLifeStateChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarLifeStateChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarLifeStateChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!AvatarLifeStateChangeNotify.proto\x1a\x10ServerBuff.proto\x1a\x13Play\
    erDieType.proto\"\xb4\x02\n\x1bAvatarLifeStateChangeNotify\x125\n\x10ser\
    ver_buff_list\x18\x02\x20\x03(\x0b2\x0b.ServerBuffR\x0eserverBuffList\
    \x12)\n\x08die_type\x18\x03\x20\x01(\x0e2\x0e.PlayerDieTypeR\x07dieType\
    \x12(\n\x10source_entity_id\x18\x06\x20\x01(\rR\x0esourceEntityId\x12\
    \x1d\n\nattack_tag\x18\x07\x20\x01(\tR\tattackTag\x12\x1f\n\x0bavatar_gu\
    id\x18\t\x20\x01(\x04R\navatarGuid\x12*\n\x11move_reliable_seq\x18\n\x20\
    \x01(\rR\x0fmoveReliableSeq\x12\x1d\n\nlife_state\x18\x0f\x20\x01(\rR\tl\
    ifeStateB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::ServerBuff::file_descriptor().clone());
            deps.push(super::PlayerDieType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarLifeStateChangeNotify::generated_message_descriptor_data());
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