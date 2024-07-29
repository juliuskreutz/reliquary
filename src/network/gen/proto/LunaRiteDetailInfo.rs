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

//! Generated file from `LunaRiteDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LunaRiteDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LunaRiteDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:LunaRiteDetailInfo.area_info_list)
    pub area_info_list: ::std::vec::Vec<super::LunaRiteAreaInfo::LunaRiteAreaInfo>,
    // @@protoc_insertion_point(field:LunaRiteDetailInfo.hint_point)
    pub hint_point: ::std::vec::Vec<super::LunaRiteHintPoint::LunaRiteHintPoint>,
    // special fields
    // @@protoc_insertion_point(special_field:LunaRiteDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LunaRiteDetailInfo {
    fn default() -> &'a LunaRiteDetailInfo {
        <LunaRiteDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl LunaRiteDetailInfo {
    pub fn new() -> LunaRiteDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "area_info_list",
            |m: &LunaRiteDetailInfo| { &m.area_info_list },
            |m: &mut LunaRiteDetailInfo| { &mut m.area_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "hint_point",
            |m: &LunaRiteDetailInfo| { &m.hint_point },
            |m: &mut LunaRiteDetailInfo| { &mut m.hint_point },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LunaRiteDetailInfo>(
            "LunaRiteDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LunaRiteDetailInfo {
    const NAME: &'static str = "LunaRiteDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.area_info_list.push(is.read_message()?);
                },
                122 => {
                    self.hint_point.push(is.read_message()?);
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
        for value in &self.area_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.hint_point {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.area_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.hint_point {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> LunaRiteDetailInfo {
        LunaRiteDetailInfo::new()
    }

    fn clear(&mut self) {
        self.area_info_list.clear();
        self.hint_point.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LunaRiteDetailInfo {
        static instance: LunaRiteDetailInfo = LunaRiteDetailInfo {
            area_info_list: ::std::vec::Vec::new(),
            hint_point: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LunaRiteDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LunaRiteDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LunaRiteDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LunaRiteDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18LunaRiteDetailInfo.proto\x1a\x16LunaRiteAreaInfo.proto\x1a\x17Luna\
    RiteHintPoint.proto\"\x80\x01\n\x12LunaRiteDetailInfo\x127\n\x0earea_inf\
    o_list\x18\x03\x20\x03(\x0b2\x11.LunaRiteAreaInfoR\x0careaInfoList\x121\
    \n\nhint_point\x18\x0f\x20\x03(\x0b2\x12.LunaRiteHintPointR\thintPointB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::LunaRiteAreaInfo::file_descriptor().clone());
            deps.push(super::LunaRiteHintPoint::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LunaRiteDetailInfo::generated_message_descriptor_data());
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