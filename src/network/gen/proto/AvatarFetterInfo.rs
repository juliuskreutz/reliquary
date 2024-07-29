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

//! Generated file from `AvatarFetterInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AvatarFetterInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarFetterInfo {
    // message fields
    // @@protoc_insertion_point(field:AvatarFetterInfo.exp_number)
    pub exp_number: u32,
    // @@protoc_insertion_point(field:AvatarFetterInfo.exp_level)
    pub exp_level: u32,
    // @@protoc_insertion_point(field:AvatarFetterInfo.open_id_list)
    pub open_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarFetterInfo.finish_id_list)
    pub finish_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarFetterInfo.rewarded_fetter_level_list)
    pub rewarded_fetter_level_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarFetterInfo.fetter_list)
    pub fetter_list: ::std::vec::Vec<super::FetterData::FetterData>,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarFetterInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarFetterInfo {
    fn default() -> &'a AvatarFetterInfo {
        <AvatarFetterInfo as ::protobuf::Message>::default_instance()
    }
}

impl AvatarFetterInfo {
    pub fn new() -> AvatarFetterInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp_number",
            |m: &AvatarFetterInfo| { &m.exp_number },
            |m: &mut AvatarFetterInfo| { &mut m.exp_number },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp_level",
            |m: &AvatarFetterInfo| { &m.exp_level },
            |m: &mut AvatarFetterInfo| { &mut m.exp_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "open_id_list",
            |m: &AvatarFetterInfo| { &m.open_id_list },
            |m: &mut AvatarFetterInfo| { &mut m.open_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "finish_id_list",
            |m: &AvatarFetterInfo| { &m.finish_id_list },
            |m: &mut AvatarFetterInfo| { &mut m.finish_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rewarded_fetter_level_list",
            |m: &AvatarFetterInfo| { &m.rewarded_fetter_level_list },
            |m: &mut AvatarFetterInfo| { &mut m.rewarded_fetter_level_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "fetter_list",
            |m: &AvatarFetterInfo| { &m.fetter_list },
            |m: &mut AvatarFetterInfo| { &mut m.fetter_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarFetterInfo>(
            "AvatarFetterInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarFetterInfo {
    const NAME: &'static str = "AvatarFetterInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.exp_number = is.read_uint32()?;
                },
                16 => {
                    self.exp_level = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.open_id_list)?;
                },
                24 => {
                    self.open_id_list.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.finish_id_list)?;
                },
                32 => {
                    self.finish_id_list.push(is.read_uint32()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.rewarded_fetter_level_list)?;
                },
                40 => {
                    self.rewarded_fetter_level_list.push(is.read_uint32()?);
                },
                50 => {
                    self.fetter_list.push(is.read_message()?);
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
        if self.exp_number != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.exp_number);
        }
        if self.exp_level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.exp_level);
        }
        for value in &self.open_id_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.finish_id_list {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        for value in &self.rewarded_fetter_level_list {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.fetter_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.exp_number != 0 {
            os.write_uint32(1, self.exp_number)?;
        }
        if self.exp_level != 0 {
            os.write_uint32(2, self.exp_level)?;
        }
        for v in &self.open_id_list {
            os.write_uint32(3, *v)?;
        };
        for v in &self.finish_id_list {
            os.write_uint32(4, *v)?;
        };
        for v in &self.rewarded_fetter_level_list {
            os.write_uint32(5, *v)?;
        };
        for v in &self.fetter_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> AvatarFetterInfo {
        AvatarFetterInfo::new()
    }

    fn clear(&mut self) {
        self.exp_number = 0;
        self.exp_level = 0;
        self.open_id_list.clear();
        self.finish_id_list.clear();
        self.rewarded_fetter_level_list.clear();
        self.fetter_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarFetterInfo {
        static instance: AvatarFetterInfo = AvatarFetterInfo {
            exp_number: 0,
            exp_level: 0,
            open_id_list: ::std::vec::Vec::new(),
            finish_id_list: ::std::vec::Vec::new(),
            rewarded_fetter_level_list: ::std::vec::Vec::new(),
            fetter_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarFetterInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarFetterInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarFetterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarFetterInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16AvatarFetterInfo.proto\x1a\x10FetterData.proto\"\x81\x02\n\x10Avat\
    arFetterInfo\x12\x1d\n\nexp_number\x18\x01\x20\x01(\rR\texpNumber\x12\
    \x1b\n\texp_level\x18\x02\x20\x01(\rR\x08expLevel\x12\x20\n\x0copen_id_l\
    ist\x18\x03\x20\x03(\rR\nopenIdList\x12$\n\x0efinish_id_list\x18\x04\x20\
    \x03(\rR\x0cfinishIdList\x12;\n\x1arewarded_fetter_level_list\x18\x05\
    \x20\x03(\rR\x17rewardedFetterLevelList\x12,\n\x0bfetter_list\x18\x06\
    \x20\x03(\x0b2\x0b.FetterDataR\nfetterListB\x1b\n\x19emu.grasscutter.net\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::FetterData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarFetterInfo::generated_message_descriptor_data());
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
