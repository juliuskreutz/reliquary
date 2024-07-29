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

//! Generated file from `ReliquaryUpgradeRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ReliquaryUpgradeRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ReliquaryUpgradeRsp {
    // message fields
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.target_reliquary_guid)
    pub target_reliquary_guid: u64,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.power_up_rate)
    pub power_up_rate: u32,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.cur_level)
    pub cur_level: u32,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.old_level)
    pub old_level: u32,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.cur_append_prop_list)
    pub cur_append_prop_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ReliquaryUpgradeRsp.old_append_prop_list)
    pub old_append_prop_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:ReliquaryUpgradeRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ReliquaryUpgradeRsp {
    fn default() -> &'a ReliquaryUpgradeRsp {
        <ReliquaryUpgradeRsp as ::protobuf::Message>::default_instance()
    }
}

impl ReliquaryUpgradeRsp {
    pub fn new() -> ReliquaryUpgradeRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_reliquary_guid",
            |m: &ReliquaryUpgradeRsp| { &m.target_reliquary_guid },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.target_reliquary_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ReliquaryUpgradeRsp| { &m.retcode },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "power_up_rate",
            |m: &ReliquaryUpgradeRsp| { &m.power_up_rate },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.power_up_rate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_level",
            |m: &ReliquaryUpgradeRsp| { &m.cur_level },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.cur_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "old_level",
            |m: &ReliquaryUpgradeRsp| { &m.old_level },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.old_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "cur_append_prop_list",
            |m: &ReliquaryUpgradeRsp| { &m.cur_append_prop_list },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.cur_append_prop_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "old_append_prop_list",
            |m: &ReliquaryUpgradeRsp| { &m.old_append_prop_list },
            |m: &mut ReliquaryUpgradeRsp| { &mut m.old_append_prop_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ReliquaryUpgradeRsp>(
            "ReliquaryUpgradeRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ReliquaryUpgradeRsp {
    const NAME: &'static str = "ReliquaryUpgradeRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.target_reliquary_guid = is.read_uint64()?;
                },
                24 => {
                    self.retcode = is.read_int32()?;
                },
                40 => {
                    self.power_up_rate = is.read_uint32()?;
                },
                48 => {
                    self.cur_level = is.read_uint32()?;
                },
                56 => {
                    self.old_level = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.cur_append_prop_list)?;
                },
                72 => {
                    self.cur_append_prop_list.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.old_append_prop_list)?;
                },
                80 => {
                    self.old_append_prop_list.push(is.read_uint32()?);
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
        if self.target_reliquary_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.target_reliquary_guid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.retcode);
        }
        if self.power_up_rate != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.power_up_rate);
        }
        if self.cur_level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.cur_level);
        }
        if self.old_level != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.old_level);
        }
        for value in &self.cur_append_prop_list {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.old_append_prop_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.target_reliquary_guid != 0 {
            os.write_uint64(2, self.target_reliquary_guid)?;
        }
        if self.retcode != 0 {
            os.write_int32(3, self.retcode)?;
        }
        if self.power_up_rate != 0 {
            os.write_uint32(5, self.power_up_rate)?;
        }
        if self.cur_level != 0 {
            os.write_uint32(6, self.cur_level)?;
        }
        if self.old_level != 0 {
            os.write_uint32(7, self.old_level)?;
        }
        for v in &self.cur_append_prop_list {
            os.write_uint32(9, *v)?;
        };
        for v in &self.old_append_prop_list {
            os.write_uint32(10, *v)?;
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

    fn new() -> ReliquaryUpgradeRsp {
        ReliquaryUpgradeRsp::new()
    }

    fn clear(&mut self) {
        self.target_reliquary_guid = 0;
        self.retcode = 0;
        self.power_up_rate = 0;
        self.cur_level = 0;
        self.old_level = 0;
        self.cur_append_prop_list.clear();
        self.old_append_prop_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ReliquaryUpgradeRsp {
        static instance: ReliquaryUpgradeRsp = ReliquaryUpgradeRsp {
            target_reliquary_guid: 0,
            retcode: 0,
            power_up_rate: 0,
            cur_level: 0,
            old_level: 0,
            cur_append_prop_list: ::std::vec::Vec::new(),
            old_append_prop_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ReliquaryUpgradeRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ReliquaryUpgradeRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ReliquaryUpgradeRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReliquaryUpgradeRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19ReliquaryUpgradeRsp.proto\"\xa3\x02\n\x13ReliquaryUpgradeRsp\x122\
    \n\x15target_reliquary_guid\x18\x02\x20\x01(\x04R\x13targetReliquaryGuid\
    \x12\x18\n\x07retcode\x18\x03\x20\x01(\x05R\x07retcode\x12\"\n\rpower_up\
    _rate\x18\x05\x20\x01(\rR\x0bpowerUpRate\x12\x1b\n\tcur_level\x18\x06\
    \x20\x01(\rR\x08curLevel\x12\x1b\n\told_level\x18\x07\x20\x01(\rR\x08old\
    Level\x12/\n\x14cur_append_prop_list\x18\t\x20\x03(\rR\x11curAppendPropL\
    ist\x12/\n\x14old_append_prop_list\x18\n\x20\x03(\rR\x11oldAppendPropLis\
    tB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ReliquaryUpgradeRsp::generated_message_descriptor_data());
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
