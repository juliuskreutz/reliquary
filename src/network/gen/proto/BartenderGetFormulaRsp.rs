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

//! Generated file from `BartenderGetFormulaRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BartenderGetFormulaRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BartenderGetFormulaRsp {
    // message fields
    // @@protoc_insertion_point(field:BartenderGetFormulaRsp.is_new)
    pub is_new: bool,
    // @@protoc_insertion_point(field:BartenderGetFormulaRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:BartenderGetFormulaRsp.affix_list)
    pub affix_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BartenderGetFormulaRsp.formula_id)
    pub formula_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BartenderGetFormulaRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BartenderGetFormulaRsp {
    fn default() -> &'a BartenderGetFormulaRsp {
        <BartenderGetFormulaRsp as ::protobuf::Message>::default_instance()
    }
}

impl BartenderGetFormulaRsp {
    pub fn new() -> BartenderGetFormulaRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_new",
            |m: &BartenderGetFormulaRsp| { &m.is_new },
            |m: &mut BartenderGetFormulaRsp| { &mut m.is_new },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &BartenderGetFormulaRsp| { &m.retcode },
            |m: &mut BartenderGetFormulaRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "affix_list",
            |m: &BartenderGetFormulaRsp| { &m.affix_list },
            |m: &mut BartenderGetFormulaRsp| { &mut m.affix_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "formula_id",
            |m: &BartenderGetFormulaRsp| { &m.formula_id },
            |m: &mut BartenderGetFormulaRsp| { &mut m.formula_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BartenderGetFormulaRsp>(
            "BartenderGetFormulaRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BartenderGetFormulaRsp {
    const NAME: &'static str = "BartenderGetFormulaRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.is_new = is.read_bool()?;
                },
                72 => {
                    self.retcode = is.read_int32()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.affix_list)?;
                },
                80 => {
                    self.affix_list.push(is.read_uint32()?);
                },
                88 => {
                    self.formula_id = is.read_uint32()?;
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
        if self.is_new != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.retcode);
        }
        for value in &self.affix_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.formula_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.formula_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_new != false {
            os.write_bool(6, self.is_new)?;
        }
        if self.retcode != 0 {
            os.write_int32(9, self.retcode)?;
        }
        for v in &self.affix_list {
            os.write_uint32(10, *v)?;
        };
        if self.formula_id != 0 {
            os.write_uint32(11, self.formula_id)?;
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

    fn new() -> BartenderGetFormulaRsp {
        BartenderGetFormulaRsp::new()
    }

    fn clear(&mut self) {
        self.is_new = false;
        self.retcode = 0;
        self.affix_list.clear();
        self.formula_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BartenderGetFormulaRsp {
        static instance: BartenderGetFormulaRsp = BartenderGetFormulaRsp {
            is_new: false,
            retcode: 0,
            affix_list: ::std::vec::Vec::new(),
            formula_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BartenderGetFormulaRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BartenderGetFormulaRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BartenderGetFormulaRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BartenderGetFormulaRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cBartenderGetFormulaRsp.proto\"\x87\x01\n\x16BartenderGetFormulaRsp\
    \x12\x15\n\x06is_new\x18\x06\x20\x01(\x08R\x05isNew\x12\x18\n\x07retcode\
    \x18\t\x20\x01(\x05R\x07retcode\x12\x1d\n\naffix_list\x18\n\x20\x03(\rR\
    \taffixList\x12\x1d\n\nformula_id\x18\x0b\x20\x01(\rR\tformulaIdB\x1b\n\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BartenderGetFormulaRsp::generated_message_descriptor_data());
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