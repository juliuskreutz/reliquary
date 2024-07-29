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

//! Generated file from `FleurFairMinigameInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FleurFairMinigameInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FleurFairMinigameInfo {
    // message fields
    // @@protoc_insertion_point(field:FleurFairMinigameInfo.open_time)
    pub open_time: u32,
    // @@protoc_insertion_point(field:FleurFairMinigameInfo.minigame_id)
    pub minigame_id: u32,
    // @@protoc_insertion_point(field:FleurFairMinigameInfo.is_open)
    pub is_open: bool,
    // message oneof groups
    pub detail: ::std::option::Option<fleur_fair_minigame_info::Detail>,
    // special fields
    // @@protoc_insertion_point(special_field:FleurFairMinigameInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FleurFairMinigameInfo {
    fn default() -> &'a FleurFairMinigameInfo {
        <FleurFairMinigameInfo as ::protobuf::Message>::default_instance()
    }
}

impl FleurFairMinigameInfo {
    pub fn new() -> FleurFairMinigameInfo {
        ::std::default::Default::default()
    }

    // .FleurFairBalloonInfo balloon_info = 14;

    pub fn balloon_info(&self) -> &super::FleurFairBalloonInfo::FleurFairBalloonInfo {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(ref v)) => v,
            _ => <super::FleurFairBalloonInfo::FleurFairBalloonInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_balloon_info(&mut self) {
        self.detail = ::std::option::Option::None;
    }

    pub fn has_balloon_info(&self) -> bool {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_balloon_info(&mut self, v: super::FleurFairBalloonInfo::FleurFairBalloonInfo) {
        self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_balloon_info(&mut self) -> &mut super::FleurFairBalloonInfo::FleurFairBalloonInfo {
        if let ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(_)) = self.detail {
        } else {
            self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(super::FleurFairBalloonInfo::FleurFairBalloonInfo::new()));
        }
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_balloon_info(&mut self) -> super::FleurFairBalloonInfo::FleurFairBalloonInfo {
        if self.has_balloon_info() {
            match self.detail.take() {
                ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FleurFairBalloonInfo::FleurFairBalloonInfo::new()
        }
    }

    // .FleurFairFallInfo fall_info = 7;

    pub fn fall_info(&self) -> &super::FleurFairFallInfo::FleurFairFallInfo {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(ref v)) => v,
            _ => <super::FleurFairFallInfo::FleurFairFallInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_fall_info(&mut self) {
        self.detail = ::std::option::Option::None;
    }

    pub fn has_fall_info(&self) -> bool {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fall_info(&mut self, v: super::FleurFairFallInfo::FleurFairFallInfo) {
        self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_fall_info(&mut self) -> &mut super::FleurFairFallInfo::FleurFairFallInfo {
        if let ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(_)) = self.detail {
        } else {
            self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(super::FleurFairFallInfo::FleurFairFallInfo::new()));
        }
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_fall_info(&mut self) -> super::FleurFairFallInfo::FleurFairFallInfo {
        if self.has_fall_info() {
            match self.detail.take() {
                ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FleurFairFallInfo::FleurFairFallInfo::new()
        }
    }

    // .FleurFairMusicGameInfo music_info = 10;

    pub fn music_info(&self) -> &super::FleurFairMusicGameInfo::FleurFairMusicGameInfo {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(ref v)) => v,
            _ => <super::FleurFairMusicGameInfo::FleurFairMusicGameInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_music_info(&mut self) {
        self.detail = ::std::option::Option::None;
    }

    pub fn has_music_info(&self) -> bool {
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_music_info(&mut self, v: super::FleurFairMusicGameInfo::FleurFairMusicGameInfo) {
        self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_music_info(&mut self) -> &mut super::FleurFairMusicGameInfo::FleurFairMusicGameInfo {
        if let ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(_)) = self.detail {
        } else {
            self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(super::FleurFairMusicGameInfo::FleurFairMusicGameInfo::new()));
        }
        match self.detail {
            ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_music_info(&mut self) -> super::FleurFairMusicGameInfo::FleurFairMusicGameInfo {
        if self.has_music_info() {
            match self.detail.take() {
                ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FleurFairMusicGameInfo::FleurFairMusicGameInfo::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "open_time",
            |m: &FleurFairMinigameInfo| { &m.open_time },
            |m: &mut FleurFairMinigameInfo| { &mut m.open_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "minigame_id",
            |m: &FleurFairMinigameInfo| { &m.minigame_id },
            |m: &mut FleurFairMinigameInfo| { &mut m.minigame_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &FleurFairMinigameInfo| { &m.is_open },
            |m: &mut FleurFairMinigameInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FleurFairBalloonInfo::FleurFairBalloonInfo>(
            "balloon_info",
            FleurFairMinigameInfo::has_balloon_info,
            FleurFairMinigameInfo::balloon_info,
            FleurFairMinigameInfo::mut_balloon_info,
            FleurFairMinigameInfo::set_balloon_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FleurFairFallInfo::FleurFairFallInfo>(
            "fall_info",
            FleurFairMinigameInfo::has_fall_info,
            FleurFairMinigameInfo::fall_info,
            FleurFairMinigameInfo::mut_fall_info,
            FleurFairMinigameInfo::set_fall_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FleurFairMusicGameInfo::FleurFairMusicGameInfo>(
            "music_info",
            FleurFairMinigameInfo::has_music_info,
            FleurFairMinigameInfo::music_info,
            FleurFairMinigameInfo::mut_music_info,
            FleurFairMinigameInfo::set_music_info,
        ));
        oneofs.push(fleur_fair_minigame_info::Detail::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FleurFairMinigameInfo>(
            "FleurFairMinigameInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FleurFairMinigameInfo {
    const NAME: &'static str = "FleurFairMinigameInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.open_time = is.read_uint32()?;
                },
                88 => {
                    self.minigame_id = is.read_uint32()?;
                },
                40 => {
                    self.is_open = is.read_bool()?;
                },
                114 => {
                    self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::BalloonInfo(is.read_message()?));
                },
                58 => {
                    self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::FallInfo(is.read_message()?));
                },
                82 => {
                    self.detail = ::std::option::Option::Some(fleur_fair_minigame_info::Detail::MusicInfo(is.read_message()?));
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
        if self.open_time != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.open_time);
        }
        if self.minigame_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.minigame_id);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if let ::std::option::Option::Some(ref v) = self.detail {
            match v {
                &fleur_fair_minigame_info::Detail::BalloonInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fleur_fair_minigame_info::Detail::FallInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fleur_fair_minigame_info::Detail::MusicInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.open_time != 0 {
            os.write_uint32(15, self.open_time)?;
        }
        if self.minigame_id != 0 {
            os.write_uint32(11, self.minigame_id)?;
        }
        if self.is_open != false {
            os.write_bool(5, self.is_open)?;
        }
        if let ::std::option::Option::Some(ref v) = self.detail {
            match v {
                &fleur_fair_minigame_info::Detail::BalloonInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &fleur_fair_minigame_info::Detail::FallInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &fleur_fair_minigame_info::Detail::MusicInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
            };
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

    fn new() -> FleurFairMinigameInfo {
        FleurFairMinigameInfo::new()
    }

    fn clear(&mut self) {
        self.open_time = 0;
        self.minigame_id = 0;
        self.is_open = false;
        self.detail = ::std::option::Option::None;
        self.detail = ::std::option::Option::None;
        self.detail = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FleurFairMinigameInfo {
        static instance: FleurFairMinigameInfo = FleurFairMinigameInfo {
            open_time: 0,
            minigame_id: 0,
            is_open: false,
            detail: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FleurFairMinigameInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FleurFairMinigameInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FleurFairMinigameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FleurFairMinigameInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FleurFairMinigameInfo`
pub mod fleur_fair_minigame_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FleurFairMinigameInfo.detail)
    pub enum Detail {
        // @@protoc_insertion_point(oneof_field:FleurFairMinigameInfo.balloon_info)
        BalloonInfo(super::super::FleurFairBalloonInfo::FleurFairBalloonInfo),
        // @@protoc_insertion_point(oneof_field:FleurFairMinigameInfo.fall_info)
        FallInfo(super::super::FleurFairFallInfo::FleurFairFallInfo),
        // @@protoc_insertion_point(oneof_field:FleurFairMinigameInfo.music_info)
        MusicInfo(super::super::FleurFairMusicGameInfo::FleurFairMusicGameInfo),
    }

    impl ::protobuf::Oneof for Detail {
    }

    impl ::protobuf::OneofFull for Detail {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FleurFairMinigameInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("detail").unwrap()).clone()
        }
    }

    impl Detail {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Detail>("detail")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bFleurFairMinigameInfo.proto\x1a\x1aFleurFairBalloonInfo.proto\x1a\
    \x17FleurFairFallInfo.proto\x1a\x1cFleurFairMusicGameInfo.proto\"\xa1\
    \x02\n\x15FleurFairMinigameInfo\x12\x1b\n\topen_time\x18\x0f\x20\x01(\rR\
    \x08openTime\x12\x1f\n\x0bminigame_id\x18\x0b\x20\x01(\rR\nminigameId\
    \x12\x17\n\x07is_open\x18\x05\x20\x01(\x08R\x06isOpen\x12:\n\x0cballoon_\
    info\x18\x0e\x20\x01(\x0b2\x15.FleurFairBalloonInfoH\0R\x0bballoonInfo\
    \x121\n\tfall_info\x18\x07\x20\x01(\x0b2\x12.FleurFairFallInfoH\0R\x08fa\
    llInfo\x128\n\nmusic_info\x18\n\x20\x01(\x0b2\x17.FleurFairMusicGameInfo\
    H\0R\tmusicInfoB\x08\n\x06detailB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::FleurFairBalloonInfo::file_descriptor().clone());
            deps.push(super::FleurFairFallInfo::file_descriptor().clone());
            deps.push(super::FleurFairMusicGameInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FleurFairMinigameInfo::generated_message_descriptor_data());
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
