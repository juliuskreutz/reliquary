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

//! Generated file from `HideAndSeekActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HideAndSeekActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HideAndSeekActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:HideAndSeekActivityDetailInfo.JPONAAHBGID)
    pub JPONAAHBGID: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HideAndSeekActivityDetailInfo.NPKOJCAPCIJ)
    pub NPKOJCAPCIJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HideAndSeekActivityDetailInfo.open_map_info_list)
    pub open_map_info_list: ::std::vec::Vec<super::HideAndSeekMapInfo::HideAndSeekMapInfo>,
    // @@protoc_insertion_point(field:HideAndSeekActivityDetailInfo.MDEIGNMPCOO)
    pub MDEIGNMPCOO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HideAndSeekActivityDetailInfo.LLDFJKPBKLM)
    pub LLDFJKPBKLM: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:HideAndSeekActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HideAndSeekActivityDetailInfo {
    fn default() -> &'a HideAndSeekActivityDetailInfo {
        <HideAndSeekActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl HideAndSeekActivityDetailInfo {
    pub fn new() -> HideAndSeekActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JPONAAHBGID",
            |m: &HideAndSeekActivityDetailInfo| { &m.JPONAAHBGID },
            |m: &mut HideAndSeekActivityDetailInfo| { &mut m.JPONAAHBGID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NPKOJCAPCIJ",
            |m: &HideAndSeekActivityDetailInfo| { &m.NPKOJCAPCIJ },
            |m: &mut HideAndSeekActivityDetailInfo| { &mut m.NPKOJCAPCIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "open_map_info_list",
            |m: &HideAndSeekActivityDetailInfo| { &m.open_map_info_list },
            |m: &mut HideAndSeekActivityDetailInfo| { &mut m.open_map_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MDEIGNMPCOO",
            |m: &HideAndSeekActivityDetailInfo| { &m.MDEIGNMPCOO },
            |m: &mut HideAndSeekActivityDetailInfo| { &mut m.MDEIGNMPCOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LLDFJKPBKLM",
            |m: &HideAndSeekActivityDetailInfo| { &m.LLDFJKPBKLM },
            |m: &mut HideAndSeekActivityDetailInfo| { &mut m.LLDFJKPBKLM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HideAndSeekActivityDetailInfo>(
            "HideAndSeekActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HideAndSeekActivityDetailInfo {
    const NAME: &'static str = "HideAndSeekActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.JPONAAHBGID)?;
                },
                16 => {
                    self.JPONAAHBGID.push(is.read_uint32()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.NPKOJCAPCIJ)?;
                },
                40 => {
                    self.NPKOJCAPCIJ.push(is.read_uint32()?);
                },
                114 => {
                    self.open_map_info_list.push(is.read_message()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.MDEIGNMPCOO)?;
                },
                64 => {
                    self.MDEIGNMPCOO.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.LLDFJKPBKLM)?;
                },
                24 => {
                    self.LLDFJKPBKLM.push(is.read_uint32()?);
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
        for value in &self.JPONAAHBGID {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        for value in &self.NPKOJCAPCIJ {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.open_map_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MDEIGNMPCOO {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.LLDFJKPBKLM {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.JPONAAHBGID {
            os.write_uint32(2, *v)?;
        };
        for v in &self.NPKOJCAPCIJ {
            os.write_uint32(5, *v)?;
        };
        for v in &self.open_map_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.MDEIGNMPCOO {
            os.write_uint32(8, *v)?;
        };
        for v in &self.LLDFJKPBKLM {
            os.write_uint32(3, *v)?;
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

    fn new() -> HideAndSeekActivityDetailInfo {
        HideAndSeekActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.JPONAAHBGID.clear();
        self.NPKOJCAPCIJ.clear();
        self.open_map_info_list.clear();
        self.MDEIGNMPCOO.clear();
        self.LLDFJKPBKLM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HideAndSeekActivityDetailInfo {
        static instance: HideAndSeekActivityDetailInfo = HideAndSeekActivityDetailInfo {
            JPONAAHBGID: ::std::vec::Vec::new(),
            NPKOJCAPCIJ: ::std::vec::Vec::new(),
            open_map_info_list: ::std::vec::Vec::new(),
            MDEIGNMPCOO: ::std::vec::Vec::new(),
            LLDFJKPBKLM: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HideAndSeekActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HideAndSeekActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HideAndSeekActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HideAndSeekActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#HideAndSeekActivityDetailInfo.proto\x1a\x18HideAndSeekMapInfo.proto\"\
    \xe9\x01\n\x1dHideAndSeekActivityDetailInfo\x12\x20\n\x0bJPONAAHBGID\x18\
    \x02\x20\x03(\rR\x0bJPONAAHBGID\x12\x20\n\x0bNPKOJCAPCIJ\x18\x05\x20\x03\
    (\rR\x0bNPKOJCAPCIJ\x12@\n\x12open_map_info_list\x18\x0e\x20\x03(\x0b2\
    \x13.HideAndSeekMapInfoR\x0fopenMapInfoList\x12\x20\n\x0bMDEIGNMPCOO\x18\
    \x08\x20\x03(\rR\x0bMDEIGNMPCOO\x12\x20\n\x0bLLDFJKPBKLM\x18\x03\x20\x03\
    (\rR\x0bLLDFJKPBKLMB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::HideAndSeekMapInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HideAndSeekActivityDetailInfo::generated_message_descriptor_data());
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
