// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
    extension_object::ExtensionObject,
    service_types::RegisteredServer,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterServer2Request {
    pub request_header: RequestHeader,
    pub server: RegisteredServer,
    pub discovery_configuration: Option<Vec<ExtensionObject>>,
}

impl MessageInfo for RegisterServer2Request {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisterServer2Request_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisterServer2Request> for RegisterServer2Request {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.server.byte_len();
        size += byte_len_array(&self.discovery_configuration);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.server.encode(stream)?;
        size += write_array(stream, &self.discovery_configuration)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let server = RegisteredServer::decode(stream, decoding_limits)?;
        let discovery_configuration: Option<Vec<ExtensionObject>> = read_array(stream, decoding_limits)?;
        Ok(RegisterServer2Request {
            request_header,
            server,
            discovery_configuration,
        })
    }
}