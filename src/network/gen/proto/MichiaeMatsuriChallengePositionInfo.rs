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

//! Generated file from `MichiaeMatsuriChallengePositionInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MichiaeMatsuriChallengePositionInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MichiaeMatsuriChallengePositionInfo {
    // message fields
    // @@protoc_insertion_point(field:MichiaeMatsuriChallengePositionInfo.gadget_id)
    pub gadget_id: u32,
    // @@protoc_insertion_point(field:MichiaeMatsuriChallengePositionInfo.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:MichiaeMatsuriChallengePositionInfo.group_id)
    pub group_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MichiaeMatsuriChallengePositionInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MichiaeMatsuriChallengePositionInfo {
    fn default() -> &'a MichiaeMatsuriChallengePositionInfo {
        <MichiaeMatsuriChallengePositionInfo as ::protobuf::Message>::default_instance()
    }
}

impl MichiaeMatsuriChallengePositionInfo {
    pub fn new() -> MichiaeMatsuriChallengePositionInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gadget_id",
            |m: &MichiaeMatsuriChallengePositionInfo| { &m.gadget_id },
            |m: &mut MichiaeMatsuriChallengePositionInfo| { &mut m.gadget_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &MichiaeMatsuriChallengePositionInfo| { &m.pos },
            |m: &mut MichiaeMatsuriChallengePositionInfo| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &MichiaeMatsuriChallengePositionInfo| { &m.group_id },
            |m: &mut MichiaeMatsuriChallengePositionInfo| { &mut m.group_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MichiaeMatsuriChallengePositionInfo>(
            "MichiaeMatsuriChallengePositionInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MichiaeMatsuriChallengePositionInfo {
    const NAME: &'static str = "MichiaeMatsuriChallengePositionInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.gadget_id = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                104 => {
                    self.group_id = is.read_uint32()?;
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
        if self.gadget_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.gadget_id);
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.group_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gadget_id != 0 {
            os.write_uint32(14, self.gadget_id)?;
        }
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.group_id != 0 {
            os.write_uint32(13, self.group_id)?;
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

    fn new() -> MichiaeMatsuriChallengePositionInfo {
        MichiaeMatsuriChallengePositionInfo::new()
    }

    fn clear(&mut self) {
        self.gadget_id = 0;
        self.pos.clear();
        self.group_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MichiaeMatsuriChallengePositionInfo {
        static instance: MichiaeMatsuriChallengePositionInfo = MichiaeMatsuriChallengePositionInfo {
            gadget_id: 0,
            pos: ::protobuf::MessageField::none(),
            group_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MichiaeMatsuriChallengePositionInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MichiaeMatsuriChallengePositionInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MichiaeMatsuriChallengePositionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MichiaeMatsuriChallengePositionInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)MichiaeMatsuriChallengePositionInfo.proto\x1a\x0cVector.proto\"x\n#Mi\
    chiaeMatsuriChallengePositionInfo\x12\x1b\n\tgadget_id\x18\x0e\x20\x01(\
    \rR\x08gadgetId\x12\x19\n\x03pos\x18\x0b\x20\x01(\x0b2\x07.VectorR\x03po\
    s\x12\x19\n\x08group_id\x18\r\x20\x01(\rR\x07groupIdB\x1b\n\x19emu.grass\
    cutter.net.protob\x06proto3\
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
            messages.push(MichiaeMatsuriChallengePositionInfo::generated_message_descriptor_data());
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