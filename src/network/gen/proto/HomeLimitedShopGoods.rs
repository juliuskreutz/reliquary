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

//! Generated file from `HomeLimitedShopGoods.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HomeLimitedShopGoods)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeLimitedShopGoods {
    // message fields
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.cost_item_list)
    pub cost_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.NPBGGAMEDJG)
    pub NPBGGAMEDJG: u32,
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.bought_num)
    pub bought_num: u32,
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.NODBIKCALJI)
    pub NODBIKCALJI: u32,
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.goods_item)
    pub goods_item: ::protobuf::MessageField<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:HomeLimitedShopGoods.JOMBNPMFHGG)
    pub JOMBNPMFHGG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HomeLimitedShopGoods.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeLimitedShopGoods {
    fn default() -> &'a HomeLimitedShopGoods {
        <HomeLimitedShopGoods as ::protobuf::Message>::default_instance()
    }
}

impl HomeLimitedShopGoods {
    pub fn new() -> HomeLimitedShopGoods {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "cost_item_list",
            |m: &HomeLimitedShopGoods| { &m.cost_item_list },
            |m: &mut HomeLimitedShopGoods| { &mut m.cost_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPBGGAMEDJG",
            |m: &HomeLimitedShopGoods| { &m.NPBGGAMEDJG },
            |m: &mut HomeLimitedShopGoods| { &mut m.NPBGGAMEDJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bought_num",
            |m: &HomeLimitedShopGoods| { &m.bought_num },
            |m: &mut HomeLimitedShopGoods| { &mut m.bought_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NODBIKCALJI",
            |m: &HomeLimitedShopGoods| { &m.NODBIKCALJI },
            |m: &mut HomeLimitedShopGoods| { &mut m.NODBIKCALJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemParam::ItemParam>(
            "goods_item",
            |m: &HomeLimitedShopGoods| { &m.goods_item },
            |m: &mut HomeLimitedShopGoods| { &mut m.goods_item },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JOMBNPMFHGG",
            |m: &HomeLimitedShopGoods| { &m.JOMBNPMFHGG },
            |m: &mut HomeLimitedShopGoods| { &mut m.JOMBNPMFHGG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeLimitedShopGoods>(
            "HomeLimitedShopGoods",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeLimitedShopGoods {
    const NAME: &'static str = "HomeLimitedShopGoods";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.cost_item_list.push(is.read_message()?);
                },
                32 => {
                    self.NPBGGAMEDJG = is.read_uint32()?;
                },
                48 => {
                    self.bought_num = is.read_uint32()?;
                },
                24 => {
                    self.NODBIKCALJI = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.goods_item)?;
                },
                80 => {
                    self.JOMBNPMFHGG = is.read_uint32()?;
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
        for value in &self.cost_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NPBGGAMEDJG != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NPBGGAMEDJG);
        }
        if self.bought_num != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.bought_num);
        }
        if self.NODBIKCALJI != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NODBIKCALJI);
        }
        if let Some(v) = self.goods_item.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.JOMBNPMFHGG != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.JOMBNPMFHGG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.cost_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.NPBGGAMEDJG != 0 {
            os.write_uint32(4, self.NPBGGAMEDJG)?;
        }
        if self.bought_num != 0 {
            os.write_uint32(6, self.bought_num)?;
        }
        if self.NODBIKCALJI != 0 {
            os.write_uint32(3, self.NODBIKCALJI)?;
        }
        if let Some(v) = self.goods_item.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.JOMBNPMFHGG != 0 {
            os.write_uint32(10, self.JOMBNPMFHGG)?;
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

    fn new() -> HomeLimitedShopGoods {
        HomeLimitedShopGoods::new()
    }

    fn clear(&mut self) {
        self.cost_item_list.clear();
        self.NPBGGAMEDJG = 0;
        self.bought_num = 0;
        self.NODBIKCALJI = 0;
        self.goods_item.clear();
        self.JOMBNPMFHGG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeLimitedShopGoods {
        static instance: HomeLimitedShopGoods = HomeLimitedShopGoods {
            cost_item_list: ::std::vec::Vec::new(),
            NPBGGAMEDJG: 0,
            bought_num: 0,
            NODBIKCALJI: 0,
            goods_item: ::protobuf::MessageField::none(),
            JOMBNPMFHGG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeLimitedShopGoods {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeLimitedShopGoods").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeLimitedShopGoods {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeLimitedShopGoods {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aHomeLimitedShopGoods.proto\x1a\x0fItemParam.proto\"\xf8\x01\n\x14H\
    omeLimitedShopGoods\x120\n\x0ecost_item_list\x18\x0f\x20\x03(\x0b2\n.Ite\
    mParamR\x0ccostItemList\x12\x20\n\x0bNPBGGAMEDJG\x18\x04\x20\x01(\rR\x0b\
    NPBGGAMEDJG\x12\x1d\n\nbought_num\x18\x06\x20\x01(\rR\tboughtNum\x12\x20\
    \n\x0bNODBIKCALJI\x18\x03\x20\x01(\rR\x0bNODBIKCALJI\x12)\n\ngoods_item\
    \x18\x02\x20\x01(\x0b2\n.ItemParamR\tgoodsItem\x12\x20\n\x0bJOMBNPMFHGG\
    \x18\n\x20\x01(\rR\x0bJOMBNPMFHGGB\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            messages.push(HomeLimitedShopGoods::generated_message_descriptor_data());
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