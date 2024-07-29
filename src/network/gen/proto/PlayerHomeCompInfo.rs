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

//! Generated file from `PlayerHomeCompInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerHomeCompInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerHomeCompInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerHomeCompInfo.friend_enter_home_option)
    pub friend_enter_home_option: ::protobuf::EnumOrUnknown<super::FriendEnterHomeOption::FriendEnterHomeOption>,
    // @@protoc_insertion_point(field:PlayerHomeCompInfo.levelup_reward_got_level_list)
    pub levelup_reward_got_level_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlayerHomeCompInfo.seen_module_id_list)
    pub seen_module_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlayerHomeCompInfo.unlocked_module_id_list)
    pub unlocked_module_id_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerHomeCompInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerHomeCompInfo {
    fn default() -> &'a PlayerHomeCompInfo {
        <PlayerHomeCompInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerHomeCompInfo {
    pub fn new() -> PlayerHomeCompInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "friend_enter_home_option",
            |m: &PlayerHomeCompInfo| { &m.friend_enter_home_option },
            |m: &mut PlayerHomeCompInfo| { &mut m.friend_enter_home_option },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "levelup_reward_got_level_list",
            |m: &PlayerHomeCompInfo| { &m.levelup_reward_got_level_list },
            |m: &mut PlayerHomeCompInfo| { &mut m.levelup_reward_got_level_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "seen_module_id_list",
            |m: &PlayerHomeCompInfo| { &m.seen_module_id_list },
            |m: &mut PlayerHomeCompInfo| { &mut m.seen_module_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_module_id_list",
            |m: &PlayerHomeCompInfo| { &m.unlocked_module_id_list },
            |m: &mut PlayerHomeCompInfo| { &mut m.unlocked_module_id_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerHomeCompInfo>(
            "PlayerHomeCompInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerHomeCompInfo {
    const NAME: &'static str = "PlayerHomeCompInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.friend_enter_home_option = is.read_enum_or_unknown()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.levelup_reward_got_level_list)?;
                },
                72 => {
                    self.levelup_reward_got_level_list.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.seen_module_id_list)?;
                },
                104 => {
                    self.seen_module_id_list.push(is.read_uint32()?);
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlocked_module_id_list)?;
                },
                120 => {
                    self.unlocked_module_id_list.push(is.read_uint32()?);
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
        if self.friend_enter_home_option != ::protobuf::EnumOrUnknown::new(super::FriendEnterHomeOption::FriendEnterHomeOption::FRIEND_ENTER_HOME_OPTION_NEED_CONFIRM) {
            my_size += ::protobuf::rt::int32_size(7, self.friend_enter_home_option.value());
        }
        for value in &self.levelup_reward_got_level_list {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.seen_module_id_list {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        for value in &self.unlocked_module_id_list {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.friend_enter_home_option != ::protobuf::EnumOrUnknown::new(super::FriendEnterHomeOption::FriendEnterHomeOption::FRIEND_ENTER_HOME_OPTION_NEED_CONFIRM) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.friend_enter_home_option))?;
        }
        for v in &self.levelup_reward_got_level_list {
            os.write_uint32(9, *v)?;
        };
        for v in &self.seen_module_id_list {
            os.write_uint32(13, *v)?;
        };
        for v in &self.unlocked_module_id_list {
            os.write_uint32(15, *v)?;
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

    fn new() -> PlayerHomeCompInfo {
        PlayerHomeCompInfo::new()
    }

    fn clear(&mut self) {
        self.friend_enter_home_option = ::protobuf::EnumOrUnknown::new(super::FriendEnterHomeOption::FriendEnterHomeOption::FRIEND_ENTER_HOME_OPTION_NEED_CONFIRM);
        self.levelup_reward_got_level_list.clear();
        self.seen_module_id_list.clear();
        self.unlocked_module_id_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerHomeCompInfo {
        static instance: PlayerHomeCompInfo = PlayerHomeCompInfo {
            friend_enter_home_option: ::protobuf::EnumOrUnknown::from_i32(0),
            levelup_reward_got_level_list: ::std::vec::Vec::new(),
            seen_module_id_list: ::std::vec::Vec::new(),
            unlocked_module_id_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerHomeCompInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerHomeCompInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerHomeCompInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerHomeCompInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18PlayerHomeCompInfo.proto\x1a\x1bFriendEnterHomeOption.proto\"\x8d\
    \x02\n\x12PlayerHomeCompInfo\x12O\n\x18friend_enter_home_option\x18\x07\
    \x20\x01(\x0e2\x16.FriendEnterHomeOptionR\x15friendEnterHomeOption\x12@\
    \n\x1dlevelup_reward_got_level_list\x18\t\x20\x03(\rR\x19levelupRewardGo\
    tLevelList\x12-\n\x13seen_module_id_list\x18\r\x20\x03(\rR\x10seenModule\
    IdList\x125\n\x17unlocked_module_id_list\x18\x0f\x20\x03(\rR\x14unlocked\
    ModuleIdListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::FriendEnterHomeOption::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerHomeCompInfo::generated_message_descriptor_data());
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
