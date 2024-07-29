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

//! Generated file from `BartenderActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BartenderActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BartenderActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.EFCFFFOKIIO)
    pub EFCFFFOKIIO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.OJDLKJNDJAH)
    pub OJDLKJNDJAH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.is_develop_module_open)
    pub is_develop_module_open: bool,
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.unlock_task_list)
    pub unlock_task_list: ::std::vec::Vec<super::BartenderTaskInfo::BartenderTaskInfo>,
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.is_content_closed)
    pub is_content_closed: bool,
    // @@protoc_insertion_point(field:BartenderActivityDetailInfo.unlock_level_list)
    pub unlock_level_list: ::std::vec::Vec<super::BartenderLevelInfo::BartenderLevelInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:BartenderActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BartenderActivityDetailInfo {
    fn default() -> &'a BartenderActivityDetailInfo {
        <BartenderActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl BartenderActivityDetailInfo {
    pub fn new() -> BartenderActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EFCFFFOKIIO",
            |m: &BartenderActivityDetailInfo| { &m.EFCFFFOKIIO },
            |m: &mut BartenderActivityDetailInfo| { &mut m.EFCFFFOKIIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OJDLKJNDJAH",
            |m: &BartenderActivityDetailInfo| { &m.OJDLKJNDJAH },
            |m: &mut BartenderActivityDetailInfo| { &mut m.OJDLKJNDJAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_develop_module_open",
            |m: &BartenderActivityDetailInfo| { &m.is_develop_module_open },
            |m: &mut BartenderActivityDetailInfo| { &mut m.is_develop_module_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlock_task_list",
            |m: &BartenderActivityDetailInfo| { &m.unlock_task_list },
            |m: &mut BartenderActivityDetailInfo| { &mut m.unlock_task_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_content_closed",
            |m: &BartenderActivityDetailInfo| { &m.is_content_closed },
            |m: &mut BartenderActivityDetailInfo| { &mut m.is_content_closed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlock_level_list",
            |m: &BartenderActivityDetailInfo| { &m.unlock_level_list },
            |m: &mut BartenderActivityDetailInfo| { &mut m.unlock_level_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BartenderActivityDetailInfo>(
            "BartenderActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BartenderActivityDetailInfo {
    const NAME: &'static str = "BartenderActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.EFCFFFOKIIO)?;
                },
                120 => {
                    self.EFCFFFOKIIO.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.OJDLKJNDJAH)?;
                },
                24 => {
                    self.OJDLKJNDJAH.push(is.read_uint32()?);
                },
                72 => {
                    self.is_develop_module_open = is.read_bool()?;
                },
                98 => {
                    self.unlock_task_list.push(is.read_message()?);
                },
                40 => {
                    self.is_content_closed = is.read_bool()?;
                },
                58 => {
                    self.unlock_level_list.push(is.read_message()?);
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
        for value in &self.EFCFFFOKIIO {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        for value in &self.OJDLKJNDJAH {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.is_develop_module_open != false {
            my_size += 1 + 1;
        }
        for value in &self.unlock_task_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_content_closed != false {
            my_size += 1 + 1;
        }
        for value in &self.unlock_level_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EFCFFFOKIIO {
            os.write_uint32(15, *v)?;
        };
        for v in &self.OJDLKJNDJAH {
            os.write_uint32(3, *v)?;
        };
        if self.is_develop_module_open != false {
            os.write_bool(9, self.is_develop_module_open)?;
        }
        for v in &self.unlock_task_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.is_content_closed != false {
            os.write_bool(5, self.is_content_closed)?;
        }
        for v in &self.unlock_level_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> BartenderActivityDetailInfo {
        BartenderActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.EFCFFFOKIIO.clear();
        self.OJDLKJNDJAH.clear();
        self.is_develop_module_open = false;
        self.unlock_task_list.clear();
        self.is_content_closed = false;
        self.unlock_level_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BartenderActivityDetailInfo {
        static instance: BartenderActivityDetailInfo = BartenderActivityDetailInfo {
            EFCFFFOKIIO: ::std::vec::Vec::new(),
            OJDLKJNDJAH: ::std::vec::Vec::new(),
            is_develop_module_open: false,
            unlock_task_list: ::std::vec::Vec::new(),
            is_content_closed: false,
            unlock_level_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BartenderActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BartenderActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BartenderActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BartenderActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!BartenderActivityDetailInfo.proto\x1a\x17BartenderTaskInfo.proto\x1a\
    \x18BartenderLevelInfo.proto\"\xc1\x02\n\x1bBartenderActivityDetailInfo\
    \x12\x20\n\x0bEFCFFFOKIIO\x18\x0f\x20\x03(\rR\x0bEFCFFFOKIIO\x12\x20\n\
    \x0bOJDLKJNDJAH\x18\x03\x20\x03(\rR\x0bOJDLKJNDJAH\x123\n\x16is_develop_\
    module_open\x18\t\x20\x01(\x08R\x13isDevelopModuleOpen\x12<\n\x10unlock_\
    task_list\x18\x0c\x20\x03(\x0b2\x12.BartenderTaskInfoR\x0eunlockTaskList\
    \x12*\n\x11is_content_closed\x18\x05\x20\x01(\x08R\x0fisContentClosed\
    \x12?\n\x11unlock_level_list\x18\x07\x20\x03(\x0b2\x13.BartenderLevelInf\
    oR\x0funlockLevelListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::BartenderTaskInfo::file_descriptor().clone());
            deps.push(super::BartenderLevelInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BartenderActivityDetailInfo::generated_message_descriptor_data());
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
