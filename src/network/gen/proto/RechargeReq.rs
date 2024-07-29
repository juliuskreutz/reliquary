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

//! Generated file from `RechargeReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RechargeReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RechargeReq {
    // message fields
    // @@protoc_insertion_point(field:RechargeReq.play_product)
    pub play_product: ::protobuf::MessageField<super::PlayProduct::PlayProduct>,
    // @@protoc_insertion_point(field:RechargeReq.mcoin_product)
    pub mcoin_product: ::protobuf::MessageField<super::ShopMcoinProduct::ShopMcoinProduct>,
    // @@protoc_insertion_point(field:RechargeReq.card_product)
    pub card_product: ::protobuf::MessageField<super::ShopCardProduct::ShopCardProduct>,
    // @@protoc_insertion_point(field:RechargeReq.concert_product)
    pub concert_product: ::protobuf::MessageField<super::ShopConcertProduct::ShopConcertProduct>,
    // special fields
    // @@protoc_insertion_point(special_field:RechargeReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RechargeReq {
    fn default() -> &'a RechargeReq {
        <RechargeReq as ::protobuf::Message>::default_instance()
    }
}

impl RechargeReq {
    pub fn new() -> RechargeReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PlayProduct::PlayProduct>(
            "play_product",
            |m: &RechargeReq| { &m.play_product },
            |m: &mut RechargeReq| { &mut m.play_product },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ShopMcoinProduct::ShopMcoinProduct>(
            "mcoin_product",
            |m: &RechargeReq| { &m.mcoin_product },
            |m: &mut RechargeReq| { &mut m.mcoin_product },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ShopCardProduct::ShopCardProduct>(
            "card_product",
            |m: &RechargeReq| { &m.card_product },
            |m: &mut RechargeReq| { &mut m.card_product },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ShopConcertProduct::ShopConcertProduct>(
            "concert_product",
            |m: &RechargeReq| { &m.concert_product },
            |m: &mut RechargeReq| { &mut m.concert_product },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RechargeReq>(
            "RechargeReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RechargeReq {
    const NAME: &'static str = "RechargeReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.play_product)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.mcoin_product)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.card_product)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.concert_product)?;
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
        if let Some(v) = self.play_product.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.mcoin_product.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.card_product.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.concert_product.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.play_product.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.mcoin_product.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.card_product.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.concert_product.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> RechargeReq {
        RechargeReq::new()
    }

    fn clear(&mut self) {
        self.play_product.clear();
        self.mcoin_product.clear();
        self.card_product.clear();
        self.concert_product.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RechargeReq {
        static instance: RechargeReq = RechargeReq {
            play_product: ::protobuf::MessageField::none(),
            mcoin_product: ::protobuf::MessageField::none(),
            card_product: ::protobuf::MessageField::none(),
            concert_product: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RechargeReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RechargeReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RechargeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RechargeReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11RechargeReq.proto\x1a\x11PlayProduct.proto\x1a\x15ShopCardProduct.\
    proto\x1a\x18ShopConcertProduct.proto\x1a\x16ShopMcoinProduct.proto\"\
    \xe9\x01\n\x0bRechargeReq\x12/\n\x0cplay_product\x18\x04\x20\x01(\x0b2\
    \x0c.PlayProductR\x0bplayProduct\x126\n\rmcoin_product\x18\x07\x20\x01(\
    \x0b2\x11.ShopMcoinProductR\x0cmcoinProduct\x123\n\x0ccard_product\x18\
    \x0b\x20\x01(\x0b2\x10.ShopCardProductR\x0bcardProduct\x12<\n\x0fconcert\
    _product\x18\x0f\x20\x01(\x0b2\x13.ShopConcertProductR\x0econcertProduct\
    B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::PlayProduct::file_descriptor().clone());
            deps.push(super::ShopCardProduct::file_descriptor().clone());
            deps.push(super::ShopConcertProduct::file_descriptor().clone());
            deps.push(super::ShopMcoinProduct::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RechargeReq::generated_message_descriptor_data());
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