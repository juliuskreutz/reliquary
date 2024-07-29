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

//! Generated file from `CityReputationRequestInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CityReputationRequestInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CityReputationRequestInfo {
    // message fields
    // @@protoc_insertion_point(field:CityReputationRequestInfo.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:CityReputationRequestInfo.request_info_list)
    pub request_info_list: ::std::vec::Vec<city_reputation_request_info::RequestInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:CityReputationRequestInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CityReputationRequestInfo {
    fn default() -> &'a CityReputationRequestInfo {
        <CityReputationRequestInfo as ::protobuf::Message>::default_instance()
    }
}

impl CityReputationRequestInfo {
    pub fn new() -> CityReputationRequestInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &CityReputationRequestInfo| { &m.is_open },
            |m: &mut CityReputationRequestInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "request_info_list",
            |m: &CityReputationRequestInfo| { &m.request_info_list },
            |m: &mut CityReputationRequestInfo| { &mut m.request_info_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CityReputationRequestInfo>(
            "CityReputationRequestInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CityReputationRequestInfo {
    const NAME: &'static str = "CityReputationRequestInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.is_open = is.read_bool()?;
                },
                122 => {
                    self.request_info_list.push(is.read_message()?);
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
        if self.is_open != false {
            my_size += 1 + 1;
        }
        for value in &self.request_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_open != false {
            os.write_bool(6, self.is_open)?;
        }
        for v in &self.request_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> CityReputationRequestInfo {
        CityReputationRequestInfo::new()
    }

    fn clear(&mut self) {
        self.is_open = false;
        self.request_info_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CityReputationRequestInfo {
        static instance: CityReputationRequestInfo = CityReputationRequestInfo {
            is_open: false,
            request_info_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CityReputationRequestInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CityReputationRequestInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CityReputationRequestInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CityReputationRequestInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CityReputationRequestInfo`
pub mod city_reputation_request_info {
    // @@protoc_insertion_point(message:CityReputationRequestInfo.RequestInfo)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct RequestInfo {
        // message fields
        // @@protoc_insertion_point(field:CityReputationRequestInfo.RequestInfo.request_id)
        pub request_id: u32,
        // @@protoc_insertion_point(field:CityReputationRequestInfo.RequestInfo.is_taken_reward)
        pub is_taken_reward: bool,
        // @@protoc_insertion_point(field:CityReputationRequestInfo.RequestInfo.quest_id)
        pub quest_id: u32,
        // special fields
        // @@protoc_insertion_point(special_field:CityReputationRequestInfo.RequestInfo.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a RequestInfo {
        fn default() -> &'a RequestInfo {
            <RequestInfo as ::protobuf::Message>::default_instance()
        }
    }

    impl RequestInfo {
        pub fn new() -> RequestInfo {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(3);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "request_id",
                |m: &RequestInfo| { &m.request_id },
                |m: &mut RequestInfo| { &mut m.request_id },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "is_taken_reward",
                |m: &RequestInfo| { &m.is_taken_reward },
                |m: &mut RequestInfo| { &mut m.is_taken_reward },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "quest_id",
                |m: &RequestInfo| { &m.quest_id },
                |m: &mut RequestInfo| { &mut m.quest_id },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestInfo>(
                "CityReputationRequestInfo.RequestInfo",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for RequestInfo {
        const NAME: &'static str = "RequestInfo";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.request_id = is.read_uint32()?;
                    },
                    88 => {
                        self.is_taken_reward = is.read_bool()?;
                    },
                    112 => {
                        self.quest_id = is.read_uint32()?;
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
            if self.request_id != 0 {
                my_size += ::protobuf::rt::uint32_size(1, self.request_id);
            }
            if self.is_taken_reward != false {
                my_size += 1 + 1;
            }
            if self.quest_id != 0 {
                my_size += ::protobuf::rt::uint32_size(14, self.quest_id);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.request_id != 0 {
                os.write_uint32(1, self.request_id)?;
            }
            if self.is_taken_reward != false {
                os.write_bool(11, self.is_taken_reward)?;
            }
            if self.quest_id != 0 {
                os.write_uint32(14, self.quest_id)?;
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

        fn new() -> RequestInfo {
            RequestInfo::new()
        }

        fn clear(&mut self) {
            self.request_id = 0;
            self.is_taken_reward = false;
            self.quest_id = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static RequestInfo {
            static instance: RequestInfo = RequestInfo {
                request_id: 0,
                is_taken_reward: false,
                quest_id: 0,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for RequestInfo {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("CityReputationRequestInfo.RequestInfo").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for RequestInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for RequestInfo {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fCityReputationRequestInfo.proto\"\xf9\x01\n\x19CityReputationReque\
    stInfo\x12\x17\n\x07is_open\x18\x06\x20\x01(\x08R\x06isOpen\x12R\n\x11re\
    quest_info_list\x18\x0f\x20\x03(\x0b2&.CityReputationRequestInfo.Request\
    InfoR\x0frequestInfoList\x1ao\n\x0bRequestInfo\x12\x1d\n\nrequest_id\x18\
    \x01\x20\x01(\rR\trequestId\x12&\n\x0fis_taken_reward\x18\x0b\x20\x01(\
    \x08R\risTakenReward\x12\x19\n\x08quest_id\x18\x0e\x20\x01(\rR\x07questI\
    dB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(CityReputationRequestInfo::generated_message_descriptor_data());
            messages.push(city_reputation_request_info::RequestInfo::generated_message_descriptor_data());
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
