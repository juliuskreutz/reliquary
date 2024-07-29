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

//! Generated file from `DungeonWayPointActivateRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DungeonWayPointActivateRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DungeonWayPointActivateRsp {
    // message fields
    // @@protoc_insertion_point(field:DungeonWayPointActivateRsp.way_point_id)
    pub way_point_id: u32,
    // @@protoc_insertion_point(field:DungeonWayPointActivateRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:DungeonWayPointActivateRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DungeonWayPointActivateRsp {
    fn default() -> &'a DungeonWayPointActivateRsp {
        <DungeonWayPointActivateRsp as ::protobuf::Message>::default_instance()
    }
}

impl DungeonWayPointActivateRsp {
    pub fn new() -> DungeonWayPointActivateRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "way_point_id",
            |m: &DungeonWayPointActivateRsp| { &m.way_point_id },
            |m: &mut DungeonWayPointActivateRsp| { &mut m.way_point_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DungeonWayPointActivateRsp| { &m.retcode },
            |m: &mut DungeonWayPointActivateRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DungeonWayPointActivateRsp>(
            "DungeonWayPointActivateRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DungeonWayPointActivateRsp {
    const NAME: &'static str = "DungeonWayPointActivateRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.way_point_id = is.read_uint32()?;
                },
                112 => {
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
        if self.way_point_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.way_point_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.way_point_id != 0 {
            os.write_uint32(3, self.way_point_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(14, self.retcode)?;
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

    fn new() -> DungeonWayPointActivateRsp {
        DungeonWayPointActivateRsp::new()
    }

    fn clear(&mut self) {
        self.way_point_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DungeonWayPointActivateRsp {
        static instance: DungeonWayPointActivateRsp = DungeonWayPointActivateRsp {
            way_point_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DungeonWayPointActivateRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DungeonWayPointActivateRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DungeonWayPointActivateRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DungeonWayPointActivateRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20DungeonWayPointActivateRsp.proto\"X\n\x1aDungeonWayPointActivateRs\
    p\x12\x20\n\x0cway_point_id\x18\x03\x20\x01(\rR\nwayPointId\x12\x18\n\
    \x07retcode\x18\x0e\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.n\
    et.protob\x06proto3\
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
            messages.push(DungeonWayPointActivateRsp::generated_message_descriptor_data());
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