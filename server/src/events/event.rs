//! Contains functions for generating events and adding them to the address space of the server.
use opcua_types::{
    UAString, NodeId, DateTime, Guid, ByteString, LocalizedText, QualifiedName, Variant,
    ExtensionObject, ObjectId, ObjectTypeId, VariableTypeId,
    service_types::TimeZoneDataType,
};

use crate::address_space::{
    AddressSpace,
    object::ObjectBuilder,
    variable::VariableBuilder,
};

/// Events can implement this to populate themselves into the address space
pub trait Event {
    type Err;

    /// Test if the event is valid
    fn is_valid(&self) -> bool;

    /// Populates into the event into the address space. The event must be valid to be inserted
    fn insert<R, S, N>(self, node_id: &NodeId, browse_name: R, description: S, parent_node: N, address_space: &mut AddressSpace) -> Result<(), Self::Err>
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              N: Into<NodeId>;

    fn insert_property<R, S, V>(event_id: &NodeId, namespace: u16, browse_name: R, display_name: S, value: V, address_space: &mut AddressSpace)
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              V: Into<Variant>
    {
        let id = NodeId::next_numeric(namespace);
        VariableBuilder::new(&id, browse_name, display_name)
            .property_of(event_id.clone())
            .has_type_definition(VariableTypeId::PropertyType)
            .value(value)
            .insert(address_space);
    }
}

/// This corresponds to BaseEventType definition in OPC UA Part 5
pub struct BaseEventType {
    /// A unique identifier for an event, e.g. a GUID in a byte string
    pub event_id: ByteString,
    /// Event type describes the type of event
    pub event_type: NodeId,
    /// Source node identifies the node that the event originated from
    /// or null.
    pub source_node: NodeId,
    /// Source name provides the description of the source of the event,
    /// e.g. the display of the event source
    pub source_name: UAString,
    /// Time provides the time the event occurred. As close
    /// to the event generator as possible.
    pub time: DateTime,
    /// Receive time provides the time the OPC UA server received
    /// the event from the underlying device of another server.
    pub receive_time: DateTime,
    /// Local time (optional) is a structure containing
    /// the offset and daylightsaving flag.
    pub local_time: Option<TimeZoneDataType>,
    /// Message provides a human readable localizable text description
    /// of the event.
    pub message: LocalizedText,
    /// Severity is an indication of the urgency of the event. Values from 1 to 1000, with 1 as the lowest
    /// severity and 1000 being the highest. A value of 1000 would indicate an event of catastrophic nature.
    ///
    /// Guidance:
    ///
    /// * 801-1000 - High
    /// * 601-800 - Medium High
    /// * 401-600 - Medium
    /// * 201-400 - Medium Low
    /// * 1-200 - Low
    pub severity: u16,
}

impl Default for BaseEventType {
    fn default() -> Self {
        let now = DateTime::now();
        Self {
            event_id: Guid::new().into(),
            event_type: ObjectTypeId::BaseEventType.into(),
            source_node: NodeId::null(),
            source_name: UAString::null(),
            time: now.clone(),
            receive_time: now,
            local_time: None,
            message: LocalizedText::from(""),
            severity: 1,
        }
    }
}

impl Event for BaseEventType {
    type Err = ();

    fn is_valid(&self) -> bool {
        !self.event_id.is_null_or_empty() &&
            !self.event_type.is_null() &&
            self.severity >= 1 && self.severity <= 1000
    }

    fn insert<R, S, N>(self, node_id: &NodeId, browse_name: R, display_name: S, parent_node: N, address_space: &mut AddressSpace) -> Result<(), Self::Err>
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              N: Into<NodeId>
    {
        if self.is_valid() {
            // create an event object in a folder with the
            let namespace = node_id.namespace;

            ObjectBuilder::new(node_id, browse_name, display_name)
                .organized_by(parent_node)
                .has_type_definition(self.event_type.clone())
                .insert(address_space);

            // Mandatory properties
            Self::insert_property(node_id, namespace, "EventId", "EventId", self.event_id.clone(), address_space);
            Self::insert_property(node_id, namespace, "EventType", "EventType", self.event_type, address_space);
            Self::insert_property(node_id, namespace, "SourceNode", "SourceNode", self.source_node, address_space);
            Self::insert_property(node_id, namespace, "SourceName", "SourceName", self.source_name, address_space);
            Self::insert_property(node_id, namespace, "Time", "Time", self.time, address_space);
            Self::insert_property(node_id, namespace, "ReceiveTime", "ReceiveTime", self.receive_time, address_space);
            Self::insert_property(node_id, namespace, "Message", "Message", self.message, address_space);
            Self::insert_property(node_id, namespace, "Severity", "Severity", self.severity, address_space);

            // LocalTime is optional
            if let Some(ref local_time) = self.local_time {
                // Serialise to extension object
                let local_time = ExtensionObject::from_encodable(ObjectId::TimeZoneDataType_Encoding_DefaultBinary, local_time);
                Self::insert_property(node_id, namespace, "LocalTime", "LocalTime", local_time, address_space);
            }

            Ok(())
        } else {
            error!("Event is invalid and will not be inserted");
            Err(())
        }
    }
}