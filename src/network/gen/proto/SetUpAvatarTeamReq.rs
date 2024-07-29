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

//! Generated file from `SetUpAvatarTeamReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetUpAvatarTeamReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetUpAvatarTeamReq {
    // message fields
    // @@protoc_insertion_point(field:SetUpAvatarTeamReq.avatar_team_guid_list)
    pub avatar_team_guid_list: ::std::vec::Vec<u64>,
    // @@protoc_insertion_point(field:SetUpAvatarTeamReq.team_id)
    pub team_id: u32,
    // @@protoc_insertion_point(field:SetUpAvatarTeamReq.cur_avatar_guid)
    pub cur_avatar_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:SetUpAvatarTeamReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetUpAvatarTeamReq {
    fn default() -> &'a SetUpAvatarTeamReq {
        <SetUpAvatarTeamReq as ::protobuf::Message>::default_instance()
    }
}

impl SetUpAvatarTeamReq {
    pub fn new() -> SetUpAvatarTeamReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_team_guid_list",
            |m: &SetUpAvatarTeamReq| { &m.avatar_team_guid_list },
            |m: &mut SetUpAvatarTeamReq| { &mut m.avatar_team_guid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "team_id",
            |m: &SetUpAvatarTeamReq| { &m.team_id },
            |m: &mut SetUpAvatarTeamReq| { &mut m.team_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_avatar_guid",
            |m: &SetUpAvatarTeamReq| { &m.cur_avatar_guid },
            |m: &mut SetUpAvatarTeamReq| { &mut m.cur_avatar_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetUpAvatarTeamReq>(
            "SetUpAvatarTeamReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetUpAvatarTeamReq {
    const NAME: &'static str = "SetUpAvatarTeamReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    is.read_repeated_packed_uint64_into(&mut self.avatar_team_guid_list)?;
                },
                48 => {
                    self.avatar_team_guid_list.push(is.read_uint64()?);
                },
                64 => {
                    self.team_id = is.read_uint32()?;
                },
                8 => {
                    self.cur_avatar_guid = is.read_uint64()?;
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
        for value in &self.avatar_team_guid_list {
            my_size += ::protobuf::rt::uint64_size(6, *value);
        };
        if self.team_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.team_id);
        }
        if self.cur_avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.cur_avatar_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.avatar_team_guid_list {
            os.write_uint64(6, *v)?;
        };
        if self.team_id != 0 {
            os.write_uint32(8, self.team_id)?;
        }
        if self.cur_avatar_guid != 0 {
            os.write_uint64(1, self.cur_avatar_guid)?;
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

    fn new() -> SetUpAvatarTeamReq {
        SetUpAvatarTeamReq::new()
    }

    fn clear(&mut self) {
        self.avatar_team_guid_list.clear();
        self.team_id = 0;
        self.cur_avatar_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetUpAvatarTeamReq {
        static instance: SetUpAvatarTeamReq = SetUpAvatarTeamReq {
            avatar_team_guid_list: ::std::vec::Vec::new(),
            team_id: 0,
            cur_avatar_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetUpAvatarTeamReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetUpAvatarTeamReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetUpAvatarTeamReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetUpAvatarTeamReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18SetUpAvatarTeamReq.proto\"\x88\x01\n\x12SetUpAvatarTeamReq\x121\n\
    \x15avatar_team_guid_list\x18\x06\x20\x03(\x04R\x12avatarTeamGuidList\
    \x12\x17\n\x07team_id\x18\x08\x20\x01(\rR\x06teamId\x12&\n\x0fcur_avatar\
    _guid\x18\x01\x20\x01(\x04R\rcurAvatarGuidB\x1b\n\x19emu.grasscutter.net\
    .protob\x06proto3\
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
            messages.push(SetUpAvatarTeamReq::generated_message_descriptor_data());
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