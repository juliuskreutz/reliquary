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

//! Generated file from `ElectroherculesBattleLevelInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ElectroherculesBattleLevelInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ElectroherculesBattleLevelInfo {
    // message fields
    // @@protoc_insertion_point(field:ElectroherculesBattleLevelInfo.min_finish_time)
    pub min_finish_time: u32,
    // @@protoc_insertion_point(field:ElectroherculesBattleLevelInfo.is_finish)
    pub is_finish: bool,
    // @@protoc_insertion_point(field:ElectroherculesBattleLevelInfo.level_id)
    pub level_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ElectroherculesBattleLevelInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ElectroherculesBattleLevelInfo {
    fn default() -> &'a ElectroherculesBattleLevelInfo {
        <ElectroherculesBattleLevelInfo as ::protobuf::Message>::default_instance()
    }
}

impl ElectroherculesBattleLevelInfo {
    pub fn new() -> ElectroherculesBattleLevelInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "min_finish_time",
            |m: &ElectroherculesBattleLevelInfo| { &m.min_finish_time },
            |m: &mut ElectroherculesBattleLevelInfo| { &mut m.min_finish_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finish",
            |m: &ElectroherculesBattleLevelInfo| { &m.is_finish },
            |m: &mut ElectroherculesBattleLevelInfo| { &mut m.is_finish },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &ElectroherculesBattleLevelInfo| { &m.level_id },
            |m: &mut ElectroherculesBattleLevelInfo| { &mut m.level_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ElectroherculesBattleLevelInfo>(
            "ElectroherculesBattleLevelInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ElectroherculesBattleLevelInfo {
    const NAME: &'static str = "ElectroherculesBattleLevelInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.min_finish_time = is.read_uint32()?;
                },
                64 => {
                    self.is_finish = is.read_bool()?;
                },
                96 => {
                    self.level_id = is.read_uint32()?;
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
        if self.min_finish_time != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.min_finish_time);
        }
        if self.is_finish != false {
            my_size += 1 + 1;
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.level_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.min_finish_time != 0 {
            os.write_uint32(10, self.min_finish_time)?;
        }
        if self.is_finish != false {
            os.write_bool(8, self.is_finish)?;
        }
        if self.level_id != 0 {
            os.write_uint32(12, self.level_id)?;
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

    fn new() -> ElectroherculesBattleLevelInfo {
        ElectroherculesBattleLevelInfo::new()
    }

    fn clear(&mut self) {
        self.min_finish_time = 0;
        self.is_finish = false;
        self.level_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ElectroherculesBattleLevelInfo {
        static instance: ElectroherculesBattleLevelInfo = ElectroherculesBattleLevelInfo {
            min_finish_time: 0,
            is_finish: false,
            level_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ElectroherculesBattleLevelInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ElectroherculesBattleLevelInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ElectroherculesBattleLevelInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ElectroherculesBattleLevelInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$ElectroherculesBattleLevelInfo.proto\"\x80\x01\n\x1eElectroherculesBa\
    ttleLevelInfo\x12&\n\x0fmin_finish_time\x18\n\x20\x01(\rR\rminFinishTime\
    \x12\x1b\n\tis_finish\x18\x08\x20\x01(\x08R\x08isFinish\x12\x19\n\x08lev\
    el_id\x18\x0c\x20\x01(\rR\x07levelIdB\x1b\n\x19emu.grasscutter.net.proto\
    b\x06proto3\
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
            messages.push(ElectroherculesBattleLevelInfo::generated_message_descriptor_data());
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
