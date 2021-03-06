# krost

krost is a flexible, efficient, automated mechanism for serializing structured data in kafka protocol. Mainly affected by [Prost](https://github.com/tokio-rs/prost) (a protobuf implementation for rust).

This crate still have a lot of work to do, but it's a good start.

### Codegen example

`ProduceRequest` definition: 

```json
{
  "apiKey": 0,
  "type": "request",
  "listeners": ["zkBroker", "broker"],
  "name": "ProduceRequest",
  // Version 1 and 2 are the same as version 0.
  //
  // Version 3 adds the transactional ID, which is used for authorization when attempting to write
  // transactional data.  Version 3 also adds support for Kafka Message Format v2.
  //
  // Version 4 is the same as version 3, but the requestor must be prepared to handle a
  // KAFKA_STORAGE_ERROR. 
  //
  // Version 5 and 6 are the same as version 3.
  //
  // Starting in version 7, records can be produced using ZStandard compression.  See KIP-110.
  //
  // Starting in Version 8, response has RecordErrors and ErrorMEssage. See KIP-467.
  //
  // Version 9 enables flexible versions.
  "validVersions": "0-9",
  "flexibleVersions": "9+",
  "fields": [
    { "name": "TransactionalId", "type": "string", "versions": "3+", "nullableVersions": "3+", "default": "null", "entityType": "transactionalId",
      "about": "The transactional ID, or null if the producer is not transactional." },
    { "name": "Acks", "type": "int16", "versions": "0+",
      "about": "The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR." },
    { "name": "TimeoutMs", "type": "int32", "versions": "0+",
      "about": "The timeout to await a response in milliseconds." },
    { "name": "TopicData", "type": "[]TopicProduceData", "versions": "0+",
      "about": "Each topic to produce to.", "fields": [
      { "name": "Name", "type": "string", "versions": "0+", "entityType": "topicName", "mapKey": true,
        "about": "The topic name." },
      { "name": "PartitionData", "type": "[]PartitionProduceData", "versions": "0+",
        "about": "Each partition to produce to.", "fields": [
        { "name": "Index", "type": "int32", "versions": "0+",
          "about": "The partition index." },
        { "name": "Records", "type": "records", "versions": "0+", "nullableVersions": "0+",
          "about": "The record data to be produced." }
      ]}
    ]}
  ]
}
```

Output:

```rust
pub mod request {
    pub mod produce {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 0i16, versions = "0-9", flexible = "9+")]
        pub struct ProduceRequest {
            ///The transactional ID, or null if the producer is not transactional.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub transactional_id: Option<String>,
            ///The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
            #[kafka(versions = "0+")]
            pub acks: i16,
            ///The timeout to await a response in milliseconds.
            #[kafka(versions = "0+")]
            pub timeout_ms: i32,
            ///Each topic to produce to.
            #[kafka(versions = "0+")]
            pub topic_data: Vec<TopicProduceData>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionProduceData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub index: i32,
            ///The record data to be produced.
            #[kafka(versions = "0+", nullable = "0+")]
            pub records: Option<krost::record::RecordBatch>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicProduceData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: String,
            ///Each partition to produce to.
            #[kafka(versions = "0+")]
            pub partition_data: Vec<PartitionProduceData>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
}
```

and you could find whole code in `krost/tests/krost.rs`

### Acknowledgments

- [tychedelia/kafka-protocol-rs](https://github.com/tychedelia/kafka-protocol-rs)
- [gardnervickers/kafka-protocol-rs](https://github.com/gardnervickers/kafka-protocol-rs)
- [influxdata/rskafka](https://github.com/influxdata/rskafka)
