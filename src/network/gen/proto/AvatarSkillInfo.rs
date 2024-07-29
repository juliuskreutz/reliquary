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

//! Generated file from `AvatarSkillInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AvatarSkillInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarSkillInfo {
    // message fields
    // @@protoc_insertion_point(field:AvatarSkillInfo.pass_cd_time)
    pub pass_cd_time: u32,
    // @@protoc_insertion_point(field:AvatarSkillInfo.full_cd_time_list)
    pub full_cd_time_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarSkillInfo.max_charge_count)
    pub max_charge_count: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarSkillInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarSkillInfo {
    fn default() -> &'a AvatarSkillInfo {
        <AvatarSkillInfo as ::protobuf::Message>::default_instance()
    }
}

impl AvatarSkillInfo {
    pub fn new() -> AvatarSkillInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pass_cd_time",
            |m: &AvatarSkillInfo| { &m.pass_cd_time },
            |m: &mut AvatarSkillInfo| { &mut m.pass_cd_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "full_cd_time_list",
            |m: &AvatarSkillInfo| { &m.full_cd_time_list },
            |m: &mut AvatarSkillInfo| { &mut m.full_cd_time_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_charge_count",
            |m: &AvatarSkillInfo| { &m.max_charge_count },
            |m: &mut AvatarSkillInfo| { &mut m.max_charge_count },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarSkillInfo>(
            "AvatarSkillInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarSkillInfo {
    const NAME: &'static str = "AvatarSkillInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.pass_cd_time = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.full_cd_time_list)?;
                },
                16 => {
                    self.full_cd_time_list.push(is.read_uint32()?);
                },
                24 => {
                    self.max_charge_count = is.read_uint32()?;
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
        if self.pass_cd_time != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.pass_cd_time);
        }
        for value in &self.full_cd_time_list {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.max_charge_count != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.max_charge_count);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.pass_cd_time != 0 {
            os.write_uint32(1, self.pass_cd_time)?;
        }
        for v in &self.full_cd_time_list {
            os.write_uint32(2, *v)?;
        };
        if self.max_charge_count != 0 {
            os.write_uint32(3, self.max_charge_count)?;
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

    fn new() -> AvatarSkillInfo {
        AvatarSkillInfo::new()
    }

    fn clear(&mut self) {
        self.pass_cd_time = 0;
        self.full_cd_time_list.clear();
        self.max_charge_count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarSkillInfo {
        static instance: AvatarSkillInfo = AvatarSkillInfo {
            pass_cd_time: 0,
            full_cd_time_list: ::std::vec::Vec::new(),
            max_charge_count: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarSkillInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarSkillInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarSkillInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarSkillInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15AvatarSkillInfo.proto\"\x88\x01\n\x0fAvatarSkillInfo\x12\x20\n\x0c\
    pass_cd_time\x18\x01\x20\x01(\rR\npassCdTime\x12)\n\x11full_cd_time_list\
    \x18\x02\x20\x03(\rR\x0efullCdTimeList\x12(\n\x10max_charge_count\x18\
    \x03\x20\x01(\rR\x0emaxChargeCountB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarSkillInfo::generated_message_descriptor_data());
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
