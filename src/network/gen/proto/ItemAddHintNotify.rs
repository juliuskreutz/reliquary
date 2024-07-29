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

//! Generated file from `ItemAddHintNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ItemAddHintNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ItemAddHintNotify {
    // message fields
    // @@protoc_insertion_point(field:ItemAddHintNotify.FMLDPGOOCPG)
    pub FMLDPGOOCPG: bool,
    // @@protoc_insertion_point(field:ItemAddHintNotify.overflow_transformed_item_list)
    pub overflow_transformed_item_list: ::std::vec::Vec<super::ItemHint::ItemHint>,
    // @@protoc_insertion_point(field:ItemAddHintNotify.reason)
    pub reason: u32,
    // @@protoc_insertion_point(field:ItemAddHintNotify.position)
    pub position: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ItemAddHintNotify.quest_id)
    pub quest_id: u32,
    // @@protoc_insertion_point(field:ItemAddHintNotify.item_list)
    pub item_list: ::std::vec::Vec<super::ItemHint::ItemHint>,
    // @@protoc_insertion_point(field:ItemAddHintNotify.BJFIEHFPADD)
    pub BJFIEHFPADD: bool,
    // @@protoc_insertion_point(field:ItemAddHintNotify.DAKGABMHAHP)
    pub DAKGABMHAHP: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ItemAddHintNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ItemAddHintNotify {
    fn default() -> &'a ItemAddHintNotify {
        <ItemAddHintNotify as ::protobuf::Message>::default_instance()
    }
}

impl ItemAddHintNotify {
    pub fn new() -> ItemAddHintNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMLDPGOOCPG",
            |m: &ItemAddHintNotify| { &m.FMLDPGOOCPG },
            |m: &mut ItemAddHintNotify| { &mut m.FMLDPGOOCPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "overflow_transformed_item_list",
            |m: &ItemAddHintNotify| { &m.overflow_transformed_item_list },
            |m: &mut ItemAddHintNotify| { &mut m.overflow_transformed_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reason",
            |m: &ItemAddHintNotify| { &m.reason },
            |m: &mut ItemAddHintNotify| { &mut m.reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "position",
            |m: &ItemAddHintNotify| { &m.position },
            |m: &mut ItemAddHintNotify| { &mut m.position },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "quest_id",
            |m: &ItemAddHintNotify| { &m.quest_id },
            |m: &mut ItemAddHintNotify| { &mut m.quest_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_list",
            |m: &ItemAddHintNotify| { &m.item_list },
            |m: &mut ItemAddHintNotify| { &mut m.item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJFIEHFPADD",
            |m: &ItemAddHintNotify| { &m.BJFIEHFPADD },
            |m: &mut ItemAddHintNotify| { &mut m.BJFIEHFPADD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAKGABMHAHP",
            |m: &ItemAddHintNotify| { &m.DAKGABMHAHP },
            |m: &mut ItemAddHintNotify| { &mut m.DAKGABMHAHP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ItemAddHintNotify>(
            "ItemAddHintNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ItemAddHintNotify {
    const NAME: &'static str = "ItemAddHintNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FMLDPGOOCPG = is.read_bool()?;
                },
                18 => {
                    self.overflow_transformed_item_list.push(is.read_message()?);
                },
                24 => {
                    self.reason = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.position)?;
                },
                64 => {
                    self.quest_id = is.read_uint32()?;
                },
                82 => {
                    self.item_list.push(is.read_message()?);
                },
                96 => {
                    self.BJFIEHFPADD = is.read_bool()?;
                },
                112 => {
                    self.DAKGABMHAHP = is.read_bool()?;
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
        if self.FMLDPGOOCPG != false {
            my_size += 1 + 1;
        }
        for value in &self.overflow_transformed_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.reason != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.reason);
        }
        if let Some(v) = self.position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.quest_id);
        }
        for value in &self.item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BJFIEHFPADD != false {
            my_size += 1 + 1;
        }
        if self.DAKGABMHAHP != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FMLDPGOOCPG != false {
            os.write_bool(1, self.FMLDPGOOCPG)?;
        }
        for v in &self.overflow_transformed_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.reason != 0 {
            os.write_uint32(3, self.reason)?;
        }
        if let Some(v) = self.position.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.quest_id != 0 {
            os.write_uint32(8, self.quest_id)?;
        }
        for v in &self.item_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.BJFIEHFPADD != false {
            os.write_bool(12, self.BJFIEHFPADD)?;
        }
        if self.DAKGABMHAHP != false {
            os.write_bool(14, self.DAKGABMHAHP)?;
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

    fn new() -> ItemAddHintNotify {
        ItemAddHintNotify::new()
    }

    fn clear(&mut self) {
        self.FMLDPGOOCPG = false;
        self.overflow_transformed_item_list.clear();
        self.reason = 0;
        self.position.clear();
        self.quest_id = 0;
        self.item_list.clear();
        self.BJFIEHFPADD = false;
        self.DAKGABMHAHP = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ItemAddHintNotify {
        static instance: ItemAddHintNotify = ItemAddHintNotify {
            FMLDPGOOCPG: false,
            overflow_transformed_item_list: ::std::vec::Vec::new(),
            reason: 0,
            position: ::protobuf::MessageField::none(),
            quest_id: 0,
            item_list: ::std::vec::Vec::new(),
            BJFIEHFPADD: false,
            DAKGABMHAHP: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ItemAddHintNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ItemAddHintNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ItemAddHintNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ItemAddHintNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ItemAddHintNotify.proto\x1a\x0eItemHint.proto\x1a\x0cVector.proto\
    \"\xc9\x02\n\x11ItemAddHintNotify\x12\x20\n\x0bFMLDPGOOCPG\x18\x01\x20\
    \x01(\x08R\x0bFMLDPGOOCPG\x12N\n\x1eoverflow_transformed_item_list\x18\
    \x02\x20\x03(\x0b2\t.ItemHintR\x1boverflowTransformedItemList\x12\x16\n\
    \x06reason\x18\x03\x20\x01(\rR\x06reason\x12#\n\x08position\x18\x04\x20\
    \x01(\x0b2\x07.VectorR\x08position\x12\x19\n\x08quest_id\x18\x08\x20\x01\
    (\rR\x07questId\x12&\n\titem_list\x18\n\x20\x03(\x0b2\t.ItemHintR\x08ite\
    mList\x12\x20\n\x0bBJFIEHFPADD\x18\x0c\x20\x01(\x08R\x0bBJFIEHFPADD\x12\
    \x20\n\x0bDAKGABMHAHP\x18\x0e\x20\x01(\x08R\x0bDAKGABMHAHPB\x1b\n\x19emu\
    .grasscutter.net.protob\x06proto3\
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
            deps.push(super::ItemHint::file_descriptor().clone());
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ItemAddHintNotify::generated_message_descriptor_data());
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
