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

//! Generated file from `AddNoGachaAvatarCardTransferItem.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AddNoGachaAvatarCardTransferItem)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddNoGachaAvatarCardTransferItem {
    // message fields
    // @@protoc_insertion_point(field:AddNoGachaAvatarCardTransferItem.count)
    pub count: u32,
    // @@protoc_insertion_point(field:AddNoGachaAvatarCardTransferItem.item_id)
    pub item_id: u32,
    // @@protoc_insertion_point(field:AddNoGachaAvatarCardTransferItem.is_new)
    pub is_new: bool,
    // special fields
    // @@protoc_insertion_point(special_field:AddNoGachaAvatarCardTransferItem.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddNoGachaAvatarCardTransferItem {
    fn default() -> &'a AddNoGachaAvatarCardTransferItem {
        <AddNoGachaAvatarCardTransferItem as ::protobuf::Message>::default_instance()
    }
}

impl AddNoGachaAvatarCardTransferItem {
    pub fn new() -> AddNoGachaAvatarCardTransferItem {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &AddNoGachaAvatarCardTransferItem| { &m.count },
            |m: &mut AddNoGachaAvatarCardTransferItem| { &mut m.count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_id",
            |m: &AddNoGachaAvatarCardTransferItem| { &m.item_id },
            |m: &mut AddNoGachaAvatarCardTransferItem| { &mut m.item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_new",
            |m: &AddNoGachaAvatarCardTransferItem| { &m.is_new },
            |m: &mut AddNoGachaAvatarCardTransferItem| { &mut m.is_new },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddNoGachaAvatarCardTransferItem>(
            "AddNoGachaAvatarCardTransferItem",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddNoGachaAvatarCardTransferItem {
    const NAME: &'static str = "AddNoGachaAvatarCardTransferItem";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.count = is.read_uint32()?;
                },
                120 => {
                    self.item_id = is.read_uint32()?;
                },
                104 => {
                    self.is_new = is.read_bool()?;
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
        if self.count != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.count);
        }
        if self.item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.item_id);
        }
        if self.is_new != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.count != 0 {
            os.write_uint32(1, self.count)?;
        }
        if self.item_id != 0 {
            os.write_uint32(15, self.item_id)?;
        }
        if self.is_new != false {
            os.write_bool(13, self.is_new)?;
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

    fn new() -> AddNoGachaAvatarCardTransferItem {
        AddNoGachaAvatarCardTransferItem::new()
    }

    fn clear(&mut self) {
        self.count = 0;
        self.item_id = 0;
        self.is_new = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddNoGachaAvatarCardTransferItem {
        static instance: AddNoGachaAvatarCardTransferItem = AddNoGachaAvatarCardTransferItem {
            count: 0,
            item_id: 0,
            is_new: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddNoGachaAvatarCardTransferItem {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddNoGachaAvatarCardTransferItem").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddNoGachaAvatarCardTransferItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddNoGachaAvatarCardTransferItem {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&AddNoGachaAvatarCardTransferItem.proto\"h\n\x20AddNoGachaAvatarCardTr\
    ansferItem\x12\x14\n\x05count\x18\x01\x20\x01(\rR\x05count\x12\x17\n\x07\
    item_id\x18\x0f\x20\x01(\rR\x06itemId\x12\x15\n\x06is_new\x18\r\x20\x01(\
    \x08R\x05isNewB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(AddNoGachaAvatarCardTransferItem::generated_message_descriptor_data());
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