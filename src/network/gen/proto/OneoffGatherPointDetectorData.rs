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

//! Generated file from `OneoffGatherPointDetectorData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OneoffGatherPointDetectorData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OneoffGatherPointDetectorData {
    // message fields
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.config_id)
    pub config_id: u32,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.is_hint_valid)
    pub is_hint_valid: bool,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.hint_center_pos)
    pub hint_center_pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.hint_radius)
    pub hint_radius: u32,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.material_id)
    pub material_id: u32,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.HJMMAOMEHOL)
    pub HJMMAOMEHOL: u32,
    // @@protoc_insertion_point(field:OneoffGatherPointDetectorData.is_all_collected)
    pub is_all_collected: bool,
    // special fields
    // @@protoc_insertion_point(special_field:OneoffGatherPointDetectorData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OneoffGatherPointDetectorData {
    fn default() -> &'a OneoffGatherPointDetectorData {
        <OneoffGatherPointDetectorData as ::protobuf::Message>::default_instance()
    }
}

impl OneoffGatherPointDetectorData {
    pub fn new() -> OneoffGatherPointDetectorData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "config_id",
            |m: &OneoffGatherPointDetectorData| { &m.config_id },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_hint_valid",
            |m: &OneoffGatherPointDetectorData| { &m.is_hint_valid },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.is_hint_valid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "hint_center_pos",
            |m: &OneoffGatherPointDetectorData| { &m.hint_center_pos },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.hint_center_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hint_radius",
            |m: &OneoffGatherPointDetectorData| { &m.hint_radius },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.hint_radius },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "material_id",
            |m: &OneoffGatherPointDetectorData| { &m.material_id },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.material_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &OneoffGatherPointDetectorData| { &m.group_id },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJMMAOMEHOL",
            |m: &OneoffGatherPointDetectorData| { &m.HJMMAOMEHOL },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.HJMMAOMEHOL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_all_collected",
            |m: &OneoffGatherPointDetectorData| { &m.is_all_collected },
            |m: &mut OneoffGatherPointDetectorData| { &mut m.is_all_collected },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OneoffGatherPointDetectorData>(
            "OneoffGatherPointDetectorData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OneoffGatherPointDetectorData {
    const NAME: &'static str = "OneoffGatherPointDetectorData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.config_id = is.read_uint32()?;
                },
                80 => {
                    self.is_hint_valid = is.read_bool()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.hint_center_pos)?;
                },
                64 => {
                    self.hint_radius = is.read_uint32()?;
                },
                72 => {
                    self.material_id = is.read_uint32()?;
                },
                88 => {
                    self.group_id = is.read_uint32()?;
                },
                120 => {
                    self.HJMMAOMEHOL = is.read_uint32()?;
                },
                48 => {
                    self.is_all_collected = is.read_bool()?;
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
        if self.config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.config_id);
        }
        if self.is_hint_valid != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.hint_center_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.hint_radius != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.hint_radius);
        }
        if self.material_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.material_id);
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.group_id);
        }
        if self.HJMMAOMEHOL != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.HJMMAOMEHOL);
        }
        if self.is_all_collected != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.config_id != 0 {
            os.write_uint32(7, self.config_id)?;
        }
        if self.is_hint_valid != false {
            os.write_bool(10, self.is_hint_valid)?;
        }
        if let Some(v) = self.hint_center_pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.hint_radius != 0 {
            os.write_uint32(8, self.hint_radius)?;
        }
        if self.material_id != 0 {
            os.write_uint32(9, self.material_id)?;
        }
        if self.group_id != 0 {
            os.write_uint32(11, self.group_id)?;
        }
        if self.HJMMAOMEHOL != 0 {
            os.write_uint32(15, self.HJMMAOMEHOL)?;
        }
        if self.is_all_collected != false {
            os.write_bool(6, self.is_all_collected)?;
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

    fn new() -> OneoffGatherPointDetectorData {
        OneoffGatherPointDetectorData::new()
    }

    fn clear(&mut self) {
        self.config_id = 0;
        self.is_hint_valid = false;
        self.hint_center_pos.clear();
        self.hint_radius = 0;
        self.material_id = 0;
        self.group_id = 0;
        self.HJMMAOMEHOL = 0;
        self.is_all_collected = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OneoffGatherPointDetectorData {
        static instance: OneoffGatherPointDetectorData = OneoffGatherPointDetectorData {
            config_id: 0,
            is_hint_valid: false,
            hint_center_pos: ::protobuf::MessageField::none(),
            hint_radius: 0,
            material_id: 0,
            group_id: 0,
            HJMMAOMEHOL: 0,
            is_all_collected: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OneoffGatherPointDetectorData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OneoffGatherPointDetectorData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OneoffGatherPointDetectorData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OneoffGatherPointDetectorData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#OneoffGatherPointDetectorData.proto\x1a\x0cVector.proto\"\xba\x02\n\
    \x1dOneoffGatherPointDetectorData\x12\x1b\n\tconfig_id\x18\x07\x20\x01(\
    \rR\x08configId\x12\"\n\ris_hint_valid\x18\n\x20\x01(\x08R\x0bisHintVali\
    d\x12/\n\x0fhint_center_pos\x18\x01\x20\x01(\x0b2\x07.VectorR\rhintCente\
    rPos\x12\x1f\n\x0bhint_radius\x18\x08\x20\x01(\rR\nhintRadius\x12\x1f\n\
    \x0bmaterial_id\x18\t\x20\x01(\rR\nmaterialId\x12\x19\n\x08group_id\x18\
    \x0b\x20\x01(\rR\x07groupId\x12\x20\n\x0bHJMMAOMEHOL\x18\x0f\x20\x01(\rR\
    \x0bHJMMAOMEHOL\x12(\n\x10is_all_collected\x18\x06\x20\x01(\x08R\x0eisAl\
    lCollectedB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OneoffGatherPointDetectorData::generated_message_descriptor_data());
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