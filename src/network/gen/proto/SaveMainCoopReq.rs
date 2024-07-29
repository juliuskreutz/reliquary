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

//! Generated file from `SaveMainCoopReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SaveMainCoopReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SaveMainCoopReq {
    // message fields
    // @@protoc_insertion_point(field:SaveMainCoopReq.GEHNFJEPCJL)
    pub GEHNFJEPCJL: ::std::collections::HashMap<u32, i32>,
    // @@protoc_insertion_point(field:SaveMainCoopReq.GDBKBKACDFO)
    pub GDBKBKACDFO: ::std::collections::HashMap<u32, i32>,
    // @@protoc_insertion_point(field:SaveMainCoopReq.save_point_id)
    pub save_point_id: u32,
    // @@protoc_insertion_point(field:SaveMainCoopReq.id)
    pub id: u32,
    // @@protoc_insertion_point(field:SaveMainCoopReq.self_confidence)
    pub self_confidence: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SaveMainCoopReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SaveMainCoopReq {
    fn default() -> &'a SaveMainCoopReq {
        <SaveMainCoopReq as ::protobuf::Message>::default_instance()
    }
}

impl SaveMainCoopReq {
    pub fn new() -> SaveMainCoopReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "GEHNFJEPCJL",
            |m: &SaveMainCoopReq| { &m.GEHNFJEPCJL },
            |m: &mut SaveMainCoopReq| { &mut m.GEHNFJEPCJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "GDBKBKACDFO",
            |m: &SaveMainCoopReq| { &m.GDBKBKACDFO },
            |m: &mut SaveMainCoopReq| { &mut m.GDBKBKACDFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "save_point_id",
            |m: &SaveMainCoopReq| { &m.save_point_id },
            |m: &mut SaveMainCoopReq| { &mut m.save_point_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &SaveMainCoopReq| { &m.id },
            |m: &mut SaveMainCoopReq| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "self_confidence",
            |m: &SaveMainCoopReq| { &m.self_confidence },
            |m: &mut SaveMainCoopReq| { &mut m.self_confidence },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SaveMainCoopReq>(
            "SaveMainCoopReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SaveMainCoopReq {
    const NAME: &'static str = "SaveMainCoopReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_int32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.GEHNFJEPCJL.insert(key, value);
                },
                82 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_int32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.GDBKBKACDFO.insert(key, value);
                },
                88 => {
                    self.save_point_id = is.read_uint32()?;
                },
                48 => {
                    self.id = is.read_uint32()?;
                },
                56 => {
                    self.self_confidence = is.read_uint32()?;
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
        for (k, v) in &self.GEHNFJEPCJL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.GDBKBKACDFO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.save_point_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.save_point_id);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.id);
        }
        if self.self_confidence != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.self_confidence);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.GEHNFJEPCJL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(26)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_int32(2, *v)?;
        };
        for (k, v) in &self.GDBKBKACDFO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(82)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_int32(2, *v)?;
        };
        if self.save_point_id != 0 {
            os.write_uint32(11, self.save_point_id)?;
        }
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
        }
        if self.self_confidence != 0 {
            os.write_uint32(7, self.self_confidence)?;
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

    fn new() -> SaveMainCoopReq {
        SaveMainCoopReq::new()
    }

    fn clear(&mut self) {
        self.GEHNFJEPCJL.clear();
        self.GDBKBKACDFO.clear();
        self.save_point_id = 0;
        self.id = 0;
        self.self_confidence = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SaveMainCoopReq {
        static instance: ::protobuf::rt::Lazy<SaveMainCoopReq> = ::protobuf::rt::Lazy::new();
        instance.get(SaveMainCoopReq::new)
    }
}

impl ::protobuf::MessageFull for SaveMainCoopReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SaveMainCoopReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SaveMainCoopReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SaveMainCoopReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SaveMainCoopReq.proto\"\xf8\x02\n\x0fSaveMainCoopReq\x12C\n\x0bGEH\
    NFJEPCJL\x18\x03\x20\x03(\x0b2!.SaveMainCoopReq.GEHNFJEPCJLEntryR\x0bGEH\
    NFJEPCJL\x12C\n\x0bGDBKBKACDFO\x18\n\x20\x03(\x0b2!.SaveMainCoopReq.GDBK\
    BKACDFOEntryR\x0bGDBKBKACDFO\x12\"\n\rsave_point_id\x18\x0b\x20\x01(\rR\
    \x0bsavePointId\x12\x0e\n\x02id\x18\x06\x20\x01(\rR\x02id\x12'\n\x0fself\
    _confidence\x18\x07\x20\x01(\rR\x0eselfConfidence\x1a>\n\x10GEHNFJEPCJLE\
    ntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\x05R\x05value:\x028\x01\x1a>\n\x10GDBKBKACDFOEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \x05R\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(SaveMainCoopReq::generated_message_descriptor_data());
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