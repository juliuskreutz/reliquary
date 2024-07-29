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

//! Generated file from `PlayerTimeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerTimeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerTimeNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerTimeNotify.player_time)
    pub player_time: u64,
    // @@protoc_insertion_point(field:PlayerTimeNotify.is_paused)
    pub is_paused: bool,
    // @@protoc_insertion_point(field:PlayerTimeNotify.server_time)
    pub server_time: u64,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerTimeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerTimeNotify {
    fn default() -> &'a PlayerTimeNotify {
        <PlayerTimeNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerTimeNotify {
    pub fn new() -> PlayerTimeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "player_time",
            |m: &PlayerTimeNotify| { &m.player_time },
            |m: &mut PlayerTimeNotify| { &mut m.player_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_paused",
            |m: &PlayerTimeNotify| { &m.is_paused },
            |m: &mut PlayerTimeNotify| { &mut m.is_paused },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_time",
            |m: &PlayerTimeNotify| { &m.server_time },
            |m: &mut PlayerTimeNotify| { &mut m.server_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerTimeNotify>(
            "PlayerTimeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerTimeNotify {
    const NAME: &'static str = "PlayerTimeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.player_time = is.read_uint64()?;
                },
                64 => {
                    self.is_paused = is.read_bool()?;
                },
                80 => {
                    self.server_time = is.read_uint64()?;
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
        if self.player_time != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.player_time);
        }
        if self.is_paused != false {
            my_size += 1 + 1;
        }
        if self.server_time != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.server_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.player_time != 0 {
            os.write_uint64(5, self.player_time)?;
        }
        if self.is_paused != false {
            os.write_bool(8, self.is_paused)?;
        }
        if self.server_time != 0 {
            os.write_uint64(10, self.server_time)?;
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

    fn new() -> PlayerTimeNotify {
        PlayerTimeNotify::new()
    }

    fn clear(&mut self) {
        self.player_time = 0;
        self.is_paused = false;
        self.server_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerTimeNotify {
        static instance: PlayerTimeNotify = PlayerTimeNotify {
            player_time: 0,
            is_paused: false,
            server_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerTimeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerTimeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerTimeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerTimeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerTimeNotify.proto\"q\n\x10PlayerTimeNotify\x12\x1f\n\x0bplaye\
    r_time\x18\x05\x20\x01(\x04R\nplayerTime\x12\x1b\n\tis_paused\x18\x08\
    \x20\x01(\x08R\x08isPaused\x12\x1f\n\x0bserver_time\x18\n\x20\x01(\x04R\
    \nserverTimeB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(PlayerTimeNotify::generated_message_descriptor_data());
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
