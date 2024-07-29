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

//! Generated file from `BattlePassBuySuccNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattlePassBuySuccNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattlePassBuySuccNotify {
    // message fields
    // @@protoc_insertion_point(field:BattlePassBuySuccNotify.schedule_id)
    pub schedule_id: u32,
    // @@protoc_insertion_point(field:BattlePassBuySuccNotify.HMKOKJKINCG)
    pub HMKOKJKINCG: u32,
    // @@protoc_insertion_point(field:BattlePassBuySuccNotify.LAKIBHGHOGE)
    pub LAKIBHGHOGE: u32,
    // @@protoc_insertion_point(field:BattlePassBuySuccNotify.item_list)
    pub item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // special fields
    // @@protoc_insertion_point(special_field:BattlePassBuySuccNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattlePassBuySuccNotify {
    fn default() -> &'a BattlePassBuySuccNotify {
        <BattlePassBuySuccNotify as ::protobuf::Message>::default_instance()
    }
}

impl BattlePassBuySuccNotify {
    pub fn new() -> BattlePassBuySuccNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "schedule_id",
            |m: &BattlePassBuySuccNotify| { &m.schedule_id },
            |m: &mut BattlePassBuySuccNotify| { &mut m.schedule_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMKOKJKINCG",
            |m: &BattlePassBuySuccNotify| { &m.HMKOKJKINCG },
            |m: &mut BattlePassBuySuccNotify| { &mut m.HMKOKJKINCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LAKIBHGHOGE",
            |m: &BattlePassBuySuccNotify| { &m.LAKIBHGHOGE },
            |m: &mut BattlePassBuySuccNotify| { &mut m.LAKIBHGHOGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_list",
            |m: &BattlePassBuySuccNotify| { &m.item_list },
            |m: &mut BattlePassBuySuccNotify| { &mut m.item_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattlePassBuySuccNotify>(
            "BattlePassBuySuccNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattlePassBuySuccNotify {
    const NAME: &'static str = "BattlePassBuySuccNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.schedule_id = is.read_uint32()?;
                },
                48 => {
                    self.HMKOKJKINCG = is.read_uint32()?;
                },
                56 => {
                    self.LAKIBHGHOGE = is.read_uint32()?;
                },
                90 => {
                    self.item_list.push(is.read_message()?);
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
        if self.schedule_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.schedule_id);
        }
        if self.HMKOKJKINCG != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.HMKOKJKINCG);
        }
        if self.LAKIBHGHOGE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LAKIBHGHOGE);
        }
        for value in &self.item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.schedule_id != 0 {
            os.write_uint32(1, self.schedule_id)?;
        }
        if self.HMKOKJKINCG != 0 {
            os.write_uint32(6, self.HMKOKJKINCG)?;
        }
        if self.LAKIBHGHOGE != 0 {
            os.write_uint32(7, self.LAKIBHGHOGE)?;
        }
        for v in &self.item_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> BattlePassBuySuccNotify {
        BattlePassBuySuccNotify::new()
    }

    fn clear(&mut self) {
        self.schedule_id = 0;
        self.HMKOKJKINCG = 0;
        self.LAKIBHGHOGE = 0;
        self.item_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattlePassBuySuccNotify {
        static instance: BattlePassBuySuccNotify = BattlePassBuySuccNotify {
            schedule_id: 0,
            HMKOKJKINCG: 0,
            LAKIBHGHOGE: 0,
            item_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattlePassBuySuccNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattlePassBuySuccNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattlePassBuySuccNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattlePassBuySuccNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dBattlePassBuySuccNotify.proto\x1a\x0fItemParam.proto\"\xa7\x01\n\
    \x17BattlePassBuySuccNotify\x12\x1f\n\x0bschedule_id\x18\x01\x20\x01(\rR\
    \nscheduleId\x12\x20\n\x0bHMKOKJKINCG\x18\x06\x20\x01(\rR\x0bHMKOKJKINCG\
    \x12\x20\n\x0bLAKIBHGHOGE\x18\x07\x20\x01(\rR\x0bLAKIBHGHOGE\x12'\n\tite\
    m_list\x18\x0b\x20\x03(\x0b2\n.ItemParamR\x08itemListB\x1b\n\x19emu.gras\
    scutter.net.protob\x06proto3\
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
            deps.push(super::ItemParam::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattlePassBuySuccNotify::generated_message_descriptor_data());
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
