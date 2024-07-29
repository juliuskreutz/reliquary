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

//! Generated file from `DoGachaRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DoGachaRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DoGachaRsp {
    // message fields
    // @@protoc_insertion_point(field:DoGachaRsp.gacha_type)
    pub gacha_type: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.gacha_item_list)
    pub gacha_item_list: ::std::vec::Vec<super::GachaItem::GachaItem>,
    // @@protoc_insertion_point(field:DoGachaRsp.gacha_schedule_id)
    pub gacha_schedule_id: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.is_under_general_restrict)
    pub is_under_general_restrict: bool,
    // @@protoc_insertion_point(field:DoGachaRsp.daily_gacha_times)
    pub daily_gacha_times: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.is_under_minors_restrict)
    pub is_under_minors_restrict: bool,
    // @@protoc_insertion_point(field:DoGachaRsp.gacha_times)
    pub gacha_times: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:DoGachaRsp.tenCostItemNum)
    pub tenCostItemNum: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.wishMaxProgress)
    pub wishMaxProgress: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.leftGachaTimes)
    pub leftGachaTimes: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.costItemNum)
    pub costItemNum: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.curScheduleDailyGachaTimes)
    pub curScheduleDailyGachaTimes: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.wishProgress)
    pub wishProgress: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.newGachaRandom)
    pub newGachaRandom: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.costItemId)
    pub costItemId: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.tenCostItemId)
    pub tenCostItemId: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.gachaTimesLimit)
    pub gachaTimesLimit: u32,
    // @@protoc_insertion_point(field:DoGachaRsp.wishItemId)
    pub wishItemId: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DoGachaRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DoGachaRsp {
    fn default() -> &'a DoGachaRsp {
        <DoGachaRsp as ::protobuf::Message>::default_instance()
    }
}

impl DoGachaRsp {
    pub fn new() -> DoGachaRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(19);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_type",
            |m: &DoGachaRsp| { &m.gacha_type },
            |m: &mut DoGachaRsp| { &mut m.gacha_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "gacha_item_list",
            |m: &DoGachaRsp| { &m.gacha_item_list },
            |m: &mut DoGachaRsp| { &mut m.gacha_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_schedule_id",
            |m: &DoGachaRsp| { &m.gacha_schedule_id },
            |m: &mut DoGachaRsp| { &mut m.gacha_schedule_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_under_general_restrict",
            |m: &DoGachaRsp| { &m.is_under_general_restrict },
            |m: &mut DoGachaRsp| { &mut m.is_under_general_restrict },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "daily_gacha_times",
            |m: &DoGachaRsp| { &m.daily_gacha_times },
            |m: &mut DoGachaRsp| { &mut m.daily_gacha_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_under_minors_restrict",
            |m: &DoGachaRsp| { &m.is_under_minors_restrict },
            |m: &mut DoGachaRsp| { &mut m.is_under_minors_restrict },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_times",
            |m: &DoGachaRsp| { &m.gacha_times },
            |m: &mut DoGachaRsp| { &mut m.gacha_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DoGachaRsp| { &m.retcode },
            |m: &mut DoGachaRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tenCostItemNum",
            |m: &DoGachaRsp| { &m.tenCostItemNum },
            |m: &mut DoGachaRsp| { &mut m.tenCostItemNum },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wishMaxProgress",
            |m: &DoGachaRsp| { &m.wishMaxProgress },
            |m: &mut DoGachaRsp| { &mut m.wishMaxProgress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "leftGachaTimes",
            |m: &DoGachaRsp| { &m.leftGachaTimes },
            |m: &mut DoGachaRsp| { &mut m.leftGachaTimes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "costItemNum",
            |m: &DoGachaRsp| { &m.costItemNum },
            |m: &mut DoGachaRsp| { &mut m.costItemNum },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "curScheduleDailyGachaTimes",
            |m: &DoGachaRsp| { &m.curScheduleDailyGachaTimes },
            |m: &mut DoGachaRsp| { &mut m.curScheduleDailyGachaTimes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wishProgress",
            |m: &DoGachaRsp| { &m.wishProgress },
            |m: &mut DoGachaRsp| { &mut m.wishProgress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "newGachaRandom",
            |m: &DoGachaRsp| { &m.newGachaRandom },
            |m: &mut DoGachaRsp| { &mut m.newGachaRandom },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "costItemId",
            |m: &DoGachaRsp| { &m.costItemId },
            |m: &mut DoGachaRsp| { &mut m.costItemId },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tenCostItemId",
            |m: &DoGachaRsp| { &m.tenCostItemId },
            |m: &mut DoGachaRsp| { &mut m.tenCostItemId },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gachaTimesLimit",
            |m: &DoGachaRsp| { &m.gachaTimesLimit },
            |m: &mut DoGachaRsp| { &mut m.gachaTimesLimit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wishItemId",
            |m: &DoGachaRsp| { &m.wishItemId },
            |m: &mut DoGachaRsp| { &mut m.wishItemId },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DoGachaRsp>(
            "DoGachaRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DoGachaRsp {
    const NAME: &'static str = "DoGachaRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.gacha_type = is.read_uint32()?;
                },
                34 => {
                    self.gacha_item_list.push(is.read_message()?);
                },
                104 => {
                    self.gacha_schedule_id = is.read_uint32()?;
                },
                5704 => {
                    self.is_under_general_restrict = is.read_bool()?;
                },
                11832 => {
                    self.daily_gacha_times = is.read_uint32()?;
                },
                13008 => {
                    self.is_under_minors_restrict = is.read_bool()?;
                },
                64 => {
                    self.gacha_times = is.read_uint32()?;
                },
                56 => {
                    self.retcode = is.read_int32()?;
                },
                112 => {
                    self.tenCostItemNum = is.read_uint32()?;
                },
                72 => {
                    self.wishMaxProgress = is.read_uint32()?;
                },
                88 => {
                    self.leftGachaTimes = is.read_uint32()?;
                },
                80 => {
                    self.costItemNum = is.read_uint32()?;
                },
                5648 => {
                    self.curScheduleDailyGachaTimes = is.read_uint32()?;
                },
                40 => {
                    self.wishProgress = is.read_uint32()?;
                },
                128 => {
                    self.newGachaRandom = is.read_uint32()?;
                },
                96 => {
                    self.costItemId = is.read_uint32()?;
                },
                24 => {
                    self.tenCostItemId = is.read_uint32()?;
                },
                120 => {
                    self.gachaTimesLimit = is.read_uint32()?;
                },
                8 => {
                    self.wishItemId = is.read_uint32()?;
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
        if self.gacha_type != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.gacha_type);
        }
        for value in &self.gacha_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.gacha_schedule_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.gacha_schedule_id);
        }
        if self.is_under_general_restrict != false {
            my_size += 2 + 1;
        }
        if self.daily_gacha_times != 0 {
            my_size += ::protobuf::rt::uint32_size(1479, self.daily_gacha_times);
        }
        if self.is_under_minors_restrict != false {
            my_size += 2 + 1;
        }
        if self.gacha_times != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.gacha_times);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.retcode);
        }
        if self.tenCostItemNum != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.tenCostItemNum);
        }
        if self.wishMaxProgress != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.wishMaxProgress);
        }
        if self.leftGachaTimes != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.leftGachaTimes);
        }
        if self.costItemNum != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.costItemNum);
        }
        if self.curScheduleDailyGachaTimes != 0 {
            my_size += ::protobuf::rt::uint32_size(706, self.curScheduleDailyGachaTimes);
        }
        if self.wishProgress != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.wishProgress);
        }
        if self.newGachaRandom != 0 {
            my_size += ::protobuf::rt::uint32_size(16, self.newGachaRandom);
        }
        if self.costItemId != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.costItemId);
        }
        if self.tenCostItemId != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.tenCostItemId);
        }
        if self.gachaTimesLimit != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.gachaTimesLimit);
        }
        if self.wishItemId != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.wishItemId);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gacha_type != 0 {
            os.write_uint32(6, self.gacha_type)?;
        }
        for v in &self.gacha_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.gacha_schedule_id != 0 {
            os.write_uint32(13, self.gacha_schedule_id)?;
        }
        if self.is_under_general_restrict != false {
            os.write_bool(713, self.is_under_general_restrict)?;
        }
        if self.daily_gacha_times != 0 {
            os.write_uint32(1479, self.daily_gacha_times)?;
        }
        if self.is_under_minors_restrict != false {
            os.write_bool(1626, self.is_under_minors_restrict)?;
        }
        if self.gacha_times != 0 {
            os.write_uint32(8, self.gacha_times)?;
        }
        if self.retcode != 0 {
            os.write_int32(7, self.retcode)?;
        }
        if self.tenCostItemNum != 0 {
            os.write_uint32(14, self.tenCostItemNum)?;
        }
        if self.wishMaxProgress != 0 {
            os.write_uint32(9, self.wishMaxProgress)?;
        }
        if self.leftGachaTimes != 0 {
            os.write_uint32(11, self.leftGachaTimes)?;
        }
        if self.costItemNum != 0 {
            os.write_uint32(10, self.costItemNum)?;
        }
        if self.curScheduleDailyGachaTimes != 0 {
            os.write_uint32(706, self.curScheduleDailyGachaTimes)?;
        }
        if self.wishProgress != 0 {
            os.write_uint32(5, self.wishProgress)?;
        }
        if self.newGachaRandom != 0 {
            os.write_uint32(16, self.newGachaRandom)?;
        }
        if self.costItemId != 0 {
            os.write_uint32(12, self.costItemId)?;
        }
        if self.tenCostItemId != 0 {
            os.write_uint32(3, self.tenCostItemId)?;
        }
        if self.gachaTimesLimit != 0 {
            os.write_uint32(15, self.gachaTimesLimit)?;
        }
        if self.wishItemId != 0 {
            os.write_uint32(1, self.wishItemId)?;
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

    fn new() -> DoGachaRsp {
        DoGachaRsp::new()
    }

    fn clear(&mut self) {
        self.gacha_type = 0;
        self.gacha_item_list.clear();
        self.gacha_schedule_id = 0;
        self.is_under_general_restrict = false;
        self.daily_gacha_times = 0;
        self.is_under_minors_restrict = false;
        self.gacha_times = 0;
        self.retcode = 0;
        self.tenCostItemNum = 0;
        self.wishMaxProgress = 0;
        self.leftGachaTimes = 0;
        self.costItemNum = 0;
        self.curScheduleDailyGachaTimes = 0;
        self.wishProgress = 0;
        self.newGachaRandom = 0;
        self.costItemId = 0;
        self.tenCostItemId = 0;
        self.gachaTimesLimit = 0;
        self.wishItemId = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DoGachaRsp {
        static instance: DoGachaRsp = DoGachaRsp {
            gacha_type: 0,
            gacha_item_list: ::std::vec::Vec::new(),
            gacha_schedule_id: 0,
            is_under_general_restrict: false,
            daily_gacha_times: 0,
            is_under_minors_restrict: false,
            gacha_times: 0,
            retcode: 0,
            tenCostItemNum: 0,
            wishMaxProgress: 0,
            leftGachaTimes: 0,
            costItemNum: 0,
            curScheduleDailyGachaTimes: 0,
            wishProgress: 0,
            newGachaRandom: 0,
            costItemId: 0,
            tenCostItemId: 0,
            gachaTimesLimit: 0,
            wishItemId: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DoGachaRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DoGachaRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DoGachaRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoGachaRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10DoGachaRsp.proto\x1a\x0fGachaItem.proto\"\xa2\x06\n\nDoGachaRsp\
    \x12\x1d\n\ngacha_type\x18\x06\x20\x01(\rR\tgachaType\x122\n\x0fgacha_it\
    em_list\x18\x04\x20\x03(\x0b2\n.GachaItemR\rgachaItemList\x12*\n\x11gach\
    a_schedule_id\x18\r\x20\x01(\rR\x0fgachaScheduleId\x12:\n\x19is_under_ge\
    neral_restrict\x18\xc9\x05\x20\x01(\x08R\x16isUnderGeneralRestrict\x12+\
    \n\x11daily_gacha_times\x18\xc7\x0b\x20\x01(\rR\x0fdailyGachaTimes\x128\
    \n\x18is_under_minors_restrict\x18\xda\x0c\x20\x01(\x08R\x15isUnderMinor\
    sRestrict\x12\x1f\n\x0bgacha_times\x18\x08\x20\x01(\rR\ngachaTimes\x12\
    \x18\n\x07retcode\x18\x07\x20\x01(\x05R\x07retcode\x12&\n\x0etenCostItem\
    Num\x18\x0e\x20\x01(\rR\x0etenCostItemNum\x12(\n\x0fwishMaxProgress\x18\
    \t\x20\x01(\rR\x0fwishMaxProgress\x12&\n\x0eleftGachaTimes\x18\x0b\x20\
    \x01(\rR\x0eleftGachaTimes\x12\x20\n\x0bcostItemNum\x18\n\x20\x01(\rR\
    \x0bcostItemNum\x12?\n\x1acurScheduleDailyGachaTimes\x18\xc2\x05\x20\x01\
    (\rR\x1acurScheduleDailyGachaTimes\x12\"\n\x0cwishProgress\x18\x05\x20\
    \x01(\rR\x0cwishProgress\x12&\n\x0enewGachaRandom\x18\x10\x20\x01(\rR\
    \x0enewGachaRandom\x12\x1e\n\ncostItemId\x18\x0c\x20\x01(\rR\ncostItemId\
    \x12$\n\rtenCostItemId\x18\x03\x20\x01(\rR\rtenCostItemId\x12(\n\x0fgach\
    aTimesLimit\x18\x0f\x20\x01(\rR\x0fgachaTimesLimit\x12\x1e\n\nwishItemId\
    \x18\x01\x20\x01(\rR\nwishItemIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::GachaItem::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DoGachaRsp::generated_message_descriptor_data());
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
