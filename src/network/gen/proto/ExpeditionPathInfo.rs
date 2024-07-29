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

//! Generated file from `ExpeditionPathInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ExpeditionPathInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExpeditionPathInfo {
    // message fields
    // @@protoc_insertion_point(field:ExpeditionPathInfo.path_id)
    pub path_id: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.bonus_probability)
    pub bonus_probability: f32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.start_time)
    pub start_time: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.AGGGPFNPNLN)
    pub AGGGPFNPNLN: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.avatar_id_list)
    pub avatar_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.state)
    pub state: ::protobuf::EnumOrUnknown<super::ExpeditionState::ExpeditionState>,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.DDCIILLJCLH)
    pub DDCIILLJCLH: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.EPLFCNHCJOD)
    pub EPLFCNHCJOD: u32,
    // @@protoc_insertion_point(field:ExpeditionPathInfo.OFNGLPBLGGP)
    pub OFNGLPBLGGP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ExpeditionPathInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExpeditionPathInfo {
    fn default() -> &'a ExpeditionPathInfo {
        <ExpeditionPathInfo as ::protobuf::Message>::default_instance()
    }
}

impl ExpeditionPathInfo {
    pub fn new() -> ExpeditionPathInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "path_id",
            |m: &ExpeditionPathInfo| { &m.path_id },
            |m: &mut ExpeditionPathInfo| { &mut m.path_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bonus_probability",
            |m: &ExpeditionPathInfo| { &m.bonus_probability },
            |m: &mut ExpeditionPathInfo| { &mut m.bonus_probability },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start_time",
            |m: &ExpeditionPathInfo| { &m.start_time },
            |m: &mut ExpeditionPathInfo| { &mut m.start_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGGGPFNPNLN",
            |m: &ExpeditionPathInfo| { &m.AGGGPFNPNLN },
            |m: &mut ExpeditionPathInfo| { &mut m.AGGGPFNPNLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_id_list",
            |m: &ExpeditionPathInfo| { &m.avatar_id_list },
            |m: &mut ExpeditionPathInfo| { &mut m.avatar_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &ExpeditionPathInfo| { &m.state },
            |m: &mut ExpeditionPathInfo| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDCIILLJCLH",
            |m: &ExpeditionPathInfo| { &m.DDCIILLJCLH },
            |m: &mut ExpeditionPathInfo| { &mut m.DDCIILLJCLH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &ExpeditionPathInfo| { &m.challenge_id },
            |m: &mut ExpeditionPathInfo| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPLFCNHCJOD",
            |m: &ExpeditionPathInfo| { &m.EPLFCNHCJOD },
            |m: &mut ExpeditionPathInfo| { &mut m.EPLFCNHCJOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFNGLPBLGGP",
            |m: &ExpeditionPathInfo| { &m.OFNGLPBLGGP },
            |m: &mut ExpeditionPathInfo| { &mut m.OFNGLPBLGGP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExpeditionPathInfo>(
            "ExpeditionPathInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExpeditionPathInfo {
    const NAME: &'static str = "ExpeditionPathInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.path_id = is.read_uint32()?;
                },
                77 => {
                    self.bonus_probability = is.read_float()?;
                },
                40 => {
                    self.start_time = is.read_uint32()?;
                },
                48 => {
                    self.AGGGPFNPNLN = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_id_list)?;
                },
                120 => {
                    self.avatar_id_list.push(is.read_uint32()?);
                },
                8 => {
                    self.state = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.DDCIILLJCLH = is.read_uint32()?;
                },
                104 => {
                    self.challenge_id = is.read_uint32()?;
                },
                112 => {
                    self.EPLFCNHCJOD = is.read_uint32()?;
                },
                88 => {
                    self.OFNGLPBLGGP = is.read_uint32()?;
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
        if self.path_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.path_id);
        }
        if self.bonus_probability != 0. {
            my_size += 1 + 4;
        }
        if self.start_time != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.start_time);
        }
        if self.AGGGPFNPNLN != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.AGGGPFNPNLN);
        }
        for value in &self.avatar_id_list {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.state != ::protobuf::EnumOrUnknown::new(super::ExpeditionState::ExpeditionState::EXPEDITION_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.state.value());
        }
        if self.DDCIILLJCLH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DDCIILLJCLH);
        }
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.challenge_id);
        }
        if self.EPLFCNHCJOD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.EPLFCNHCJOD);
        }
        if self.OFNGLPBLGGP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.OFNGLPBLGGP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.path_id != 0 {
            os.write_uint32(7, self.path_id)?;
        }
        if self.bonus_probability != 0. {
            os.write_float(9, self.bonus_probability)?;
        }
        if self.start_time != 0 {
            os.write_uint32(5, self.start_time)?;
        }
        if self.AGGGPFNPNLN != 0 {
            os.write_uint32(6, self.AGGGPFNPNLN)?;
        }
        for v in &self.avatar_id_list {
            os.write_uint32(15, *v)?;
        };
        if self.state != ::protobuf::EnumOrUnknown::new(super::ExpeditionState::ExpeditionState::EXPEDITION_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.state))?;
        }
        if self.DDCIILLJCLH != 0 {
            os.write_uint32(4, self.DDCIILLJCLH)?;
        }
        if self.challenge_id != 0 {
            os.write_uint32(13, self.challenge_id)?;
        }
        if self.EPLFCNHCJOD != 0 {
            os.write_uint32(14, self.EPLFCNHCJOD)?;
        }
        if self.OFNGLPBLGGP != 0 {
            os.write_uint32(11, self.OFNGLPBLGGP)?;
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

    fn new() -> ExpeditionPathInfo {
        ExpeditionPathInfo::new()
    }

    fn clear(&mut self) {
        self.path_id = 0;
        self.bonus_probability = 0.;
        self.start_time = 0;
        self.AGGGPFNPNLN = 0;
        self.avatar_id_list.clear();
        self.state = ::protobuf::EnumOrUnknown::new(super::ExpeditionState::ExpeditionState::EXPEDITION_NONE);
        self.DDCIILLJCLH = 0;
        self.challenge_id = 0;
        self.EPLFCNHCJOD = 0;
        self.OFNGLPBLGGP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExpeditionPathInfo {
        static instance: ExpeditionPathInfo = ExpeditionPathInfo {
            path_id: 0,
            bonus_probability: 0.,
            start_time: 0,
            AGGGPFNPNLN: 0,
            avatar_id_list: ::std::vec::Vec::new(),
            state: ::protobuf::EnumOrUnknown::from_i32(0),
            DDCIILLJCLH: 0,
            challenge_id: 0,
            EPLFCNHCJOD: 0,
            OFNGLPBLGGP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExpeditionPathInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExpeditionPathInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExpeditionPathInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExpeditionPathInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ExpeditionPathInfo.proto\x1a\x15ExpeditionState.proto\"\xf2\x02\n\
    \x12ExpeditionPathInfo\x12\x17\n\x07path_id\x18\x07\x20\x01(\rR\x06pathI\
    d\x12+\n\x11bonus_probability\x18\t\x20\x01(\x02R\x10bonusProbability\
    \x12\x1d\n\nstart_time\x18\x05\x20\x01(\rR\tstartTime\x12\x20\n\x0bAGGGP\
    FNPNLN\x18\x06\x20\x01(\rR\x0bAGGGPFNPNLN\x12$\n\x0eavatar_id_list\x18\
    \x0f\x20\x03(\rR\x0cavatarIdList\x12&\n\x05state\x18\x01\x20\x01(\x0e2\
    \x10.ExpeditionStateR\x05state\x12\x20\n\x0bDDCIILLJCLH\x18\x04\x20\x01(\
    \rR\x0bDDCIILLJCLH\x12!\n\x0cchallenge_id\x18\r\x20\x01(\rR\x0bchallenge\
    Id\x12\x20\n\x0bEPLFCNHCJOD\x18\x0e\x20\x01(\rR\x0bEPLFCNHCJOD\x12\x20\n\
    \x0bOFNGLPBLGGP\x18\x0b\x20\x01(\rR\x0bOFNGLPBLGGPB\x1b\n\x19emu.grasscu\
    tter.net.protob\x06proto3\
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
            deps.push(super::ExpeditionState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ExpeditionPathInfo::generated_message_descriptor_data());
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