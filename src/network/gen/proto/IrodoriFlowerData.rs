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

//! Generated file from `IrodoriFlowerData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IrodoriFlowerData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IrodoriFlowerData {
    // message fields
    // @@protoc_insertion_point(field:IrodoriFlowerData.finished_theme_list)
    pub finished_theme_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IrodoriFlowerData.used_flower_list)
    pub used_flower_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // special fields
    // @@protoc_insertion_point(special_field:IrodoriFlowerData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IrodoriFlowerData {
    fn default() -> &'a IrodoriFlowerData {
        <IrodoriFlowerData as ::protobuf::Message>::default_instance()
    }
}

impl IrodoriFlowerData {
    pub fn new() -> IrodoriFlowerData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "finished_theme_list",
            |m: &IrodoriFlowerData| { &m.finished_theme_list },
            |m: &mut IrodoriFlowerData| { &mut m.finished_theme_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "used_flower_list",
            |m: &IrodoriFlowerData| { &m.used_flower_list },
            |m: &mut IrodoriFlowerData| { &mut m.used_flower_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IrodoriFlowerData>(
            "IrodoriFlowerData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IrodoriFlowerData {
    const NAME: &'static str = "IrodoriFlowerData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.finished_theme_list)?;
                },
                24 => {
                    self.finished_theme_list.push(is.read_uint32()?);
                },
                18 => {
                    self.used_flower_list.push(is.read_message()?);
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
        for value in &self.finished_theme_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.used_flower_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.finished_theme_list {
            os.write_uint32(3, *v)?;
        };
        for v in &self.used_flower_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> IrodoriFlowerData {
        IrodoriFlowerData::new()
    }

    fn clear(&mut self) {
        self.finished_theme_list.clear();
        self.used_flower_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IrodoriFlowerData {
        static instance: IrodoriFlowerData = IrodoriFlowerData {
            finished_theme_list: ::std::vec::Vec::new(),
            used_flower_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IrodoriFlowerData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IrodoriFlowerData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IrodoriFlowerData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IrodoriFlowerData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17IrodoriFlowerData.proto\x1a\x0fItemParam.proto\"y\n\x11IrodoriFlow\
    erData\x12.\n\x13finished_theme_list\x18\x03\x20\x03(\rR\x11finishedThem\
    eList\x124\n\x10used_flower_list\x18\x02\x20\x03(\x0b2\n.ItemParamR\x0eu\
    sedFlowerListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::ItemParam::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IrodoriFlowerData::generated_message_descriptor_data());
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
