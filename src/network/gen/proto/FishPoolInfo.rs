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

//! Generated file from `FishPoolInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FishPoolInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FishPoolInfo {
    // message fields
    // @@protoc_insertion_point(field:FishPoolInfo.pool_id)
    pub pool_id: u32,
    // @@protoc_insertion_point(field:FishPoolInfo.fish_area_list)
    pub fish_area_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:FishPoolInfo.today_fish_num)
    pub today_fish_num: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FishPoolInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FishPoolInfo {
    fn default() -> &'a FishPoolInfo {
        <FishPoolInfo as ::protobuf::Message>::default_instance()
    }
}

impl FishPoolInfo {
    pub fn new() -> FishPoolInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pool_id",
            |m: &FishPoolInfo| { &m.pool_id },
            |m: &mut FishPoolInfo| { &mut m.pool_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "fish_area_list",
            |m: &FishPoolInfo| { &m.fish_area_list },
            |m: &mut FishPoolInfo| { &mut m.fish_area_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "today_fish_num",
            |m: &FishPoolInfo| { &m.today_fish_num },
            |m: &mut FishPoolInfo| { &mut m.today_fish_num },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FishPoolInfo>(
            "FishPoolInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FishPoolInfo {
    const NAME: &'static str = "FishPoolInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.pool_id = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.fish_area_list)?;
                },
                16 => {
                    self.fish_area_list.push(is.read_uint32()?);
                },
                24 => {
                    self.today_fish_num = is.read_uint32()?;
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
        if self.pool_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.pool_id);
        }
        for value in &self.fish_area_list {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.today_fish_num != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.today_fish_num);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.pool_id != 0 {
            os.write_uint32(1, self.pool_id)?;
        }
        for v in &self.fish_area_list {
            os.write_uint32(2, *v)?;
        };
        if self.today_fish_num != 0 {
            os.write_uint32(3, self.today_fish_num)?;
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

    fn new() -> FishPoolInfo {
        FishPoolInfo::new()
    }

    fn clear(&mut self) {
        self.pool_id = 0;
        self.fish_area_list.clear();
        self.today_fish_num = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FishPoolInfo {
        static instance: FishPoolInfo = FishPoolInfo {
            pool_id: 0,
            fish_area_list: ::std::vec::Vec::new(),
            today_fish_num: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FishPoolInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FishPoolInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FishPoolInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FishPoolInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12FishPoolInfo.proto\"s\n\x0cFishPoolInfo\x12\x17\n\x07pool_id\x18\
    \x01\x20\x01(\rR\x06poolId\x12$\n\x0efish_area_list\x18\x02\x20\x03(\rR\
    \x0cfishAreaList\x12$\n\x0etoday_fish_num\x18\x03\x20\x01(\rR\x0ctodayFi\
    shNumB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(FishPoolInfo::generated_message_descriptor_data());
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
