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

//! Generated file from `WeaponUpgradeReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:WeaponUpgradeReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WeaponUpgradeReq {
    // message fields
    // @@protoc_insertion_point(field:WeaponUpgradeReq.item_param_list)
    pub item_param_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:WeaponUpgradeReq.target_weapon_guid)
    pub target_weapon_guid: u64,
    // @@protoc_insertion_point(field:WeaponUpgradeReq.food_weapon_guid_list)
    pub food_weapon_guid_list: ::std::vec::Vec<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:WeaponUpgradeReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WeaponUpgradeReq {
    fn default() -> &'a WeaponUpgradeReq {
        <WeaponUpgradeReq as ::protobuf::Message>::default_instance()
    }
}

impl WeaponUpgradeReq {
    pub fn new() -> WeaponUpgradeReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_param_list",
            |m: &WeaponUpgradeReq| { &m.item_param_list },
            |m: &mut WeaponUpgradeReq| { &mut m.item_param_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_weapon_guid",
            |m: &WeaponUpgradeReq| { &m.target_weapon_guid },
            |m: &mut WeaponUpgradeReq| { &mut m.target_weapon_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "food_weapon_guid_list",
            |m: &WeaponUpgradeReq| { &m.food_weapon_guid_list },
            |m: &mut WeaponUpgradeReq| { &mut m.food_weapon_guid_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WeaponUpgradeReq>(
            "WeaponUpgradeReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WeaponUpgradeReq {
    const NAME: &'static str = "WeaponUpgradeReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.item_param_list.push(is.read_message()?);
                },
                48 => {
                    self.target_weapon_guid = is.read_uint64()?;
                },
                106 => {
                    is.read_repeated_packed_uint64_into(&mut self.food_weapon_guid_list)?;
                },
                104 => {
                    self.food_weapon_guid_list.push(is.read_uint64()?);
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
        for value in &self.item_param_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.target_weapon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.target_weapon_guid);
        }
        for value in &self.food_weapon_guid_list {
            my_size += ::protobuf::rt::uint64_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.item_param_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.target_weapon_guid != 0 {
            os.write_uint64(6, self.target_weapon_guid)?;
        }
        for v in &self.food_weapon_guid_list {
            os.write_uint64(13, *v)?;
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

    fn new() -> WeaponUpgradeReq {
        WeaponUpgradeReq::new()
    }

    fn clear(&mut self) {
        self.item_param_list.clear();
        self.target_weapon_guid = 0;
        self.food_weapon_guid_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WeaponUpgradeReq {
        static instance: WeaponUpgradeReq = WeaponUpgradeReq {
            item_param_list: ::std::vec::Vec::new(),
            target_weapon_guid: 0,
            food_weapon_guid_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WeaponUpgradeReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WeaponUpgradeReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WeaponUpgradeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeaponUpgradeReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16WeaponUpgradeReq.proto\x1a\x0fItemParam.proto\"\xa7\x01\n\x10Weapo\
    nUpgradeReq\x122\n\x0fitem_param_list\x18\x03\x20\x03(\x0b2\n.ItemParamR\
    \ritemParamList\x12,\n\x12target_weapon_guid\x18\x06\x20\x01(\x04R\x10ta\
    rgetWeaponGuid\x121\n\x15food_weapon_guid_list\x18\r\x20\x03(\x04R\x12fo\
    odWeaponGuidListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(WeaponUpgradeReq::generated_message_descriptor_data());
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
