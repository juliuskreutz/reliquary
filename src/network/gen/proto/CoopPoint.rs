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

//! Generated file from `CoopPoint.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CoopPoint)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CoopPoint {
    // message fields
    // @@protoc_insertion_point(field:CoopPoint.id)
    pub id: u32,
    // @@protoc_insertion_point(field:CoopPoint.state)
    pub state: ::protobuf::EnumOrUnknown<coop_point::State>,
    // @@protoc_insertion_point(field:CoopPoint.self_confidence)
    pub self_confidence: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CoopPoint.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CoopPoint {
    fn default() -> &'a CoopPoint {
        <CoopPoint as ::protobuf::Message>::default_instance()
    }
}

impl CoopPoint {
    pub fn new() -> CoopPoint {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &CoopPoint| { &m.id },
            |m: &mut CoopPoint| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &CoopPoint| { &m.state },
            |m: &mut CoopPoint| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "self_confidence",
            |m: &CoopPoint| { &m.self_confidence },
            |m: &mut CoopPoint| { &mut m.self_confidence },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CoopPoint>(
            "CoopPoint",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CoopPoint {
    const NAME: &'static str = "CoopPoint";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.id = is.read_uint32()?;
                },
                112 => {
                    self.state = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.self_confidence = is.read_uint32()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.id);
        }
        if self.state != ::protobuf::EnumOrUnknown::new(coop_point::State::STATE_UNSTARTED) {
            my_size += ::protobuf::rt::int32_size(14, self.state.value());
        }
        if self.self_confidence != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.self_confidence);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(8, self.id)?;
        }
        if self.state != ::protobuf::EnumOrUnknown::new(coop_point::State::STATE_UNSTARTED) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.state))?;
        }
        if self.self_confidence != 0 {
            os.write_uint32(10, self.self_confidence)?;
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

    fn new() -> CoopPoint {
        CoopPoint::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.state = ::protobuf::EnumOrUnknown::new(coop_point::State::STATE_UNSTARTED);
        self.self_confidence = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CoopPoint {
        static instance: CoopPoint = CoopPoint {
            id: 0,
            state: ::protobuf::EnumOrUnknown::from_i32(0),
            self_confidence: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CoopPoint {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CoopPoint").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CoopPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoopPoint {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CoopPoint`
pub mod coop_point {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:CoopPoint.State)
    pub enum State {
        // @@protoc_insertion_point(enum_value:CoopPoint.State.STATE_UNSTARTED)
        STATE_UNSTARTED = 0,
        // @@protoc_insertion_point(enum_value:CoopPoint.State.STATE_STARTED)
        STATE_STARTED = 1,
        // @@protoc_insertion_point(enum_value:CoopPoint.State.STATE_FINISHED)
        STATE_FINISHED = 2,
    }

    impl ::protobuf::Enum for State {
        const NAME: &'static str = "State";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<State> {
            match value {
                0 => ::std::option::Option::Some(State::STATE_UNSTARTED),
                1 => ::std::option::Option::Some(State::STATE_STARTED),
                2 => ::std::option::Option::Some(State::STATE_FINISHED),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<State> {
            match str {
                "STATE_UNSTARTED" => ::std::option::Option::Some(State::STATE_UNSTARTED),
                "STATE_STARTED" => ::std::option::Option::Some(State::STATE_STARTED),
                "STATE_FINISHED" => ::std::option::Option::Some(State::STATE_FINISHED),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [State] = &[
            State::STATE_UNSTARTED,
            State::STATE_STARTED,
            State::STATE_FINISHED,
        ];
    }

    impl ::protobuf::EnumFull for State {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("CoopPoint.State").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for State {
        fn default() -> Self {
            State::STATE_UNSTARTED
        }
    }

    impl State {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<State>("CoopPoint.State")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fCoopPoint.proto\"\xb1\x01\n\tCoopPoint\x12\x0e\n\x02id\x18\x08\x20\
    \x01(\rR\x02id\x12&\n\x05state\x18\x0e\x20\x01(\x0e2\x10.CoopPoint.State\
    R\x05state\x12'\n\x0fself_confidence\x18\n\x20\x01(\rR\x0eselfConfidence\
    \"C\n\x05State\x12\x13\n\x0fSTATE_UNSTARTED\x10\0\x12\x11\n\rSTATE_START\
    ED\x10\x01\x12\x12\n\x0eSTATE_FINISHED\x10\x02B\x1b\n\x19emu.grasscutter\
    .net.protob\x06proto3\
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
            messages.push(CoopPoint::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(coop_point::State::generated_enum_descriptor_data());
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
