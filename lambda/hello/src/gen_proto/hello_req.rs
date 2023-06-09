// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.21.12
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

//! Generated file from `hello_req.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.request.HelloRequest)
pub struct HelloRequest {
    // message fields
    // @@protoc_insertion_point(field:hello_api.request.HelloRequest.api_req_type)
    pub api_req_type: ::protobuf::EnumOrUnknown<ApiReqType>,
    // message oneof groups
    pub request: ::std::option::Option<hello_request::Request>,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.request.HelloRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HelloRequest {
    fn default() -> &'a HelloRequest {
        <HelloRequest as ::protobuf::Message>::default_instance()
    }
}

impl HelloRequest {
    pub fn new() -> HelloRequest {
        ::std::default::Default::default()
    }

    // .hello_api.request.AddUserRequest add_user_request = 2;

    pub fn add_user_request(&self) -> &AddUserRequest {
        match self.request {
            ::std::option::Option::Some(hello_request::Request::AddUserRequest(ref v)) => v,
            _ => <AddUserRequest as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_add_user_request(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_add_user_request(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(hello_request::Request::AddUserRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add_user_request(&mut self, v: AddUserRequest) {
        self.request = ::std::option::Option::Some(hello_request::Request::AddUserRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_add_user_request(&mut self) -> &mut AddUserRequest {
        if let ::std::option::Option::Some(hello_request::Request::AddUserRequest(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(hello_request::Request::AddUserRequest(AddUserRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(hello_request::Request::AddUserRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_add_user_request(&mut self) -> AddUserRequest {
        if self.has_add_user_request() {
            match self.request.take() {
                ::std::option::Option::Some(hello_request::Request::AddUserRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            AddUserRequest::new()
        }
    }

    // .hello_api.request.GetUserNameRequest get_user_name_request = 3;

    pub fn get_user_name_request(&self) -> &GetUserNameRequest {
        match self.request {
            ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(ref v)) => v,
            _ => <GetUserNameRequest as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_get_user_name_request(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_user_name_request(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_user_name_request(&mut self, v: GetUserNameRequest) {
        self.request = ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_user_name_request(&mut self) -> &mut GetUserNameRequest {
        if let ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(GetUserNameRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_user_name_request(&mut self) -> GetUserNameRequest {
        if self.has_get_user_name_request() {
            match self.request.take() {
                ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            GetUserNameRequest::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "api_req_type",
            |m: &HelloRequest| { &m.api_req_type },
            |m: &mut HelloRequest| { &mut m.api_req_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, AddUserRequest>(
            "add_user_request",
            HelloRequest::has_add_user_request,
            HelloRequest::add_user_request,
            HelloRequest::mut_add_user_request,
            HelloRequest::set_add_user_request,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, GetUserNameRequest>(
            "get_user_name_request",
            HelloRequest::has_get_user_name_request,
            HelloRequest::get_user_name_request,
            HelloRequest::mut_get_user_name_request,
            HelloRequest::set_get_user_name_request,
        ));
        oneofs.push(hello_request::Request::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HelloRequest>(
            "HelloRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HelloRequest {
    const NAME: &'static str = "HelloRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.api_req_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.request = ::std::option::Option::Some(hello_request::Request::AddUserRequest(is.read_message()?));
                },
                26 => {
                    self.request = ::std::option::Option::Some(hello_request::Request::GetUserNameRequest(is.read_message()?));
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
        if self.api_req_type != ::protobuf::EnumOrUnknown::new(ApiReqType::API_REQ_TYPE_UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.api_req_type.value());
        }
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &hello_request::Request::AddUserRequest(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hello_request::Request::GetUserNameRequest(ref v) => {
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
        if self.api_req_type != ::protobuf::EnumOrUnknown::new(ApiReqType::API_REQ_TYPE_UNSPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.api_req_type))?;
        }
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &hello_request::Request::AddUserRequest(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &hello_request::Request::GetUserNameRequest(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> HelloRequest {
        HelloRequest::new()
    }

    fn clear(&mut self) {
        self.api_req_type = ::protobuf::EnumOrUnknown::new(ApiReqType::API_REQ_TYPE_UNSPECIFIED);
        self.request = ::std::option::Option::None;
        self.request = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HelloRequest {
        static instance: HelloRequest = HelloRequest {
            api_req_type: ::protobuf::EnumOrUnknown::from_i32(0),
            request: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HelloRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HelloRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HelloRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HelloRequest`
pub mod hello_request {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:hello_api.request.HelloRequest.request)
    pub enum Request {
        // @@protoc_insertion_point(oneof_field:hello_api.request.HelloRequest.add_user_request)
        AddUserRequest(super::AddUserRequest),
        // @@protoc_insertion_point(oneof_field:hello_api.request.HelloRequest.get_user_name_request)
        GetUserNameRequest(super::GetUserNameRequest),
    }

    impl ::protobuf::Oneof for Request {
    }

    impl ::protobuf::OneofFull for Request {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HelloRequest as ::protobuf::MessageFull>::descriptor().oneof_by_name("request").unwrap()).clone()
        }
    }

    impl Request {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Request>("request")
        }
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.request.AddUserRequest)
pub struct AddUserRequest {
    // message fields
    // @@protoc_insertion_point(field:hello_api.request.AddUserRequest.user_id)
    pub user_id: ::std::string::String,
    // @@protoc_insertion_point(field:hello_api.request.AddUserRequest.user_name)
    pub user_name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.request.AddUserRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddUserRequest {
    fn default() -> &'a AddUserRequest {
        <AddUserRequest as ::protobuf::Message>::default_instance()
    }
}

impl AddUserRequest {
    pub fn new() -> AddUserRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &AddUserRequest| { &m.user_id },
            |m: &mut AddUserRequest| { &mut m.user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_name",
            |m: &AddUserRequest| { &m.user_name },
            |m: &mut AddUserRequest| { &mut m.user_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddUserRequest>(
            "AddUserRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddUserRequest {
    const NAME: &'static str = "AddUserRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.user_id = is.read_string()?;
                },
                18 => {
                    self.user_name = is.read_string()?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if !self.user_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.user_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            os.write_string(2, &self.user_name)?;
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

    fn new() -> AddUserRequest {
        AddUserRequest::new()
    }

    fn clear(&mut self) {
        self.user_id.clear();
        self.user_name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddUserRequest {
        static instance: AddUserRequest = AddUserRequest {
            user_id: ::std::string::String::new(),
            user_name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddUserRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddUserRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddUserRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUserRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.request.GetUserNameRequest)
pub struct GetUserNameRequest {
    // message fields
    // @@protoc_insertion_point(field:hello_api.request.GetUserNameRequest.user_id)
    pub user_id: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.request.GetUserNameRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetUserNameRequest {
    fn default() -> &'a GetUserNameRequest {
        <GetUserNameRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetUserNameRequest {
    pub fn new() -> GetUserNameRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &GetUserNameRequest| { &m.user_id },
            |m: &mut GetUserNameRequest| { &mut m.user_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetUserNameRequest>(
            "GetUserNameRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetUserNameRequest {
    const NAME: &'static str = "GetUserNameRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.user_id = is.read_string()?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
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

    fn new() -> GetUserNameRequest {
        GetUserNameRequest::new()
    }

    fn clear(&mut self) {
        self.user_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetUserNameRequest {
        static instance: GetUserNameRequest = GetUserNameRequest {
            user_id: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetUserNameRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetUserNameRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetUserNameRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetUserNameRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:hello_api.request.ApiReqType)
pub enum ApiReqType {
    // @@protoc_insertion_point(enum_value:hello_api.request.ApiReqType.API_REQ_TYPE_UNSPECIFIED)
    API_REQ_TYPE_UNSPECIFIED = 0,
    // @@protoc_insertion_point(enum_value:hello_api.request.ApiReqType.API_REQ_TYPE_GET_USER)
    API_REQ_TYPE_GET_USER = 1,
    // @@protoc_insertion_point(enum_value:hello_api.request.ApiReqType.API_REQ_TYPE_ADD_USER)
    API_REQ_TYPE_ADD_USER = 2,
}

impl ::protobuf::Enum for ApiReqType {
    const NAME: &'static str = "ApiReqType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ApiReqType> {
        match value {
            0 => ::std::option::Option::Some(ApiReqType::API_REQ_TYPE_UNSPECIFIED),
            1 => ::std::option::Option::Some(ApiReqType::API_REQ_TYPE_GET_USER),
            2 => ::std::option::Option::Some(ApiReqType::API_REQ_TYPE_ADD_USER),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ApiReqType] = &[
        ApiReqType::API_REQ_TYPE_UNSPECIFIED,
        ApiReqType::API_REQ_TYPE_GET_USER,
        ApiReqType::API_REQ_TYPE_ADD_USER,
    ];
}

impl ::protobuf::EnumFull for ApiReqType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ApiReqType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ApiReqType {
    fn default() -> Self {
        ApiReqType::API_REQ_TYPE_UNSPECIFIED
    }
}

impl ApiReqType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ApiReqType>("ApiReqType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fhello_req.proto\x12\x11hello_api.request\"\x85\x02\n\x0cHelloReque\
    st\x12?\n\x0capi_req_type\x18\x01\x20\x01(\x0e2\x1d.hello_api.request.Ap\
    iReqTypeR\napiReqType\x12M\n\x10add_user_request\x18\x02\x20\x01(\x0b2!.\
    hello_api.request.AddUserRequestH\0R\x0eaddUserRequest\x12Z\n\x15get_use\
    r_name_request\x18\x03\x20\x01(\x0b2%.hello_api.request.GetUserNameReque\
    stH\0R\x12getUserNameRequestB\t\n\x07request\"F\n\x0eAddUserRequest\x12\
    \x17\n\x07user_id\x18\x01\x20\x01(\tR\x06userId\x12\x1b\n\tuser_name\x18\
    \x02\x20\x01(\tR\x08userName\"-\n\x12GetUserNameRequest\x12\x17\n\x07use\
    r_id\x18\x01\x20\x01(\tR\x06userId*`\n\nApiReqType\x12\x1c\n\x18API_REQ_\
    TYPE_UNSPECIFIED\x10\0\x12\x19\n\x15API_REQ_TYPE_GET_USER\x10\x01\x12\
    \x19\n\x15API_REQ_TYPE_ADD_USER\x10\x02b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(HelloRequest::generated_message_descriptor_data());
            messages.push(AddUserRequest::generated_message_descriptor_data());
            messages.push(GetUserNameRequest::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(ApiReqType::generated_enum_descriptor_data());
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
