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

//! Generated file from `TakeFurnitureMakeReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeFurnitureMakeReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeFurnitureMakeReq {
    // message fields
    // @@protoc_insertion_point(field:TakeFurnitureMakeReq.index)
    pub index: u32,
    // @@protoc_insertion_point(field:TakeFurnitureMakeReq.is_fast_finish)
    pub is_fast_finish: bool,
    // @@protoc_insertion_point(field:TakeFurnitureMakeReq.make_id)
    pub make_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeFurnitureMakeReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeFurnitureMakeReq {
    fn default() -> &'a TakeFurnitureMakeReq {
        <TakeFurnitureMakeReq as ::protobuf::Message>::default_instance()
    }
}

impl TakeFurnitureMakeReq {
    pub fn new() -> TakeFurnitureMakeReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &TakeFurnitureMakeReq| { &m.index },
            |m: &mut TakeFurnitureMakeReq| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_fast_finish",
            |m: &TakeFurnitureMakeReq| { &m.is_fast_finish },
            |m: &mut TakeFurnitureMakeReq| { &mut m.is_fast_finish },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "make_id",
            |m: &TakeFurnitureMakeReq| { &m.make_id },
            |m: &mut TakeFurnitureMakeReq| { &mut m.make_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeFurnitureMakeReq>(
            "TakeFurnitureMakeReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeFurnitureMakeReq {
    const NAME: &'static str = "TakeFurnitureMakeReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.index = is.read_uint32()?;
                },
                56 => {
                    self.is_fast_finish = is.read_bool()?;
                },
                104 => {
                    self.make_id = is.read_uint32()?;
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
        if self.index != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.index);
        }
        if self.is_fast_finish != false {
            my_size += 1 + 1;
        }
        if self.make_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.make_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.index != 0 {
            os.write_uint32(1, self.index)?;
        }
        if self.is_fast_finish != false {
            os.write_bool(7, self.is_fast_finish)?;
        }
        if self.make_id != 0 {
            os.write_uint32(13, self.make_id)?;
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

    fn new() -> TakeFurnitureMakeReq {
        TakeFurnitureMakeReq::new()
    }

    fn clear(&mut self) {
        self.index = 0;
        self.is_fast_finish = false;
        self.make_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeFurnitureMakeReq {
        static instance: TakeFurnitureMakeReq = TakeFurnitureMakeReq {
            index: 0,
            is_fast_finish: false,
            make_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeFurnitureMakeReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeFurnitureMakeReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeFurnitureMakeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeFurnitureMakeReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aTakeFurnitureMakeReq.proto\"k\n\x14TakeFurnitureMakeReq\x12\x14\n\
    \x05index\x18\x01\x20\x01(\rR\x05index\x12$\n\x0eis_fast_finish\x18\x07\
    \x20\x01(\x08R\x0cisFastFinish\x12\x17\n\x07make_id\x18\r\x20\x01(\rR\
    \x06makeIdB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(TakeFurnitureMakeReq::generated_message_descriptor_data());
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