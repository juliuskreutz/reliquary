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

//! Generated file from `PlayerCompoundMaterialBoostRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerCompoundMaterialBoostRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerCompoundMaterialBoostRsp {
    // message fields
    // @@protoc_insertion_point(field:PlayerCompoundMaterialBoostRsp.compoundQueueDataList)
    pub compoundQueueDataList: ::std::vec::Vec<super::CompoundQueueData::CompoundQueueData>,
    // @@protoc_insertion_point(field:PlayerCompoundMaterialBoostRsp.take_status)
    pub take_status: ::protobuf::EnumOrUnknown<super::CompoundBoostTakeStatusType::CompoundBoostTakeStatusType>,
    // @@protoc_insertion_point(field:PlayerCompoundMaterialBoostRsp.take_item_list)
    pub take_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:PlayerCompoundMaterialBoostRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerCompoundMaterialBoostRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerCompoundMaterialBoostRsp {
    fn default() -> &'a PlayerCompoundMaterialBoostRsp {
        <PlayerCompoundMaterialBoostRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlayerCompoundMaterialBoostRsp {
    pub fn new() -> PlayerCompoundMaterialBoostRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "compoundQueueDataList",
            |m: &PlayerCompoundMaterialBoostRsp| { &m.compoundQueueDataList },
            |m: &mut PlayerCompoundMaterialBoostRsp| { &mut m.compoundQueueDataList },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "take_status",
            |m: &PlayerCompoundMaterialBoostRsp| { &m.take_status },
            |m: &mut PlayerCompoundMaterialBoostRsp| { &mut m.take_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "take_item_list",
            |m: &PlayerCompoundMaterialBoostRsp| { &m.take_item_list },
            |m: &mut PlayerCompoundMaterialBoostRsp| { &mut m.take_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlayerCompoundMaterialBoostRsp| { &m.retcode },
            |m: &mut PlayerCompoundMaterialBoostRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerCompoundMaterialBoostRsp>(
            "PlayerCompoundMaterialBoostRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerCompoundMaterialBoostRsp {
    const NAME: &'static str = "PlayerCompoundMaterialBoostRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.compoundQueueDataList.push(is.read_message()?);
                },
                72 => {
                    self.take_status = is.read_enum_or_unknown()?;
                },
                98 => {
                    self.take_item_list.push(is.read_message()?);
                },
                32 => {
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
        for value in &self.compoundQueueDataList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.take_status != ::protobuf::EnumOrUnknown::new(super::CompoundBoostTakeStatusType::CompoundBoostTakeStatusType::COMPOUND_BOOST_TAKE_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(9, self.take_status.value());
        }
        for value in &self.take_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.compoundQueueDataList {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.take_status != ::protobuf::EnumOrUnknown::new(super::CompoundBoostTakeStatusType::CompoundBoostTakeStatusType::COMPOUND_BOOST_TAKE_STATUS_NONE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.take_status))?;
        }
        for v in &self.take_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.retcode != 0 {
            os.write_int32(4, self.retcode)?;
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

    fn new() -> PlayerCompoundMaterialBoostRsp {
        PlayerCompoundMaterialBoostRsp::new()
    }

    fn clear(&mut self) {
        self.compoundQueueDataList.clear();
        self.take_status = ::protobuf::EnumOrUnknown::new(super::CompoundBoostTakeStatusType::CompoundBoostTakeStatusType::COMPOUND_BOOST_TAKE_STATUS_NONE);
        self.take_item_list.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerCompoundMaterialBoostRsp {
        static instance: PlayerCompoundMaterialBoostRsp = PlayerCompoundMaterialBoostRsp {
            compoundQueueDataList: ::std::vec::Vec::new(),
            take_status: ::protobuf::EnumOrUnknown::from_i32(0),
            take_item_list: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerCompoundMaterialBoostRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerCompoundMaterialBoostRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerCompoundMaterialBoostRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerCompoundMaterialBoostRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$PlayerCompoundMaterialBoostRsp.proto\x1a\x17CompoundQueueData.proto\
    \x1a!CompoundBoostTakeStatusType.proto\x1a\x0fItemParam.proto\"\xf5\x01\
    \n\x1ePlayerCompoundMaterialBoostRsp\x12H\n\x15compoundQueueDataList\x18\
    \x0f\x20\x03(\x0b2\x12.CompoundQueueDataR\x15compoundQueueDataList\x12=\
    \n\x0btake_status\x18\t\x20\x01(\x0e2\x1c.CompoundBoostTakeStatusTypeR\n\
    takeStatus\x120\n\x0etake_item_list\x18\x0c\x20\x03(\x0b2\n.ItemParamR\
    \x0ctakeItemList\x12\x18\n\x07retcode\x18\x04\x20\x01(\x05R\x07retcodeB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CompoundQueueData::file_descriptor().clone());
            deps.push(super::CompoundBoostTakeStatusType::file_descriptor().clone());
            deps.push(super::ItemParam::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerCompoundMaterialBoostRsp::generated_message_descriptor_data());
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