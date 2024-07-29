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

//! Generated file from `TakeoffEquipRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeoffEquipRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeoffEquipRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeoffEquipRsp.avatar_guid)
    pub avatar_guid: u64,
    // @@protoc_insertion_point(field:TakeoffEquipRsp.slot)
    pub slot: u32,
    // @@protoc_insertion_point(field:TakeoffEquipRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeoffEquipRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeoffEquipRsp {
    fn default() -> &'a TakeoffEquipRsp {
        <TakeoffEquipRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeoffEquipRsp {
    pub fn new() -> TakeoffEquipRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &TakeoffEquipRsp| { &m.avatar_guid },
            |m: &mut TakeoffEquipRsp| { &mut m.avatar_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &TakeoffEquipRsp| { &m.slot },
            |m: &mut TakeoffEquipRsp| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeoffEquipRsp| { &m.retcode },
            |m: &mut TakeoffEquipRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeoffEquipRsp>(
            "TakeoffEquipRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeoffEquipRsp {
    const NAME: &'static str = "TakeoffEquipRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.avatar_guid = is.read_uint64()?;
                },
                48 => {
                    self.slot = is.read_uint32()?;
                },
                64 => {
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
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.avatar_guid);
        }
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.slot);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_guid != 0 {
            os.write_uint64(2, self.avatar_guid)?;
        }
        if self.slot != 0 {
            os.write_uint32(6, self.slot)?;
        }
        if self.retcode != 0 {
            os.write_int32(8, self.retcode)?;
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

    fn new() -> TakeoffEquipRsp {
        TakeoffEquipRsp::new()
    }

    fn clear(&mut self) {
        self.avatar_guid = 0;
        self.slot = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeoffEquipRsp {
        static instance: TakeoffEquipRsp = TakeoffEquipRsp {
            avatar_guid: 0,
            slot: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeoffEquipRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeoffEquipRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeoffEquipRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeoffEquipRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15TakeoffEquipRsp.proto\"`\n\x0fTakeoffEquipRsp\x12\x1f\n\x0bavatar_\
    guid\x18\x02\x20\x01(\x04R\navatarGuid\x12\x12\n\x04slot\x18\x06\x20\x01\
    (\rR\x04slot\x12\x18\n\x07retcode\x18\x08\x20\x01(\x05R\x07retcodeB\x1b\
    \n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(TakeoffEquipRsp::generated_message_descriptor_data());
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