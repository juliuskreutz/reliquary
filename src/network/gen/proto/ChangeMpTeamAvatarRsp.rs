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

//! Generated file from `ChangeMpTeamAvatarRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChangeMpTeamAvatarRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChangeMpTeamAvatarRsp {
    // message fields
    // @@protoc_insertion_point(field:ChangeMpTeamAvatarRsp.avatar_guid_list)
    pub avatar_guid_list: ::std::vec::Vec<u64>,
    // @@protoc_insertion_point(field:ChangeMpTeamAvatarRsp.cur_avatar_guid)
    pub cur_avatar_guid: u64,
    // @@protoc_insertion_point(field:ChangeMpTeamAvatarRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:ChangeMpTeamAvatarRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChangeMpTeamAvatarRsp {
    fn default() -> &'a ChangeMpTeamAvatarRsp {
        <ChangeMpTeamAvatarRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChangeMpTeamAvatarRsp {
    pub fn new() -> ChangeMpTeamAvatarRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_guid_list",
            |m: &ChangeMpTeamAvatarRsp| { &m.avatar_guid_list },
            |m: &mut ChangeMpTeamAvatarRsp| { &mut m.avatar_guid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_avatar_guid",
            |m: &ChangeMpTeamAvatarRsp| { &m.cur_avatar_guid },
            |m: &mut ChangeMpTeamAvatarRsp| { &mut m.cur_avatar_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChangeMpTeamAvatarRsp| { &m.retcode },
            |m: &mut ChangeMpTeamAvatarRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChangeMpTeamAvatarRsp>(
            "ChangeMpTeamAvatarRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChangeMpTeamAvatarRsp {
    const NAME: &'static str = "ChangeMpTeamAvatarRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint64_into(&mut self.avatar_guid_list)?;
                },
                80 => {
                    self.avatar_guid_list.push(is.read_uint64()?);
                },
                104 => {
                    self.cur_avatar_guid = is.read_uint64()?;
                },
                72 => {
                    self.retcode = is.read_int32()?;
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
        for value in &self.avatar_guid_list {
            my_size += ::protobuf::rt::uint64_size(10, *value);
        };
        if self.cur_avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(13, self.cur_avatar_guid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.avatar_guid_list {
            os.write_uint64(10, *v)?;
        };
        if self.cur_avatar_guid != 0 {
            os.write_uint64(13, self.cur_avatar_guid)?;
        }
        if self.retcode != 0 {
            os.write_int32(9, self.retcode)?;
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

    fn new() -> ChangeMpTeamAvatarRsp {
        ChangeMpTeamAvatarRsp::new()
    }

    fn clear(&mut self) {
        self.avatar_guid_list.clear();
        self.cur_avatar_guid = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChangeMpTeamAvatarRsp {
        static instance: ChangeMpTeamAvatarRsp = ChangeMpTeamAvatarRsp {
            avatar_guid_list: ::std::vec::Vec::new(),
            cur_avatar_guid: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChangeMpTeamAvatarRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChangeMpTeamAvatarRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChangeMpTeamAvatarRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeMpTeamAvatarRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bChangeMpTeamAvatarRsp.proto\"\x83\x01\n\x15ChangeMpTeamAvatarRsp\
    \x12(\n\x10avatar_guid_list\x18\n\x20\x03(\x04R\x0eavatarGuidList\x12&\n\
    \x0fcur_avatar_guid\x18\r\x20\x01(\x04R\rcurAvatarGuid\x12\x18\n\x07retc\
    ode\x18\t\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.net.protob\
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
            messages.push(ChangeMpTeamAvatarRsp::generated_message_descriptor_data());
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