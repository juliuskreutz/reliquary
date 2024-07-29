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

//! Generated file from `PlayerApplyEnterHomeResultNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerApplyEnterHomeResultNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerApplyEnterHomeResultNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerApplyEnterHomeResultNotify.is_agreed)
    pub is_agreed: bool,
    // @@protoc_insertion_point(field:PlayerApplyEnterHomeResultNotify.target_nickname)
    pub target_nickname: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerApplyEnterHomeResultNotify.target_uid)
    pub target_uid: u32,
    // @@protoc_insertion_point(field:PlayerApplyEnterHomeResultNotify.reason)
    pub reason: ::protobuf::EnumOrUnknown<player_apply_enter_home_result_notify::Reason>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerApplyEnterHomeResultNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerApplyEnterHomeResultNotify {
    fn default() -> &'a PlayerApplyEnterHomeResultNotify {
        <PlayerApplyEnterHomeResultNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerApplyEnterHomeResultNotify {
    pub fn new() -> PlayerApplyEnterHomeResultNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_agreed",
            |m: &PlayerApplyEnterHomeResultNotify| { &m.is_agreed },
            |m: &mut PlayerApplyEnterHomeResultNotify| { &mut m.is_agreed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_nickname",
            |m: &PlayerApplyEnterHomeResultNotify| { &m.target_nickname },
            |m: &mut PlayerApplyEnterHomeResultNotify| { &mut m.target_nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_uid",
            |m: &PlayerApplyEnterHomeResultNotify| { &m.target_uid },
            |m: &mut PlayerApplyEnterHomeResultNotify| { &mut m.target_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reason",
            |m: &PlayerApplyEnterHomeResultNotify| { &m.reason },
            |m: &mut PlayerApplyEnterHomeResultNotify| { &mut m.reason },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerApplyEnterHomeResultNotify>(
            "PlayerApplyEnterHomeResultNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerApplyEnterHomeResultNotify {
    const NAME: &'static str = "PlayerApplyEnterHomeResultNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.is_agreed = is.read_bool()?;
                },
                58 => {
                    self.target_nickname = is.read_string()?;
                },
                88 => {
                    self.target_uid = is.read_uint32()?;
                },
                32 => {
                    self.reason = is.read_enum_or_unknown()?;
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
        if self.is_agreed != false {
            my_size += 1 + 1;
        }
        if !self.target_nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.target_nickname);
        }
        if self.target_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.target_uid);
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(player_apply_enter_home_result_notify::Reason::PLAYER_JUDGE) {
            my_size += ::protobuf::rt::int32_size(4, self.reason.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_agreed != false {
            os.write_bool(1, self.is_agreed)?;
        }
        if !self.target_nickname.is_empty() {
            os.write_string(7, &self.target_nickname)?;
        }
        if self.target_uid != 0 {
            os.write_uint32(11, self.target_uid)?;
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(player_apply_enter_home_result_notify::Reason::PLAYER_JUDGE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.reason))?;
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

    fn new() -> PlayerApplyEnterHomeResultNotify {
        PlayerApplyEnterHomeResultNotify::new()
    }

    fn clear(&mut self) {
        self.is_agreed = false;
        self.target_nickname.clear();
        self.target_uid = 0;
        self.reason = ::protobuf::EnumOrUnknown::new(player_apply_enter_home_result_notify::Reason::PLAYER_JUDGE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerApplyEnterHomeResultNotify {
        static instance: PlayerApplyEnterHomeResultNotify = PlayerApplyEnterHomeResultNotify {
            is_agreed: false,
            target_nickname: ::std::string::String::new(),
            target_uid: 0,
            reason: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerApplyEnterHomeResultNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerApplyEnterHomeResultNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerApplyEnterHomeResultNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerApplyEnterHomeResultNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PlayerApplyEnterHomeResultNotify`
pub mod player_apply_enter_home_result_notify {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:PlayerApplyEnterHomeResultNotify.Reason)
    pub enum Reason {
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.PLAYER_JUDGE)
        PLAYER_JUDGE = 0,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.PLAYER_ENTER_OPTION_REFUSE)
        PLAYER_ENTER_OPTION_REFUSE = 1,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.PLAYER_ENTER_OPTION_DIRECT)
        PLAYER_ENTER_OPTION_DIRECT = 2,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.SYSTEM_JUDGE)
        SYSTEM_JUDGE = 3,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.HOST_IN_MATCH)
        HOST_IN_MATCH = 4,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.PS_PLAYER_NOT_ACCEPT_OTHERS)
        PS_PLAYER_NOT_ACCEPT_OTHERS = 5,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.OPEN_STATE_NOT_OPEN)
        OPEN_STATE_NOT_OPEN = 6,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.HOST_IN_EDIT_MODE)
        HOST_IN_EDIT_MODE = 7,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterHomeResultNotify.Reason.PRIOR_CHECK)
        PRIOR_CHECK = 8,
    }

    impl ::protobuf::Enum for Reason {
        const NAME: &'static str = "Reason";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Reason> {
            match value {
                0 => ::std::option::Option::Some(Reason::PLAYER_JUDGE),
                1 => ::std::option::Option::Some(Reason::PLAYER_ENTER_OPTION_REFUSE),
                2 => ::std::option::Option::Some(Reason::PLAYER_ENTER_OPTION_DIRECT),
                3 => ::std::option::Option::Some(Reason::SYSTEM_JUDGE),
                4 => ::std::option::Option::Some(Reason::HOST_IN_MATCH),
                5 => ::std::option::Option::Some(Reason::PS_PLAYER_NOT_ACCEPT_OTHERS),
                6 => ::std::option::Option::Some(Reason::OPEN_STATE_NOT_OPEN),
                7 => ::std::option::Option::Some(Reason::HOST_IN_EDIT_MODE),
                8 => ::std::option::Option::Some(Reason::PRIOR_CHECK),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Reason> {
            match str {
                "PLAYER_JUDGE" => ::std::option::Option::Some(Reason::PLAYER_JUDGE),
                "PLAYER_ENTER_OPTION_REFUSE" => ::std::option::Option::Some(Reason::PLAYER_ENTER_OPTION_REFUSE),
                "PLAYER_ENTER_OPTION_DIRECT" => ::std::option::Option::Some(Reason::PLAYER_ENTER_OPTION_DIRECT),
                "SYSTEM_JUDGE" => ::std::option::Option::Some(Reason::SYSTEM_JUDGE),
                "HOST_IN_MATCH" => ::std::option::Option::Some(Reason::HOST_IN_MATCH),
                "PS_PLAYER_NOT_ACCEPT_OTHERS" => ::std::option::Option::Some(Reason::PS_PLAYER_NOT_ACCEPT_OTHERS),
                "OPEN_STATE_NOT_OPEN" => ::std::option::Option::Some(Reason::OPEN_STATE_NOT_OPEN),
                "HOST_IN_EDIT_MODE" => ::std::option::Option::Some(Reason::HOST_IN_EDIT_MODE),
                "PRIOR_CHECK" => ::std::option::Option::Some(Reason::PRIOR_CHECK),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Reason] = &[
            Reason::PLAYER_JUDGE,
            Reason::PLAYER_ENTER_OPTION_REFUSE,
            Reason::PLAYER_ENTER_OPTION_DIRECT,
            Reason::SYSTEM_JUDGE,
            Reason::HOST_IN_MATCH,
            Reason::PS_PLAYER_NOT_ACCEPT_OTHERS,
            Reason::OPEN_STATE_NOT_OPEN,
            Reason::HOST_IN_EDIT_MODE,
            Reason::PRIOR_CHECK,
        ];
    }

    impl ::protobuf::EnumFull for Reason {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("PlayerApplyEnterHomeResultNotify.Reason").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Reason {
        fn default() -> Self {
            Reason::PLAYER_JUDGE
        }
    }

    impl Reason {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Reason>("PlayerApplyEnterHomeResultNotify.Reason")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&PlayerApplyEnterHomeResultNotify.proto\"\xad\x03\n\x20PlayerApplyEnte\
    rHomeResultNotify\x12\x1b\n\tis_agreed\x18\x01\x20\x01(\x08R\x08isAgreed\
    \x12'\n\x0ftarget_nickname\x18\x07\x20\x01(\tR\x0etargetNickname\x12\x1d\
    \n\ntarget_uid\x18\x0b\x20\x01(\rR\ttargetUid\x12@\n\x06reason\x18\x04\
    \x20\x01(\x0e2(.PlayerApplyEnterHomeResultNotify.ReasonR\x06reason\"\xe1\
    \x01\n\x06Reason\x12\x10\n\x0cPLAYER_JUDGE\x10\0\x12\x1e\n\x1aPLAYER_ENT\
    ER_OPTION_REFUSE\x10\x01\x12\x1e\n\x1aPLAYER_ENTER_OPTION_DIRECT\x10\x02\
    \x12\x10\n\x0cSYSTEM_JUDGE\x10\x03\x12\x11\n\rHOST_IN_MATCH\x10\x04\x12\
    \x1f\n\x1bPS_PLAYER_NOT_ACCEPT_OTHERS\x10\x05\x12\x17\n\x13OPEN_STATE_NO\
    T_OPEN\x10\x06\x12\x15\n\x11HOST_IN_EDIT_MODE\x10\x07\x12\x0f\n\x0bPRIOR\
    _CHECK\x10\x08B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(PlayerApplyEnterHomeResultNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(player_apply_enter_home_result_notify::Reason::generated_enum_descriptor_data());
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
