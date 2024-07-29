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

//! Generated file from `ChangeGameTimeReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChangeGameTimeReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChangeGameTimeReq {
    // message fields
    // @@protoc_insertion_point(field:ChangeGameTimeReq.is_force_set)
    pub is_force_set: bool,
    // @@protoc_insertion_point(field:ChangeGameTimeReq.game_time)
    pub game_time: u32,
    // @@protoc_insertion_point(field:ChangeGameTimeReq.extra_days)
    pub extra_days: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChangeGameTimeReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChangeGameTimeReq {
    fn default() -> &'a ChangeGameTimeReq {
        <ChangeGameTimeReq as ::protobuf::Message>::default_instance()
    }
}

impl ChangeGameTimeReq {
    pub fn new() -> ChangeGameTimeReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_force_set",
            |m: &ChangeGameTimeReq| { &m.is_force_set },
            |m: &mut ChangeGameTimeReq| { &mut m.is_force_set },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "game_time",
            |m: &ChangeGameTimeReq| { &m.game_time },
            |m: &mut ChangeGameTimeReq| { &mut m.game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_days",
            |m: &ChangeGameTimeReq| { &m.extra_days },
            |m: &mut ChangeGameTimeReq| { &mut m.extra_days },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChangeGameTimeReq>(
            "ChangeGameTimeReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChangeGameTimeReq {
    const NAME: &'static str = "ChangeGameTimeReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.is_force_set = is.read_bool()?;
                },
                72 => {
                    self.game_time = is.read_uint32()?;
                },
                16 => {
                    self.extra_days = is.read_uint32()?;
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
        if self.is_force_set != false {
            my_size += 1 + 1;
        }
        if self.game_time != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.game_time);
        }
        if self.extra_days != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.extra_days);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_force_set != false {
            os.write_bool(1, self.is_force_set)?;
        }
        if self.game_time != 0 {
            os.write_uint32(9, self.game_time)?;
        }
        if self.extra_days != 0 {
            os.write_uint32(2, self.extra_days)?;
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

    fn new() -> ChangeGameTimeReq {
        ChangeGameTimeReq::new()
    }

    fn clear(&mut self) {
        self.is_force_set = false;
        self.game_time = 0;
        self.extra_days = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChangeGameTimeReq {
        static instance: ChangeGameTimeReq = ChangeGameTimeReq {
            is_force_set: false,
            game_time: 0,
            extra_days: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChangeGameTimeReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChangeGameTimeReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChangeGameTimeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeGameTimeReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ChangeGameTimeReq.proto\"q\n\x11ChangeGameTimeReq\x12\x20\n\x0cis_\
    force_set\x18\x01\x20\x01(\x08R\nisForceSet\x12\x1b\n\tgame_time\x18\t\
    \x20\x01(\rR\x08gameTime\x12\x1d\n\nextra_days\x18\x02\x20\x01(\rR\textr\
    aDaysB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ChangeGameTimeReq::generated_message_descriptor_data());
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