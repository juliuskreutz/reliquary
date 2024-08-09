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

//! Generated file from `RankUpAvatarCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RankUpAvatarCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RankUpAvatarCsReq {
    // message fields
    // @@protoc_insertion_point(field:RankUpAvatarCsReq.item_cost_list)
    pub item_cost_list: ::protobuf::MessageField<super::ItemCostList::ItemCostList>,
    // @@protoc_insertion_point(field:RankUpAvatarCsReq.equip_avatar_id)
    pub equip_avatar_id: u32,
    // @@protoc_insertion_point(field:RankUpAvatarCsReq.rank)
    pub rank: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RankUpAvatarCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RankUpAvatarCsReq {
    fn default() -> &'a RankUpAvatarCsReq {
        <RankUpAvatarCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RankUpAvatarCsReq {
    pub fn new() -> RankUpAvatarCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostList::ItemCostList>(
            "item_cost_list",
            |m: &RankUpAvatarCsReq| { &m.item_cost_list },
            |m: &mut RankUpAvatarCsReq| { &mut m.item_cost_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "equip_avatar_id",
            |m: &RankUpAvatarCsReq| { &m.equip_avatar_id },
            |m: &mut RankUpAvatarCsReq| { &mut m.equip_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &RankUpAvatarCsReq| { &m.rank },
            |m: &mut RankUpAvatarCsReq| { &mut m.rank },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RankUpAvatarCsReq>(
            "RankUpAvatarCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RankUpAvatarCsReq {
    const NAME: &'static str = "RankUpAvatarCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.item_cost_list)?;
                },
                24 => {
                    self.equip_avatar_id = is.read_uint32()?;
                },
                104 => {
                    self.rank = is.read_uint32()?;
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
        if let Some(v) = self.item_cost_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.equip_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.equip_avatar_id);
        }
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.rank);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.item_cost_list.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.equip_avatar_id != 0 {
            os.write_uint32(3, self.equip_avatar_id)?;
        }
        if self.rank != 0 {
            os.write_uint32(13, self.rank)?;
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

    fn new() -> RankUpAvatarCsReq {
        RankUpAvatarCsReq::new()
    }

    fn clear(&mut self) {
        self.item_cost_list.clear();
        self.equip_avatar_id = 0;
        self.rank = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RankUpAvatarCsReq {
        static instance: RankUpAvatarCsReq = RankUpAvatarCsReq {
            item_cost_list: ::protobuf::MessageField::none(),
            equip_avatar_id: 0,
            rank: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RankUpAvatarCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RankUpAvatarCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RankUpAvatarCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RankUpAvatarCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17RankUpAvatarCsReq.proto\x1a\x12ItemCostList.proto\"\x84\x01\n\x11R\
    ankUpAvatarCsReq\x123\n\x0eitem_cost_list\x18\x05\x20\x01(\x0b2\r.ItemCo\
    stListR\x0citemCostList\x12&\n\x0fequip_avatar_id\x18\x03\x20\x01(\rR\re\
    quipAvatarId\x12\x12\n\x04rank\x18\r\x20\x01(\rR\x04rankB\x15\n\x13emu.l\
    unarcore.protob\x06proto3\
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
            deps.push(super::ItemCostList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RankUpAvatarCsReq::generated_message_descriptor_data());
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