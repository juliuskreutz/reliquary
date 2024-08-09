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

//! Generated file from `UseItemCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UseItemCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UseItemCsReq {
    // message fields
    // @@protoc_insertion_point(field:UseItemCsReq.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.use_item_id)
    pub use_item_id: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.use_avatar_type)
    pub use_avatar_type: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:UseItemCsReq.use_item_count)
    pub use_item_count: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.optional_reward_id)
    pub optional_reward_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UseItemCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UseItemCsReq {
    fn default() -> &'a UseItemCsReq {
        <UseItemCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UseItemCsReq {
    pub fn new() -> UseItemCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &UseItemCsReq| { &m.base_avatar_id },
            |m: &mut UseItemCsReq| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_item_id",
            |m: &UseItemCsReq| { &m.use_item_id },
            |m: &mut UseItemCsReq| { &mut m.use_item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_avatar_type",
            |m: &UseItemCsReq| { &m.use_avatar_type },
            |m: &mut UseItemCsReq| { &mut m.use_avatar_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_item_count",
            |m: &UseItemCsReq| { &m.use_item_count },
            |m: &mut UseItemCsReq| { &mut m.use_item_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "optional_reward_id",
            |m: &UseItemCsReq| { &m.optional_reward_id },
            |m: &mut UseItemCsReq| { &mut m.optional_reward_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UseItemCsReq>(
            "UseItemCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UseItemCsReq {
    const NAME: &'static str = "UseItemCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                80 => {
                    self.use_item_id = is.read_uint32()?;
                },
                64 => {
                    self.use_avatar_type = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.use_item_count = is.read_uint32()?;
                },
                40 => {
                    self.optional_reward_id = is.read_uint32()?;
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
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.base_avatar_id);
        }
        if self.use_item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.use_item_id);
        }
        if self.use_avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.use_avatar_type.value());
        }
        if self.use_item_count != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.use_item_count);
        }
        if self.optional_reward_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.optional_reward_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.base_avatar_id != 0 {
            os.write_uint32(11, self.base_avatar_id)?;
        }
        if self.use_item_id != 0 {
            os.write_uint32(10, self.use_item_id)?;
        }
        if self.use_avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.use_avatar_type))?;
        }
        if self.use_item_count != 0 {
            os.write_uint32(15, self.use_item_count)?;
        }
        if self.optional_reward_id != 0 {
            os.write_uint32(5, self.optional_reward_id)?;
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

    fn new() -> UseItemCsReq {
        UseItemCsReq::new()
    }

    fn clear(&mut self) {
        self.base_avatar_id = 0;
        self.use_item_id = 0;
        self.use_avatar_type = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.use_item_count = 0;
        self.optional_reward_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UseItemCsReq {
        static instance: UseItemCsReq = UseItemCsReq {
            base_avatar_id: 0,
            use_item_id: 0,
            use_avatar_type: ::protobuf::EnumOrUnknown::from_i32(0),
            use_item_count: 0,
            optional_reward_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UseItemCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UseItemCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UseItemCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UseItemCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12UseItemCsReq.proto\x1a\x10AvatarType.proto\"\xdd\x01\n\x0cUseItemC\
    sReq\x12$\n\x0ebase_avatar_id\x18\x0b\x20\x01(\rR\x0cbaseAvatarId\x12\
    \x1e\n\x0buse_item_id\x18\n\x20\x01(\rR\tuseItemId\x123\n\x0fuse_avatar_\
    type\x18\x08\x20\x01(\x0e2\x0b.AvatarTypeR\ruseAvatarType\x12$\n\x0euse_\
    item_count\x18\x0f\x20\x01(\rR\x0cuseItemCount\x12,\n\x12optional_reward\
    _id\x18\x05\x20\x01(\rR\x10optionalRewardIdB\x15\n\x13emu.lunarcore.prot\
    ob\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UseItemCsReq::generated_message_descriptor_data());
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