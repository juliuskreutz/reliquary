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

//! Generated file from `DeliveryActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DeliveryActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DeliveryActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:DeliveryActivityDetailInfo.finished_delivery_quest_index)
    pub finished_delivery_quest_index: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DeliveryActivityDetailInfo.day_index)
    pub day_index: u32,
    // @@protoc_insertion_point(field:DeliveryActivityDetailInfo.is_taken_reward)
    pub is_taken_reward: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DeliveryActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DeliveryActivityDetailInfo {
    fn default() -> &'a DeliveryActivityDetailInfo {
        <DeliveryActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl DeliveryActivityDetailInfo {
    pub fn new() -> DeliveryActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "finished_delivery_quest_index",
            |m: &DeliveryActivityDetailInfo| { &m.finished_delivery_quest_index },
            |m: &mut DeliveryActivityDetailInfo| { &mut m.finished_delivery_quest_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "day_index",
            |m: &DeliveryActivityDetailInfo| { &m.day_index },
            |m: &mut DeliveryActivityDetailInfo| { &mut m.day_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_taken_reward",
            |m: &DeliveryActivityDetailInfo| { &m.is_taken_reward },
            |m: &mut DeliveryActivityDetailInfo| { &mut m.is_taken_reward },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DeliveryActivityDetailInfo>(
            "DeliveryActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DeliveryActivityDetailInfo {
    const NAME: &'static str = "DeliveryActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.finished_delivery_quest_index)?;
                },
                112 => {
                    self.finished_delivery_quest_index.push(is.read_uint32()?);
                },
                104 => {
                    self.day_index = is.read_uint32()?;
                },
                48 => {
                    self.is_taken_reward = is.read_bool()?;
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
        for value in &self.finished_delivery_quest_index {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if self.day_index != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.day_index);
        }
        if self.is_taken_reward != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.finished_delivery_quest_index {
            os.write_uint32(14, *v)?;
        };
        if self.day_index != 0 {
            os.write_uint32(13, self.day_index)?;
        }
        if self.is_taken_reward != false {
            os.write_bool(6, self.is_taken_reward)?;
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

    fn new() -> DeliveryActivityDetailInfo {
        DeliveryActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.finished_delivery_quest_index.clear();
        self.day_index = 0;
        self.is_taken_reward = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DeliveryActivityDetailInfo {
        static instance: DeliveryActivityDetailInfo = DeliveryActivityDetailInfo {
            finished_delivery_quest_index: ::std::vec::Vec::new(),
            day_index: 0,
            is_taken_reward: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DeliveryActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DeliveryActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DeliveryActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeliveryActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20DeliveryActivityDetailInfo.proto\"\xa4\x01\n\x1aDeliveryActivityDe\
    tailInfo\x12A\n\x1dfinished_delivery_quest_index\x18\x0e\x20\x03(\rR\x1a\
    finishedDeliveryQuestIndex\x12\x1b\n\tday_index\x18\r\x20\x01(\rR\x08day\
    Index\x12&\n\x0fis_taken_reward\x18\x06\x20\x01(\x08R\risTakenRewardB\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DeliveryActivityDetailInfo::generated_message_descriptor_data());
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
