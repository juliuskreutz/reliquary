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

//! Generated file from `BuyGoodsRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BuyGoodsRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BuyGoodsRsp {
    // message fields
    // @@protoc_insertion_point(field:BuyGoodsRsp.goods_list)
    pub goods_list: ::std::vec::Vec<super::ShopGoods::ShopGoods>,
    // @@protoc_insertion_point(field:BuyGoodsRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:BuyGoodsRsp.goods)
    pub goods: ::protobuf::MessageField<super::ShopGoods::ShopGoods>,
    // @@protoc_insertion_point(field:BuyGoodsRsp.buy_count)
    pub buy_count: u32,
    // @@protoc_insertion_point(field:BuyGoodsRsp.shop_type)
    pub shop_type: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BuyGoodsRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BuyGoodsRsp {
    fn default() -> &'a BuyGoodsRsp {
        <BuyGoodsRsp as ::protobuf::Message>::default_instance()
    }
}

impl BuyGoodsRsp {
    pub fn new() -> BuyGoodsRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "goods_list",
            |m: &BuyGoodsRsp| { &m.goods_list },
            |m: &mut BuyGoodsRsp| { &mut m.goods_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &BuyGoodsRsp| { &m.retcode },
            |m: &mut BuyGoodsRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ShopGoods::ShopGoods>(
            "goods",
            |m: &BuyGoodsRsp| { &m.goods },
            |m: &mut BuyGoodsRsp| { &mut m.goods },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buy_count",
            |m: &BuyGoodsRsp| { &m.buy_count },
            |m: &mut BuyGoodsRsp| { &mut m.buy_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "shop_type",
            |m: &BuyGoodsRsp| { &m.shop_type },
            |m: &mut BuyGoodsRsp| { &mut m.shop_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BuyGoodsRsp>(
            "BuyGoodsRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BuyGoodsRsp {
    const NAME: &'static str = "BuyGoodsRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.goods_list.push(is.read_message()?);
                },
                32 => {
                    self.retcode = is.read_int32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.goods)?;
                },
                96 => {
                    self.buy_count = is.read_uint32()?;
                },
                112 => {
                    self.shop_type = is.read_uint32()?;
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
        for value in &self.goods_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.retcode);
        }
        if let Some(v) = self.goods.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.buy_count != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.buy_count);
        }
        if self.shop_type != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.shop_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.goods_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.retcode != 0 {
            os.write_int32(4, self.retcode)?;
        }
        if let Some(v) = self.goods.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.buy_count != 0 {
            os.write_uint32(12, self.buy_count)?;
        }
        if self.shop_type != 0 {
            os.write_uint32(14, self.shop_type)?;
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

    fn new() -> BuyGoodsRsp {
        BuyGoodsRsp::new()
    }

    fn clear(&mut self) {
        self.goods_list.clear();
        self.retcode = 0;
        self.goods.clear();
        self.buy_count = 0;
        self.shop_type = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BuyGoodsRsp {
        static instance: BuyGoodsRsp = BuyGoodsRsp {
            goods_list: ::std::vec::Vec::new(),
            retcode: 0,
            goods: ::protobuf::MessageField::none(),
            buy_count: 0,
            shop_type: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BuyGoodsRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BuyGoodsRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BuyGoodsRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BuyGoodsRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BuyGoodsRsp.proto\x1a\x0fShopGoods.proto\"\xae\x01\n\x0bBuyGoodsRs\
    p\x12)\n\ngoods_list\x18\x01\x20\x03(\x0b2\n.ShopGoodsR\tgoodsList\x12\
    \x18\n\x07retcode\x18\x04\x20\x01(\x05R\x07retcode\x12\x20\n\x05goods\
    \x18\t\x20\x01(\x0b2\n.ShopGoodsR\x05goods\x12\x1b\n\tbuy_count\x18\x0c\
    \x20\x01(\rR\x08buyCount\x12\x1b\n\tshop_type\x18\x0e\x20\x01(\rR\x08sho\
    pTypeB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::ShopGoods::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BuyGoodsRsp::generated_message_descriptor_data());
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