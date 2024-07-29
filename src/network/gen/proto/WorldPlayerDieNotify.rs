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

//! Generated file from `WorldPlayerDieNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:WorldPlayerDieNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WorldPlayerDieNotify {
    // message fields
    // @@protoc_insertion_point(field:WorldPlayerDieNotify.die_type)
    pub die_type: ::protobuf::EnumOrUnknown<super::PlayerDieType::PlayerDieType>,
    // @@protoc_insertion_point(field:WorldPlayerDieNotify.murderer_entity_id)
    pub murderer_entity_id: u32,
    // @@protoc_insertion_point(field:WorldPlayerDieNotify.CMFMJLIALCD)
    pub CMFMJLIALCD: u32,
    // message oneof groups
    pub entity: ::std::option::Option<world_player_die_notify::Entity>,
    // special fields
    // @@protoc_insertion_point(special_field:WorldPlayerDieNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WorldPlayerDieNotify {
    fn default() -> &'a WorldPlayerDieNotify {
        <WorldPlayerDieNotify as ::protobuf::Message>::default_instance()
    }
}

impl WorldPlayerDieNotify {
    pub fn new() -> WorldPlayerDieNotify {
        ::std::default::Default::default()
    }

    // uint32 monster_id = 13;

    pub fn monster_id(&self) -> u32 {
        match self.entity {
            ::std::option::Option::Some(world_player_die_notify::Entity::MonsterId(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_monster_id(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_monster_id(&self) -> bool {
        match self.entity {
            ::std::option::Option::Some(world_player_die_notify::Entity::MonsterId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_monster_id(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(world_player_die_notify::Entity::MonsterId(v))
    }

    // uint32 gadget_id = 1;

    pub fn gadget_id(&self) -> u32 {
        match self.entity {
            ::std::option::Option::Some(world_player_die_notify::Entity::GadgetId(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_gadget_id(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_gadget_id(&self) -> bool {
        match self.entity {
            ::std::option::Option::Some(world_player_die_notify::Entity::GadgetId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_gadget_id(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(world_player_die_notify::Entity::GadgetId(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "die_type",
            |m: &WorldPlayerDieNotify| { &m.die_type },
            |m: &mut WorldPlayerDieNotify| { &mut m.die_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "murderer_entity_id",
            |m: &WorldPlayerDieNotify| { &m.murderer_entity_id },
            |m: &mut WorldPlayerDieNotify| { &mut m.murderer_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMFMJLIALCD",
            |m: &WorldPlayerDieNotify| { &m.CMFMJLIALCD },
            |m: &mut WorldPlayerDieNotify| { &mut m.CMFMJLIALCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "monster_id",
            WorldPlayerDieNotify::has_monster_id,
            WorldPlayerDieNotify::monster_id,
            WorldPlayerDieNotify::set_monster_id,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "gadget_id",
            WorldPlayerDieNotify::has_gadget_id,
            WorldPlayerDieNotify::gadget_id,
            WorldPlayerDieNotify::set_gadget_id,
        ));
        oneofs.push(world_player_die_notify::Entity::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WorldPlayerDieNotify>(
            "WorldPlayerDieNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WorldPlayerDieNotify {
    const NAME: &'static str = "WorldPlayerDieNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.die_type = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.murderer_entity_id = is.read_uint32()?;
                },
                120 => {
                    self.CMFMJLIALCD = is.read_uint32()?;
                },
                104 => {
                    self.entity = ::std::option::Option::Some(world_player_die_notify::Entity::MonsterId(is.read_uint32()?));
                },
                8 => {
                    self.entity = ::std::option::Option::Some(world_player_die_notify::Entity::GadgetId(is.read_uint32()?));
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
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.die_type.value());
        }
        if self.murderer_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.murderer_entity_id);
        }
        if self.CMFMJLIALCD != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CMFMJLIALCD);
        }
        if let ::std::option::Option::Some(ref v) = self.entity {
            match v {
                &world_player_die_notify::Entity::MonsterId(v) => {
                    my_size += ::protobuf::rt::uint32_size(13, v);
                },
                &world_player_die_notify::Entity::GadgetId(v) => {
                    my_size += ::protobuf::rt::uint32_size(1, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.die_type))?;
        }
        if self.murderer_entity_id != 0 {
            os.write_uint32(6, self.murderer_entity_id)?;
        }
        if self.CMFMJLIALCD != 0 {
            os.write_uint32(15, self.CMFMJLIALCD)?;
        }
        if let ::std::option::Option::Some(ref v) = self.entity {
            match v {
                &world_player_die_notify::Entity::MonsterId(v) => {
                    os.write_uint32(13, v)?;
                },
                &world_player_die_notify::Entity::GadgetId(v) => {
                    os.write_uint32(1, v)?;
                },
            };
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

    fn new() -> WorldPlayerDieNotify {
        WorldPlayerDieNotify::new()
    }

    fn clear(&mut self) {
        self.die_type = ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE);
        self.murderer_entity_id = 0;
        self.CMFMJLIALCD = 0;
        self.entity = ::std::option::Option::None;
        self.entity = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WorldPlayerDieNotify {
        static instance: WorldPlayerDieNotify = WorldPlayerDieNotify {
            die_type: ::protobuf::EnumOrUnknown::from_i32(0),
            murderer_entity_id: 0,
            CMFMJLIALCD: 0,
            entity: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WorldPlayerDieNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WorldPlayerDieNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WorldPlayerDieNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorldPlayerDieNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `WorldPlayerDieNotify`
pub mod world_player_die_notify {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:WorldPlayerDieNotify.entity)
    pub enum Entity {
        // @@protoc_insertion_point(oneof_field:WorldPlayerDieNotify.monster_id)
        MonsterId(u32),
        // @@protoc_insertion_point(oneof_field:WorldPlayerDieNotify.gadget_id)
        GadgetId(u32),
    }

    impl ::protobuf::Oneof for Entity {
    }

    impl ::protobuf::OneofFull for Entity {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::WorldPlayerDieNotify as ::protobuf::MessageFull>::descriptor().oneof_by_name("entity").unwrap()).clone()
        }
    }

    impl Entity {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Entity>("entity")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aWorldPlayerDieNotify.proto\x1a\x13PlayerDieType.proto\"\xdb\x01\n\
    \x14WorldPlayerDieNotify\x12)\n\x08die_type\x18\x03\x20\x01(\x0e2\x0e.Pl\
    ayerDieTypeR\x07dieType\x12,\n\x12murderer_entity_id\x18\x06\x20\x01(\rR\
    \x10murdererEntityId\x12\x20\n\x0bCMFMJLIALCD\x18\x0f\x20\x01(\rR\x0bCMF\
    MJLIALCD\x12\x1f\n\nmonster_id\x18\r\x20\x01(\rH\0R\tmonsterId\x12\x1d\n\
    \tgadget_id\x18\x01\x20\x01(\rH\0R\x08gadgetIdB\x08\n\x06entityB\x1b\n\
    \x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::PlayerDieType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(WorldPlayerDieNotify::generated_message_descriptor_data());
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