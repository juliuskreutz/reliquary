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

//! Generated file from `HomeFurnitureGroupData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HomeFurnitureGroupData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeFurnitureGroupData {
    // message fields
    // @@protoc_insertion_point(field:HomeFurnitureGroupData.virtual_furniure_list)
    pub virtual_furniure_list: ::std::vec::Vec<super::HomeFurnitureData::HomeFurnitureData>,
    // @@protoc_insertion_point(field:HomeFurnitureGroupData.group_furniture_index)
    pub group_furniture_index: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HomeFurnitureGroupData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeFurnitureGroupData {
    fn default() -> &'a HomeFurnitureGroupData {
        <HomeFurnitureGroupData as ::protobuf::Message>::default_instance()
    }
}

impl HomeFurnitureGroupData {
    pub fn new() -> HomeFurnitureGroupData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "virtual_furniure_list",
            |m: &HomeFurnitureGroupData| { &m.virtual_furniure_list },
            |m: &mut HomeFurnitureGroupData| { &mut m.virtual_furniure_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_furniture_index",
            |m: &HomeFurnitureGroupData| { &m.group_furniture_index },
            |m: &mut HomeFurnitureGroupData| { &mut m.group_furniture_index },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeFurnitureGroupData>(
            "HomeFurnitureGroupData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeFurnitureGroupData {
    const NAME: &'static str = "HomeFurnitureGroupData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.virtual_furniure_list.push(is.read_message()?);
                },
                64 => {
                    self.group_furniture_index = is.read_uint32()?;
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
        for value in &self.virtual_furniure_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.group_furniture_index != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.group_furniture_index);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.virtual_furniure_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.group_furniture_index != 0 {
            os.write_uint32(8, self.group_furniture_index)?;
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

    fn new() -> HomeFurnitureGroupData {
        HomeFurnitureGroupData::new()
    }

    fn clear(&mut self) {
        self.virtual_furniure_list.clear();
        self.group_furniture_index = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeFurnitureGroupData {
        static instance: HomeFurnitureGroupData = HomeFurnitureGroupData {
            virtual_furniure_list: ::std::vec::Vec::new(),
            group_furniture_index: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeFurnitureGroupData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeFurnitureGroupData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeFurnitureGroupData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeFurnitureGroupData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cHomeFurnitureGroupData.proto\x1a\x17HomeFurnitureData.proto\"\x94\
    \x01\n\x16HomeFurnitureGroupData\x12F\n\x15virtual_furniure_list\x18\x05\
    \x20\x03(\x0b2\x12.HomeFurnitureDataR\x13virtualFurniureList\x122\n\x15g\
    roup_furniture_index\x18\x08\x20\x01(\rR\x13groupFurnitureIndexB\x1b\n\
    \x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::HomeFurnitureData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeFurnitureGroupData::generated_message_descriptor_data());
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