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

//! Generated file from `GatherGadgetInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GatherGadgetInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GatherGadgetInfo {
    // message fields
    // @@protoc_insertion_point(field:GatherGadgetInfo.item_id)
    pub item_id: u32,
    // @@protoc_insertion_point(field:GatherGadgetInfo.is_forbid_guest)
    pub is_forbid_guest: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GatherGadgetInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GatherGadgetInfo {
    fn default() -> &'a GatherGadgetInfo {
        <GatherGadgetInfo as ::protobuf::Message>::default_instance()
    }
}

impl GatherGadgetInfo {
    pub fn new() -> GatherGadgetInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_id",
            |m: &GatherGadgetInfo| { &m.item_id },
            |m: &mut GatherGadgetInfo| { &mut m.item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_forbid_guest",
            |m: &GatherGadgetInfo| { &m.is_forbid_guest },
            |m: &mut GatherGadgetInfo| { &mut m.is_forbid_guest },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GatherGadgetInfo>(
            "GatherGadgetInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GatherGadgetInfo {
    const NAME: &'static str = "GatherGadgetInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.item_id = is.read_uint32()?;
                },
                16 => {
                    self.is_forbid_guest = is.read_bool()?;
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
        if self.item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.item_id);
        }
        if self.is_forbid_guest != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.item_id != 0 {
            os.write_uint32(1, self.item_id)?;
        }
        if self.is_forbid_guest != false {
            os.write_bool(2, self.is_forbid_guest)?;
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

    fn new() -> GatherGadgetInfo {
        GatherGadgetInfo::new()
    }

    fn clear(&mut self) {
        self.item_id = 0;
        self.is_forbid_guest = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GatherGadgetInfo {
        static instance: GatherGadgetInfo = GatherGadgetInfo {
            item_id: 0,
            is_forbid_guest: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GatherGadgetInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GatherGadgetInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GatherGadgetInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GatherGadgetInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16GatherGadgetInfo.proto\"S\n\x10GatherGadgetInfo\x12\x17\n\x07item_\
    id\x18\x01\x20\x01(\rR\x06itemId\x12&\n\x0fis_forbid_guest\x18\x02\x20\
    \x01(\x08R\risForbidGuestB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(GatherGadgetInfo::generated_message_descriptor_data());
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