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

//! Generated file from `ToTheMoonQueryPathRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ToTheMoonQueryPathRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ToTheMoonQueryPathRsp {
    // message fields
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.corners)
    pub corners: ::std::vec::Vec<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.query_id)
    pub query_id: i32,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.query_status)
    pub query_status: ::protobuf::EnumOrUnknown<to_the_moon_query_path_rsp::PathStatusType>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.level)
    pub level: ::std::vec::Vec<i32>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.index)
    pub index: ::std::vec::Vec<i64>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:ToTheMoonQueryPathRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ToTheMoonQueryPathRsp {
    fn default() -> &'a ToTheMoonQueryPathRsp {
        <ToTheMoonQueryPathRsp as ::protobuf::Message>::default_instance()
    }
}

impl ToTheMoonQueryPathRsp {
    pub fn new() -> ToTheMoonQueryPathRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "corners",
            |m: &ToTheMoonQueryPathRsp| { &m.corners },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.corners },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_id",
            |m: &ToTheMoonQueryPathRsp| { &m.query_id },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.query_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_status",
            |m: &ToTheMoonQueryPathRsp| { &m.query_status },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.query_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "level",
            |m: &ToTheMoonQueryPathRsp| { &m.level },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "index",
            |m: &ToTheMoonQueryPathRsp| { &m.index },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ToTheMoonQueryPathRsp| { &m.retcode },
            |m: &mut ToTheMoonQueryPathRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ToTheMoonQueryPathRsp>(
            "ToTheMoonQueryPathRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ToTheMoonQueryPathRsp {
    const NAME: &'static str = "ToTheMoonQueryPathRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.corners.push(is.read_message()?);
                },
                16 => {
                    self.query_id = is.read_int32()?;
                },
                88 => {
                    self.query_status = is.read_enum_or_unknown()?;
                },
                106 => {
                    is.read_repeated_packed_int32_into(&mut self.level)?;
                },
                104 => {
                    self.level.push(is.read_int32()?);
                },
                122 => {
                    is.read_repeated_packed_int64_into(&mut self.index)?;
                },
                120 => {
                    self.index.push(is.read_int64()?);
                },
                56 => {
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
        for value in &self.corners {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.query_id != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.query_id);
        }
        if self.query_status != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_rsp::PathStatusType::STATUS_FAIL) {
            my_size += ::protobuf::rt::int32_size(11, self.query_status.value());
        }
        for value in &self.level {
            my_size += ::protobuf::rt::int32_size(13, *value);
        };
        for value in &self.index {
            my_size += ::protobuf::rt::int64_size(15, *value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.corners {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.query_id != 0 {
            os.write_int32(2, self.query_id)?;
        }
        if self.query_status != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_rsp::PathStatusType::STATUS_FAIL) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.query_status))?;
        }
        for v in &self.level {
            os.write_int32(13, *v)?;
        };
        for v in &self.index {
            os.write_int64(15, *v)?;
        };
        if self.retcode != 0 {
            os.write_int32(7, self.retcode)?;
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

    fn new() -> ToTheMoonQueryPathRsp {
        ToTheMoonQueryPathRsp::new()
    }

    fn clear(&mut self) {
        self.corners.clear();
        self.query_id = 0;
        self.query_status = ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_rsp::PathStatusType::STATUS_FAIL);
        self.level.clear();
        self.index.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ToTheMoonQueryPathRsp {
        static instance: ToTheMoonQueryPathRsp = ToTheMoonQueryPathRsp {
            corners: ::std::vec::Vec::new(),
            query_id: 0,
            query_status: ::protobuf::EnumOrUnknown::from_i32(0),
            level: ::std::vec::Vec::new(),
            index: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ToTheMoonQueryPathRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ToTheMoonQueryPathRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ToTheMoonQueryPathRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToTheMoonQueryPathRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ToTheMoonQueryPathRsp`
pub mod to_the_moon_query_path_rsp {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ToTheMoonQueryPathRsp.PathStatusType)
    pub enum PathStatusType {
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathRsp.PathStatusType.STATUS_FAIL)
        STATUS_FAIL = 0,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathRsp.PathStatusType.STATUS_SUCC)
        STATUS_SUCC = 1,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathRsp.PathStatusType.STATUS_PARTIAL)
        STATUS_PARTIAL = 2,
    }

    impl ::protobuf::Enum for PathStatusType {
        const NAME: &'static str = "PathStatusType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<PathStatusType> {
            match value {
                0 => ::std::option::Option::Some(PathStatusType::STATUS_FAIL),
                1 => ::std::option::Option::Some(PathStatusType::STATUS_SUCC),
                2 => ::std::option::Option::Some(PathStatusType::STATUS_PARTIAL),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<PathStatusType> {
            match str {
                "STATUS_FAIL" => ::std::option::Option::Some(PathStatusType::STATUS_FAIL),
                "STATUS_SUCC" => ::std::option::Option::Some(PathStatusType::STATUS_SUCC),
                "STATUS_PARTIAL" => ::std::option::Option::Some(PathStatusType::STATUS_PARTIAL),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [PathStatusType] = &[
            PathStatusType::STATUS_FAIL,
            PathStatusType::STATUS_SUCC,
            PathStatusType::STATUS_PARTIAL,
        ];
    }

    impl ::protobuf::EnumFull for PathStatusType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ToTheMoonQueryPathRsp.PathStatusType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for PathStatusType {
        fn default() -> Self {
            PathStatusType::STATUS_FAIL
        }
    }

    impl PathStatusType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PathStatusType>("ToTheMoonQueryPathRsp.PathStatusType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bToTheMoonQueryPathRsp.proto\x1a\x0cVector.proto\"\xad\x02\n\x15ToT\
    heMoonQueryPathRsp\x12!\n\x07corners\x18\x0e\x20\x03(\x0b2\x07.VectorR\
    \x07corners\x12\x19\n\x08query_id\x18\x02\x20\x01(\x05R\x07queryId\x12H\
    \n\x0cquery_status\x18\x0b\x20\x01(\x0e2%.ToTheMoonQueryPathRsp.PathStat\
    usTypeR\x0bqueryStatus\x12\x14\n\x05level\x18\r\x20\x03(\x05R\x05level\
    \x12\x14\n\x05index\x18\x0f\x20\x03(\x03R\x05index\x12\x18\n\x07retcode\
    \x18\x07\x20\x01(\x05R\x07retcode\"F\n\x0ePathStatusType\x12\x0f\n\x0bST\
    ATUS_FAIL\x10\0\x12\x0f\n\x0bSTATUS_SUCC\x10\x01\x12\x12\n\x0eSTATUS_PAR\
    TIAL\x10\x02B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ToTheMoonQueryPathRsp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(to_the_moon_query_path_rsp::PathStatusType::generated_enum_descriptor_data());
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
