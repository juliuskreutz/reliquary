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

//! Generated file from `GetBattlePassProductRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetBattlePassProductRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetBattlePassProductRsp {
    // message fields
    // @@protoc_insertion_point(field:GetBattlePassProductRsp.battle_pass_product_play_type)
    pub battle_pass_product_play_type: u32,
    // @@protoc_insertion_point(field:GetBattlePassProductRsp.cur_schedule_id)
    pub cur_schedule_id: u32,
    // @@protoc_insertion_point(field:GetBattlePassProductRsp.price_tier)
    pub price_tier: ::std::string::String,
    // @@protoc_insertion_point(field:GetBattlePassProductRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetBattlePassProductRsp.product_id)
    pub product_id: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:GetBattlePassProductRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetBattlePassProductRsp {
    fn default() -> &'a GetBattlePassProductRsp {
        <GetBattlePassProductRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetBattlePassProductRsp {
    pub fn new() -> GetBattlePassProductRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_pass_product_play_type",
            |m: &GetBattlePassProductRsp| { &m.battle_pass_product_play_type },
            |m: &mut GetBattlePassProductRsp| { &mut m.battle_pass_product_play_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_schedule_id",
            |m: &GetBattlePassProductRsp| { &m.cur_schedule_id },
            |m: &mut GetBattlePassProductRsp| { &mut m.cur_schedule_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "price_tier",
            |m: &GetBattlePassProductRsp| { &m.price_tier },
            |m: &mut GetBattlePassProductRsp| { &mut m.price_tier },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetBattlePassProductRsp| { &m.retcode },
            |m: &mut GetBattlePassProductRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "product_id",
            |m: &GetBattlePassProductRsp| { &m.product_id },
            |m: &mut GetBattlePassProductRsp| { &mut m.product_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetBattlePassProductRsp>(
            "GetBattlePassProductRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetBattlePassProductRsp {
    const NAME: &'static str = "GetBattlePassProductRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.battle_pass_product_play_type = is.read_uint32()?;
                },
                32 => {
                    self.cur_schedule_id = is.read_uint32()?;
                },
                66 => {
                    self.price_tier = is.read_string()?;
                },
                96 => {
                    self.retcode = is.read_int32()?;
                },
                114 => {
                    self.product_id = is.read_string()?;
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
        if self.battle_pass_product_play_type != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.battle_pass_product_play_type);
        }
        if self.cur_schedule_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.cur_schedule_id);
        }
        if !self.price_tier.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.price_tier);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.retcode);
        }
        if !self.product_id.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.product_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.battle_pass_product_play_type != 0 {
            os.write_uint32(2, self.battle_pass_product_play_type)?;
        }
        if self.cur_schedule_id != 0 {
            os.write_uint32(4, self.cur_schedule_id)?;
        }
        if !self.price_tier.is_empty() {
            os.write_string(8, &self.price_tier)?;
        }
        if self.retcode != 0 {
            os.write_int32(12, self.retcode)?;
        }
        if !self.product_id.is_empty() {
            os.write_string(14, &self.product_id)?;
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

    fn new() -> GetBattlePassProductRsp {
        GetBattlePassProductRsp::new()
    }

    fn clear(&mut self) {
        self.battle_pass_product_play_type = 0;
        self.cur_schedule_id = 0;
        self.price_tier.clear();
        self.retcode = 0;
        self.product_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetBattlePassProductRsp {
        static instance: GetBattlePassProductRsp = GetBattlePassProductRsp {
            battle_pass_product_play_type: 0,
            cur_schedule_id: 0,
            price_tier: ::std::string::String::new(),
            retcode: 0,
            product_id: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetBattlePassProductRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetBattlePassProductRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetBattlePassProductRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBattlePassProductRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dGetBattlePassProductRsp.proto\"\xdb\x01\n\x17GetBattlePassProductR\
    sp\x12@\n\x1dbattle_pass_product_play_type\x18\x02\x20\x01(\rR\x19battle\
    PassProductPlayType\x12&\n\x0fcur_schedule_id\x18\x04\x20\x01(\rR\rcurSc\
    heduleId\x12\x1d\n\nprice_tier\x18\x08\x20\x01(\tR\tpriceTier\x12\x18\n\
    \x07retcode\x18\x0c\x20\x01(\x05R\x07retcode\x12\x1d\n\nproduct_id\x18\
    \x0e\x20\x01(\tR\tproductIdB\x1b\n\x19emu.grasscutter.net.protob\x06prot\
    o3\
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
            messages.push(GetBattlePassProductRsp::generated_message_descriptor_data());
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
