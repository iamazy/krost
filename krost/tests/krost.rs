#![allow(dead_code)]
use krost::KrostType;
use from_variants::FromVariants;
pub mod request {
    pub mod produce {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 0i16, versions = "0-9", flexible = "9+")]
        pub struct ProduceRequest {
            ///The transactional ID, or null if the producer is not transactional.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub transactional_id: krost::types::NullableString,
            ///The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
            #[kafka(versions = "0+")]
            pub acks: krost::types::Int16,
            ///The timeout to await a response in milliseconds.
            #[kafka(versions = "0+")]
            pub timeout_ms: krost::types::Int32,
            ///Each topic to produce to.
            #[kafka(versions = "0+")]
            pub topic_data: krost::types::Array<TopicProduceData>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionProduceData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub index: krost::types::Int32,
            ///The record data to be produced.
            #[kafka(versions = "0+", nullable = "0+")]
            pub records: krost::record::RecordBatch,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicProduceData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition to produce to.
            #[kafka(versions = "0+")]
            pub partition_data: krost::types::Array<PartitionProduceData>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod fetch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 1i16, versions = "0-13", flexible = "12+")]
        pub struct FetchRequest {
            ///The clusterId if known. This is used to validate metadata fetches prior to broker registration.
            #[kafka(
                versions = "12+",
                tagged = "12+",
                tag = 0i32,
                nullable = "12+",
                default = "null",
            )]
            pub cluster_id: krost::types::NullableString,
            ///The broker ID of the follower, of -1 if this request is from a consumer.
            #[kafka(versions = "0+")]
            pub replica_id: krost::types::Int32,
            ///The maximum time in milliseconds to wait for the response.
            #[kafka(versions = "0+")]
            pub max_wait_ms: krost::types::Int32,
            ///The minimum bytes to accumulate in the response.
            #[kafka(versions = "0+")]
            pub min_bytes: krost::types::Int32,
            ///The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
            #[kafka(versions = "3+", default = "0x7fffffff")]
            pub max_bytes: krost::types::Int32,
            ///This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
            #[kafka(versions = "4+", default = "0")]
            pub isolation_level: krost::types::Int8,
            ///The fetch session ID.
            #[kafka(versions = "7+", default = "0")]
            pub session_id: krost::types::Int32,
            ///The fetch session epoch, which is used for ordering requests in a session.
            #[kafka(versions = "7+", default = "-1")]
            pub session_epoch: krost::types::Int32,
            ///The topics to fetch.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<FetchTopic>,
            ///In an incremental fetch request, the partitions to remove.
            #[kafka(versions = "7+")]
            pub forgotten_topics_data: krost::types::Array<ForgottenTopic>,
            ///Rack ID of the consumer making this request
            #[kafka(versions = "11+", default = "")]
            pub rack_id: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct FetchPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition: krost::types::Int32,
            ///The current leader epoch of the partition.
            #[kafka(versions = "9+", default = "-1")]
            pub current_leader_epoch: krost::types::Int32,
            ///The message offset.
            #[kafka(versions = "0+")]
            pub fetch_offset: krost::types::Int64,
            ///The epoch of the last fetched record or -1 if there is none
            #[kafka(versions = "12+", default = "-1")]
            pub last_fetched_epoch: krost::types::Int32,
            ///The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
            #[kafka(versions = "5+", default = "-1")]
            pub log_start_offset: krost::types::Int64,
            ///The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
            #[kafka(versions = "0+")]
            pub partition_max_bytes: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct FetchTopic {
            ///The name of the topic to fetch.
            #[kafka(versions = "0-12")]
            pub topic: krost::types::String,
            ///The unique topic ID
            #[kafka(versions = "13+")]
            pub topic_id: krost::types::Uuid,
            ///The partitions to fetch.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<FetchPartition>,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ForgottenTopic {
            ///The topic name.
            #[kafka(versions = "7-12")]
            pub topic: krost::types::String,
            ///The unique topic ID
            #[kafka(versions = "13+")]
            pub topic_id: krost::types::Uuid,
            ///The partitions indexes to forget.
            #[kafka(versions = "7+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_offsets {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 2i16, versions = "0-7", flexible = "6+")]
        pub struct ListOffsetsRequest {
            ///The broker ID of the requestor, or -1 if this request is being made by a normal consumer.
            #[kafka(versions = "0+")]
            pub replica_id: krost::types::Int32,
            ///This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
            #[kafka(versions = "2+")]
            pub isolation_level: krost::types::Int8,
            ///Each topic in the request.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<ListOffsetsTopic>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListOffsetsPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The current leader epoch.
            #[kafka(versions = "4+", default = "-1")]
            pub current_leader_epoch: krost::types::Int32,
            ///The current timestamp.
            #[kafka(versions = "0+")]
            pub timestamp: krost::types::Int64,
            ///The maximum number of offsets to report.
            #[kafka(versions = "0", default = "1")]
            pub max_num_offsets: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListOffsetsTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition in the request.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<ListOffsetsPartition>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod metadata {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 3i16, versions = "0-12", flexible = "9+")]
        pub struct MetadataRequest {
            ///The topics to fetch metadata for.
            #[kafka(versions = "0+", nullable = "1+")]
            pub topics: krost::types::Array<MetadataRequestTopic>,
            ///If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
            #[kafka(versions = "4+", default = "true")]
            pub allow_auto_topic_creation: krost::types::Bool,
            ///Whether to include cluster authorized operations.
            #[kafka(versions = "8-10")]
            pub include_cluster_authorized_operations: krost::types::Bool,
            ///Whether to include topic authorized operations.
            #[kafka(versions = "8+")]
            pub include_topic_authorized_operations: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MetadataRequestTopic {
            ///The topic id.
            #[kafka(versions = "10+")]
            pub topic_id: krost::types::Uuid,
            ///The topic name.
            #[kafka(versions = "0+", nullable = "10+")]
            pub name: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod leader_and_isr {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 4i16, versions = "0-6", flexible = "4+")]
        pub struct LeaderAndIsrRequest {
            ///The current controller ID.
            #[kafka(versions = "0+")]
            pub controller_id: krost::types::Int32,
            ///The current controller epoch.
            #[kafka(versions = "0+")]
            pub controller_epoch: krost::types::Int32,
            ///The current broker epoch.
            #[kafka(versions = "2+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///The type that indicates whether all topics are included in the request
            #[kafka(versions = "5+")]
            pub r#type: krost::types::Int8,
            ///The state of each partition, in a v0 or v1 message.
            #[kafka(versions = "0-1")]
            pub ungrouped_partition_states: krost::types::Array<
                LeaderAndIsrPartitionState,
            >,
            ///Each topic.
            #[kafka(versions = "2+")]
            pub topic_states: krost::types::Array<LeaderAndIsrTopicState>,
            ///The current live leaders.
            #[kafka(versions = "0+")]
            pub live_leaders: krost::types::Array<LeaderAndIsrLiveLeader>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct LeaderAndIsrTopicState {
            ///The topic name.
            #[kafka(versions = "2+")]
            pub topic_name: krost::types::String,
            ///The unique topic ID.
            #[kafka(versions = "5+")]
            pub topic_id: krost::types::Uuid,
            ///The state of each partition
            #[kafka(versions = "2+")]
            pub partition_states: krost::types::Array<LeaderAndIsrPartitionState>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct LeaderAndIsrLiveLeader {
            ///The leader's broker ID.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The leader's hostname.
            #[kafka(versions = "0+")]
            pub host_name: krost::types::String,
            ///The leader's port.
            #[kafka(versions = "0+")]
            pub port: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod stop_replica {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 5i16, versions = "0-3", flexible = "2+")]
        pub struct StopReplicaRequest {
            ///The controller id.
            #[kafka(versions = "0+")]
            pub controller_id: krost::types::Int32,
            ///The controller epoch.
            #[kafka(versions = "0+")]
            pub controller_epoch: krost::types::Int32,
            ///The broker epoch.
            #[kafka(versions = "1+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///Whether these partitions should be deleted.
            #[kafka(versions = "0-2")]
            pub delete_partitions: krost::types::Bool,
            ///The partitions to stop.
            #[kafka(versions = "0")]
            pub ungrouped_partitions: krost::types::Array<StopReplicaPartitionV0>,
            ///The topics to stop.
            #[kafka(versions = "1-2")]
            pub topics: krost::types::Array<StopReplicaTopicV1>,
            ///Each topic.
            #[kafka(versions = "3+")]
            pub topic_states: krost::types::Array<StopReplicaTopicState>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct StopReplicaPartitionV0 {
            ///The topic name.
            #[kafka(versions = "0")]
            pub topic_name: krost::types::String,
            ///The partition index.
            #[kafka(versions = "0")]
            pub partition_index: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct StopReplicaTopicV1 {
            ///The topic name.
            #[kafka(versions = "1-2")]
            pub name: krost::types::String,
            ///The partition indexes.
            #[kafka(versions = "1-2")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct StopReplicaPartitionState {
            ///The partition index.
            #[kafka(versions = "3+")]
            pub partition_index: krost::types::Int32,
            ///The leader epoch.
            #[kafka(versions = "3+", default = "-1")]
            pub leader_epoch: krost::types::Int32,
            ///Whether this partition should be deleted.
            #[kafka(versions = "3+")]
            pub delete_partition: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct StopReplicaTopicState {
            ///The topic name.
            #[kafka(versions = "3+")]
            pub topic_name: krost::types::String,
            ///The state of each partition
            #[kafka(versions = "3+")]
            pub partition_states: krost::types::Array<StopReplicaPartitionState>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod update_metadata {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 6i16, versions = "0-7", flexible = "6+")]
        pub struct UpdateMetadataRequest {
            ///The controller id.
            #[kafka(versions = "0+")]
            pub controller_id: krost::types::Int32,
            ///The controller epoch.
            #[kafka(versions = "0+")]
            pub controller_epoch: krost::types::Int32,
            ///The broker epoch.
            #[kafka(versions = "5+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///In older versions of this RPC, each partition that we would like to update.
            #[kafka(versions = "0-4")]
            pub ungrouped_partition_states: krost::types::Array<
                UpdateMetadataPartitionState,
            >,
            ///In newer versions of this RPC, each topic that we would like to update.
            #[kafka(versions = "5+")]
            pub topic_states: krost::types::Array<UpdateMetadataTopicState>,
            #[kafka(versions = "0+")]
            pub live_brokers: krost::types::Array<UpdateMetadataBroker>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct UpdateMetadataTopicState {
            ///The topic name.
            #[kafka(versions = "5+")]
            pub topic_name: krost::types::String,
            ///The topic id.
            #[kafka(versions = "7+")]
            pub topic_id: krost::types::Uuid,
            ///The partition that we would like to update.
            #[kafka(versions = "5+")]
            pub partition_states: krost::types::Array<UpdateMetadataPartitionState>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct UpdateMetadataEndpoint {
            ///The port of this endpoint
            #[kafka(versions = "1+")]
            pub port: krost::types::Int32,
            ///The hostname of this endpoint
            #[kafka(versions = "1+")]
            pub host: krost::types::String,
            ///The listener name.
            #[kafka(versions = "3+")]
            pub listener: krost::types::String,
            ///The security protocol type.
            #[kafka(versions = "1+")]
            pub security_protocol: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct UpdateMetadataBroker {
            ///The broker id.
            #[kafka(versions = "0+")]
            pub id: krost::types::Int32,
            ///The broker hostname.
            #[kafka(versions = "0")]
            pub v0_host: krost::types::String,
            ///The broker port.
            #[kafka(versions = "0")]
            pub v0_port: krost::types::Int32,
            ///The broker endpoints.
            #[kafka(versions = "1+")]
            pub endpoints: krost::types::Array<UpdateMetadataEndpoint>,
            ///The rack which this broker belongs to.
            #[kafka(versions = "2+", nullable = "0+")]
            pub rack: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod controlled_shutdown {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 7i16, versions = "0-3", flexible = "3+")]
        pub struct ControlledShutdownRequest {
            ///The id of the broker for which controlled shutdown has been requested.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The broker epoch.
            #[kafka(versions = "2+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_commit {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 8i16, versions = "0-8", flexible = "8+")]
        pub struct OffsetCommitRequest {
            ///The unique group identifier.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The generation of the group.
            #[kafka(versions = "1+", default = "-1")]
            pub generation_id: krost::types::Int32,
            ///The member ID assigned by the group coordinator.
            #[kafka(versions = "1+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "7+", nullable = "7+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The time period in ms to retain the offset.
            #[kafka(versions = "2-4", default = "-1")]
            pub retention_time_ms: krost::types::Int64,
            ///The topics to commit offsets for.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetCommitRequestTopic>,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetCommitRequestPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The message offset to be committed.
            #[kafka(versions = "0+")]
            pub committed_offset: krost::types::Int64,
            ///The leader epoch of this partition.
            #[kafka(versions = "6+", default = "-1")]
            pub committed_leader_epoch: krost::types::Int32,
            ///The timestamp of the commit.
            #[kafka(versions = "1", default = "-1")]
            pub commit_timestamp: krost::types::Int64,
            ///Any associated metadata the client wants to keep.
            #[kafka(versions = "0+", nullable = "0+")]
            pub committed_metadata: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetCommitRequestTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition to commit offsets for.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OffsetCommitRequestPartition>,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_fetch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 9i16, versions = "0-8", flexible = "6+")]
        pub struct OffsetFetchRequest {
            ///The group to fetch offsets for.
            #[kafka(versions = "0-7")]
            pub group_id: krost::types::String,
            ///Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
            #[kafka(versions = "0-7", nullable = "2-7")]
            pub topics: krost::types::Array<OffsetFetchRequestTopic>,
            ///Each group we would like to fetch offsets for
            #[kafka(versions = "8+")]
            pub groups: krost::types::Array<OffsetFetchRequestGroup>,
            ///Whether broker should hold on returning unstable offsets but set a retriable error code for the partitions.
            #[kafka(versions = "7+", default = "false")]
            pub require_stable: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchRequestTopic {
            ///The topic name.
            #[kafka(versions = "0-7")]
            pub name: krost::types::String,
            ///The partition indexes we would like to fetch offsets for.
            #[kafka(versions = "0-7")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchRequestTopics {
            ///The topic name.
            #[kafka(versions = "8+")]
            pub name: krost::types::String,
            ///The partition indexes we would like to fetch offsets for.
            #[kafka(versions = "8+")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchRequestGroup {
            ///The group ID.
            #[kafka(versions = "8+")]
            pub group_id: krost::types::String,
            ///Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
            #[kafka(versions = "8+", nullable = "8+")]
            pub topics: krost::types::Array<OffsetFetchRequestTopics>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod find_coordinator {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 10i16, versions = "0-4", flexible = "3+")]
        pub struct FindCoordinatorRequest {
            ///The coordinator key.
            #[kafka(versions = "0-3")]
            pub key: krost::types::String,
            ///The coordinator key type. (Group, transaction, etc.)
            #[kafka(versions = "1+", default = "0")]
            pub key_type: krost::types::Int8,
            ///The coordinator keys.
            #[kafka(versions = "4+")]
            pub coordinator_keys: krost::types::Array<krost::types::String>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod join_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 11i16, versions = "0-9", flexible = "6+")]
        pub struct JoinGroupRequest {
            ///The group identifier.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
            #[kafka(versions = "0+")]
            pub session_timeout_ms: krost::types::Int32,
            ///The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
            #[kafka(versions = "1+", default = "-1")]
            pub rebalance_timeout_ms: krost::types::Int32,
            ///The member id assigned by the group coordinator.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The unique name the for class of protocols implemented by the group we want to join.
            #[kafka(versions = "0+")]
            pub protocol_type: krost::types::String,
            ///The list of protocols that the member supports.
            #[kafka(versions = "0+")]
            pub protocols: krost::types::Array<JoinGroupRequestProtocol>,
            ///The reason why the member (re-)joins the group.
            #[kafka(versions = "8+", nullable = "8+", default = "null")]
            pub reason: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct JoinGroupRequestProtocol {
            ///The protocol name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The protocol metadata.
            #[kafka(versions = "0+")]
            pub metadata: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod heartbeat {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 12i16, versions = "0-4", flexible = "4+")]
        pub struct HeartbeatRequest {
            ///The group id.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The generation of the group.
            #[kafka(versions = "0+")]
            pub generation_id: krost::types::Int32,
            ///The member ID.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod leave_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 13i16, versions = "0-5", flexible = "4+")]
        pub struct LeaveGroupRequest {
            ///The ID of the group to leave.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The member ID to remove from the group.
            #[kafka(versions = "0-2")]
            pub member_id: krost::types::String,
            ///List of leaving member identities.
            #[kafka(versions = "3+")]
            pub members: krost::types::Array<MemberIdentity>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MemberIdentity {
            ///The member ID to remove from the group.
            #[kafka(versions = "3+")]
            pub member_id: krost::types::String,
            ///The group instance ID to remove from the group.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The reason why the member left the group.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub reason: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sync_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 14i16, versions = "0-5", flexible = "4+")]
        pub struct SyncGroupRequest {
            ///The unique group identifier.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The generation of the group.
            #[kafka(versions = "0+")]
            pub generation_id: krost::types::Int32,
            ///The member ID assigned by the group.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The group protocol type.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub protocol_type: krost::types::NullableString,
            ///The group protocol name.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub protocol_name: krost::types::NullableString,
            ///Each assignment.
            #[kafka(versions = "0+")]
            pub assignments: krost::types::Array<SyncGroupRequestAssignment>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct SyncGroupRequestAssignment {
            ///The ID of the member to assign.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The member assignment.
            #[kafka(versions = "0+")]
            pub assignment: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 15i16, versions = "0-5", flexible = "5+")]
        pub struct DescribeGroupsRequest {
            ///The names of the groups to describe
            #[kafka(versions = "0+")]
            pub groups: krost::types::Array<krost::types::String>,
            ///Whether to include authorized operations.
            #[kafka(versions = "3+")]
            pub include_authorized_operations: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 16i16, versions = "0-4", flexible = "3+")]
        pub struct ListGroupsRequest {
            ///The states of the groups we want to list. If empty all groups are returned with their state.
            #[kafka(versions = "4+")]
            pub states_filter: krost::types::Array<krost::types::String>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sasl_handshake {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 17i16, versions = "0-1")]
        pub struct SaslHandshakeRequest {
            ///The SASL mechanism chosen by the client.
            #[kafka(versions = "0+")]
            pub mechanism: krost::types::String,
        }
    }
    pub mod api_versions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 18i16, versions = "0-3", flexible = "3+")]
        pub struct ApiVersionsRequest {
            ///The name of the client.
            #[kafka(versions = "3+")]
            pub client_software_name: krost::types::String,
            ///The version of the client.
            #[kafka(versions = "3+")]
            pub client_software_version: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_topics {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 19i16, versions = "0-7", flexible = "5+")]
        pub struct CreateTopicsRequest {
            ///The topics to create.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<CreatableTopic>,
            ///How long to wait in milliseconds before timing out the request.
            #[kafka(versions = "0+", default = "60000")]
            pub timeout_ms: krost::types::Int32,
            ///If true, check that the topics can be created as specified, but don't create anything.
            #[kafka(versions = "1+", default = "false")]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatableReplicaAssignment {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The brokers to place the partition on.
            #[kafka(versions = "0+")]
            pub broker_ids: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreateableTopicConfig {
            ///The configuration name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The configuration value.
            #[kafka(versions = "0+", nullable = "0+")]
            pub value: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatableTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The number of partitions to create in the topic, or -1 if we are either specifying a manual partition assignment or using the default partitions.
            #[kafka(versions = "0+")]
            pub num_partitions: krost::types::Int32,
            ///The number of replicas to create for each partition in the topic, or -1 if we are either specifying a manual partition assignment or using the default replication factor.
            #[kafka(versions = "0+")]
            pub replication_factor: krost::types::Int16,
            ///The manual partition assignment, or the empty array if we are using automatic assignment.
            #[kafka(versions = "0+")]
            pub assignments: krost::types::Array<CreatableReplicaAssignment>,
            ///The custom topic configurations to set.
            #[kafka(versions = "0+")]
            pub configs: krost::types::Array<CreateableTopicConfig>,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_topics {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 20i16, versions = "0-6", flexible = "4+")]
        pub struct DeleteTopicsRequest {
            ///The name or topic ID of the topic
            #[kafka(versions = "6+")]
            pub topics: krost::types::Array<DeleteTopicState>,
            ///The names of the topics to delete
            #[kafka(versions = "0-5")]
            pub topic_names: krost::types::Array<krost::types::String>,
            ///The length of time in milliseconds to wait for the deletions to complete.
            #[kafka(versions = "0+")]
            pub timeout_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteTopicState {
            ///The topic name
            #[kafka(versions = "6+", nullable = "6+", default = "null")]
            pub name: krost::types::NullableString,
            ///The unique topic ID
            #[kafka(versions = "6+")]
            pub topic_id: krost::types::Uuid,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_records {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 21i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteRecordsRequest {
            ///Each topic that we want to delete records from.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<DeleteRecordsTopic>,
            ///How long to wait for the deletion to complete, in milliseconds.
            #[kafka(versions = "0+")]
            pub timeout_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteRecordsPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The deletion offset.
            #[kafka(versions = "0+")]
            pub offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteRecordsTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition that we want to delete records from.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<DeleteRecordsPartition>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod init_producer_id {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 22i16, versions = "0-4", flexible = "2+")]
        pub struct InitProducerIdRequest {
            ///The transactional id, or null if the producer is not transactional.
            #[kafka(versions = "0+", nullable = "0+")]
            pub transactional_id: krost::types::NullableString,
            ///The time in ms to wait before aborting idle transactions sent by this producer. This is only relevant if a TransactionalId has been defined.
            #[kafka(versions = "0+")]
            pub transaction_timeout_ms: krost::types::Int32,
            ///The producer id. This is used to disambiguate requests if a transactional id is reused following its expiration.
            #[kafka(versions = "3+", default = "-1")]
            pub producer_id: krost::types::Int64,
            ///The producer's current epoch. This will be checked against the producer epoch on the broker, and the request will return an error if they do not match.
            #[kafka(versions = "3+", default = "-1")]
            pub producer_epoch: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_for_leader_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 23i16, versions = "0-4", flexible = "4+")]
        pub struct OffsetForLeaderEpochRequest {
            ///The broker ID of the follower, of -1 if this request is from a consumer.
            #[kafka(versions = "3+", default = -2f64)]
            pub replica_id: krost::types::Int32,
            ///Each topic to get offsets for.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetForLeaderTopic>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetForLeaderPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition: krost::types::Int32,
            ///An epoch used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
            #[kafka(versions = "2+", default = "-1")]
            pub current_leader_epoch: krost::types::Int32,
            ///The epoch to look up an offset for.
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetForLeaderTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            ///Each partition to get offsets for.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OffsetForLeaderPartition>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod add_partitions_to_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 24i16, versions = "0-3", flexible = "3+")]
        pub struct AddPartitionsToTxnRequest {
            ///The transactional id corresponding to the transaction.
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            ///Current producer id in use by the transactional id.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///Current epoch associated with the producer id.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The partitions to add to the transaction.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<AddPartitionsToTxnTopic>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AddPartitionsToTxnTopic {
            ///The name of the topic.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partition indexes to add to the transaction
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod add_offsets_to_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 25i16, versions = "0-3", flexible = "3+")]
        pub struct AddOffsetsToTxnRequest {
            ///The transactional id corresponding to the transaction.
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            ///Current producer id in use by the transactional id.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///Current epoch associated with the producer id.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The unique group identifier.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod end_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 26i16, versions = "0-3", flexible = "3+")]
        pub struct EndTxnRequest {
            ///The ID of the transaction to end.
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            ///The producer ID.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///The current epoch associated with the producer.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///True if the transaction was committed, false if it was aborted.
            #[kafka(versions = "0+")]
            pub committed: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod write_txn_markers {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 27i16, versions = "0-1", flexible = "1+")]
        pub struct WriteTxnMarkersRequest {
            ///The transaction markers to be written.
            #[kafka(versions = "0+")]
            pub markers: krost::types::Array<WritableTxnMarker>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct WritableTxnMarkerTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The indexes of the partitions to write transaction markers for.
            #[kafka(versions = "0+")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct WritableTxnMarker {
            ///The current producer ID.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///The current epoch associated with the producer ID.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
            #[kafka(versions = "0+")]
            pub transaction_result: krost::types::Bool,
            ///Each topic that we want to write transaction marker(s) for.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<WritableTxnMarkerTopic>,
            ///Epoch associated with the transaction state partition hosted by this transaction coordinator
            #[kafka(versions = "0+")]
            pub coordinator_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod txn_offset_commit {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 28i16, versions = "0-3", flexible = "3+")]
        pub struct TxnOffsetCommitRequest {
            ///The ID of the transaction.
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            ///The ID of the group.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The current producer ID in use by the transactional ID.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///The current epoch associated with the producer ID.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The generation of the consumer.
            #[kafka(versions = "3+", default = "-1")]
            pub generation_id: krost::types::Int32,
            ///The member ID assigned by the group coordinator.
            #[kafka(versions = "3+", default = "")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "3+", nullable = "3+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///Each topic that we want to commit offsets for.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TxnOffsetCommitRequestTopic>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TxnOffsetCommitRequestPartition {
            ///The index of the partition within the topic.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The message offset to be committed.
            #[kafka(versions = "0+")]
            pub committed_offset: krost::types::Int64,
            ///The leader epoch of the last consumed record.
            #[kafka(versions = "2+", default = "-1")]
            pub committed_leader_epoch: krost::types::Int32,
            ///Any associated metadata the client wants to keep.
            #[kafka(versions = "0+", nullable = "0+")]
            pub committed_metadata: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TxnOffsetCommitRequestTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partitions inside the topic that we want to committ offsets for.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<TxnOffsetCommitRequestPartition>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 29i16, versions = "0-2", flexible = "2+")]
        pub struct DescribeAclsRequest {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type_filter: krost::types::Int8,
            ///The resource name, or null to match any resource name.
            #[kafka(versions = "0+", nullable = "0+")]
            pub resource_name_filter: krost::types::NullableString,
            ///The resource pattern to match.
            #[kafka(versions = "1+", default = "3")]
            pub pattern_type_filter: krost::types::Int8,
            ///The principal to match, or null to match any principal.
            #[kafka(versions = "0+", nullable = "0+")]
            pub principal_filter: krost::types::NullableString,
            ///The host to match, or null to match any host.
            #[kafka(versions = "0+", nullable = "0+")]
            pub host_filter: krost::types::NullableString,
            ///The operation to match.
            #[kafka(versions = "0+")]
            pub operation: krost::types::Int8,
            ///The permission type to match.
            #[kafka(versions = "0+")]
            pub permission_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 30i16, versions = "0-2", flexible = "2+")]
        pub struct CreateAclsRequest {
            ///The ACLs that we want to create.
            #[kafka(versions = "0+")]
            pub creations: krost::types::Array<AclCreation>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AclCreation {
            ///The type of the resource.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name for the ACL.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The pattern type for the ACL.
            #[kafka(versions = "1+", default = "3")]
            pub resource_pattern_type: krost::types::Int8,
            ///The principal for the ACL.
            #[kafka(versions = "0+")]
            pub principal: krost::types::String,
            ///The host for the ACL.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The operation type for the ACL (read, write, etc.).
            #[kafka(versions = "0+")]
            pub operation: krost::types::Int8,
            ///The permission type for the ACL (allow, deny, etc.).
            #[kafka(versions = "0+")]
            pub permission_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 31i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteAclsRequest {
            ///The filters to use when deleting ACLs.
            #[kafka(versions = "0+")]
            pub filters: krost::types::Array<DeleteAclsFilter>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteAclsFilter {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type_filter: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+", nullable = "0+")]
            pub resource_name_filter: krost::types::NullableString,
            ///The pattern type.
            #[kafka(versions = "1+", default = "3")]
            pub pattern_type_filter: krost::types::Int8,
            ///The principal filter, or null to accept all principals.
            #[kafka(versions = "0+", nullable = "0+")]
            pub principal_filter: krost::types::NullableString,
            ///The host filter, or null to accept all hosts.
            #[kafka(versions = "0+", nullable = "0+")]
            pub host_filter: krost::types::NullableString,
            ///The ACL operation.
            #[kafka(versions = "0+")]
            pub operation: krost::types::Int8,
            ///The permission type.
            #[kafka(versions = "0+")]
            pub permission_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 32i16, versions = "0-4", flexible = "4+")]
        pub struct DescribeConfigsRequest {
            ///The resources whose configurations we want to describe.
            #[kafka(versions = "0+")]
            pub resources: krost::types::Array<DescribeConfigsResource>,
            ///True if we should include all synonyms.
            #[kafka(versions = "1+", default = "false")]
            pub include_synonyms: krost::types::Bool,
            ///True if we should include configuration documentation.
            #[kafka(versions = "3+", default = "false")]
            pub include_documentation: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeConfigsResource {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The configuration keys to list, or null to list all configuration keys.
            #[kafka(versions = "0+", nullable = "0+")]
            pub configuration_keys: krost::types::Array<krost::types::NullableString>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 33i16, versions = "0-2", flexible = "2+")]
        pub struct AlterConfigsRequest {
            ///The updates for each resource.
            #[kafka(versions = "0+")]
            pub resources: krost::types::Array<AlterConfigsResource>,
            ///True if we should validate the request, but not change the configurations.
            #[kafka(versions = "0+")]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterableConfig {
            ///The configuration key name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The value to set for the configuration key.
            #[kafka(versions = "0+", nullable = "0+")]
            pub value: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterConfigsResource {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The configurations.
            #[kafka(versions = "0+")]
            pub configs: krost::types::Array<AlterableConfig>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_replica_log_dirs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 34i16, versions = "0-2", flexible = "2+")]
        pub struct AlterReplicaLogDirsRequest {
            ///The alterations to make for each directory.
            #[kafka(versions = "0+")]
            pub dirs: krost::types::Array<AlterReplicaLogDir>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterReplicaLogDirTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partition indexes.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterReplicaLogDir {
            ///The absolute directory path.
            #[kafka(versions = "0+")]
            pub path: krost::types::String,
            ///The topics to add to the directory.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<AlterReplicaLogDirTopic>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_log_dirs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 35i16, versions = "0-3", flexible = "2+")]
        pub struct DescribeLogDirsRequest {
            ///Each topic that we want to describe log directories for, or null for all topics.
            #[kafka(versions = "0+", nullable = "0+")]
            pub topics: krost::types::Array<DescribableLogDirTopic>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribableLogDirTopic {
            ///The topic name
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            ///The partition indexes.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sasl_authenticate {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 36i16, versions = "0-2", flexible = "2+")]
        pub struct SaslAuthenticateRequest {
            ///The SASL authentication bytes from the client, as defined by the SASL mechanism.
            #[kafka(versions = "0+")]
            pub auth_bytes: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_partitions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 37i16, versions = "0-3", flexible = "2+")]
        pub struct CreatePartitionsRequest {
            ///Each topic that we want to create new partitions inside.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<CreatePartitionsTopic>,
            ///The time in ms to wait for the partitions to be created.
            #[kafka(versions = "0+")]
            pub timeout_ms: krost::types::Int32,
            ///If true, then validate the request, but don't actually increase the number of partitions.
            #[kafka(versions = "0+")]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatePartitionsAssignment {
            ///The assigned broker IDs.
            #[kafka(versions = "0+")]
            pub broker_ids: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatePartitionsTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The new partition count.
            #[kafka(versions = "0+")]
            pub count: krost::types::Int32,
            ///The new partition assignments.
            #[kafka(versions = "0+", nullable = "0+")]
            pub assignments: krost::types::Array<CreatePartitionsAssignment>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 38i16, versions = "0-2", flexible = "2+")]
        pub struct CreateDelegationTokenRequest {
            ///A list of those who are allowed to renew this token before it expires.
            #[kafka(versions = "0+")]
            pub renewers: krost::types::Array<CreatableRenewers>,
            ///The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
            #[kafka(versions = "0+")]
            pub max_lifetime_ms: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatableRenewers {
            ///The type of the Kafka principal.
            #[kafka(versions = "0+")]
            pub principal_type: krost::types::String,
            ///The name of the Kafka principal.
            #[kafka(versions = "0+")]
            pub principal_name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod renew_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 39i16, versions = "0-2", flexible = "2+")]
        pub struct RenewDelegationTokenRequest {
            ///The HMAC of the delegation token to be renewed.
            #[kafka(versions = "0+")]
            pub hmac: krost::types::Bytes,
            ///The renewal time period in milliseconds.
            #[kafka(versions = "0+")]
            pub renew_period_ms: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod expire_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 40i16, versions = "0-2", flexible = "2+")]
        pub struct ExpireDelegationTokenRequest {
            ///The HMAC of the delegation token to be expired.
            #[kafka(versions = "0+")]
            pub hmac: krost::types::Bytes,
            ///The expiry time period in milliseconds.
            #[kafka(versions = "0+")]
            pub expiry_time_period_ms: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 41i16, versions = "0-2", flexible = "2+")]
        pub struct DescribeDelegationTokenRequest {
            ///Each owner that we want to describe delegation tokens for, or null to describe all tokens.
            #[kafka(versions = "0+", nullable = "0+")]
            pub owners: krost::types::Array<DescribeDelegationTokenOwner>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeDelegationTokenOwner {
            ///The owner principal type.
            #[kafka(versions = "0+")]
            pub principal_type: krost::types::String,
            ///The owner principal name.
            #[kafka(versions = "0+")]
            pub principal_name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 42i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteGroupsRequest {
            ///The group names to delete.
            #[kafka(versions = "0+")]
            pub groups_names: krost::types::Array<krost::types::String>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod elect_leaders {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 43i16, versions = "0-2", flexible = "2+")]
        pub struct ElectLeadersRequest {
            ///Type of elections to conduct for the partition. A value of '0' elects the preferred replica. A value of '1' elects the first live replica if there are no in-sync replica.
            #[kafka(versions = "1+")]
            pub election_type: krost::types::Int8,
            ///The topic partitions to elect leaders.
            #[kafka(versions = "0+", nullable = "0+")]
            pub topic_partitions: krost::types::Array<TopicPartitions>,
            ///The time in ms to wait for the election to complete.
            #[kafka(versions = "0+", default = "60000")]
            pub timeout_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicPartitions {
            ///The name of a topic.
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            ///The partitions of this topic whose leader should be elected.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod incremental_alter_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 44i16, versions = "0-1", flexible = "1+")]
        pub struct IncrementalAlterConfigsRequest {
            ///The incremental updates for each resource.
            #[kafka(versions = "0+")]
            pub resources: krost::types::Array<AlterConfigsResource>,
            ///True if we should validate the request, but not change the configurations.
            #[kafka(versions = "0+")]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterableConfig {
            ///The configuration key name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The type (Set, Delete, Append, Subtract) of operation.
            #[kafka(versions = "0+")]
            pub config_operation: krost::types::Int8,
            ///The value to set for the configuration key.
            #[kafka(versions = "0+", nullable = "0+")]
            pub value: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterConfigsResource {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The configurations.
            #[kafka(versions = "0+")]
            pub configs: krost::types::Array<AlterableConfig>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_partition_reassignments {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 45i16, versions = "0", flexible = "0+")]
        pub struct AlterPartitionReassignmentsRequest {
            ///The time in ms to wait for the request to complete.
            #[kafka(versions = "0+", default = "60000")]
            pub timeout_ms: krost::types::Int32,
            ///The topics to reassign.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<ReassignableTopic>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ReassignablePartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The replicas to place the partitions on, or null to cancel a pending reassignment for this partition.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub replicas: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ReassignableTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partitions to reassign.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<ReassignablePartition>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_partition_reassignments {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 46i16, versions = "0", flexible = "0+")]
        pub struct ListPartitionReassignmentsRequest {
            ///The time in ms to wait for the request to complete.
            #[kafka(versions = "0+", default = "60000")]
            pub timeout_ms: krost::types::Int32,
            ///The topics to list partition reassignments for, or null to list everything.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub topics: krost::types::Array<ListPartitionReassignmentsTopics>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListPartitionReassignmentsTopics {
            ///The topic name
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partitions to list partition reassignments for.
            #[kafka(versions = "0+")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_delete {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 47i16, versions = "0")]
        pub struct OffsetDeleteRequest {
            ///The unique group identifier.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The topics to delete offsets for
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetDeleteRequestTopic>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetDeleteRequestPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetDeleteRequestTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition to delete offsets for.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OffsetDeleteRequestPartition>,
        }
    }
    pub mod describe_client_quotas {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 48i16, versions = "0-1", flexible = "1+")]
        pub struct DescribeClientQuotasRequest {
            ///Filter components to apply to quota entities.
            #[kafka(versions = "0+")]
            pub components: krost::types::Array<ComponentData>,
            ///Whether the match is strict, i.e. should exclude entities with unspecified entity types.
            #[kafka(versions = "0+")]
            pub strict: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ComponentData {
            ///The entity type that the filter component applies to.
            #[kafka(versions = "0+")]
            pub entity_type: krost::types::String,
            ///How to match the entity {0 = exact name, 1 = default name, 2 = any specified name}.
            #[kafka(versions = "0+")]
            pub match_type: krost::types::Int8,
            ///The string to match against, or null if unused for the match type.
            #[kafka(versions = "0+", nullable = "0+")]
            pub r#match: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_client_quotas {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 49i16, versions = "0-1", flexible = "1+")]
        pub struct AlterClientQuotasRequest {
            ///The quota configuration entries to alter.
            #[kafka(versions = "0+")]
            pub entries: krost::types::Array<EntryData>,
            ///Whether the alteration should be validated, but not performed.
            #[kafka(versions = "0+")]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(versions = "0+")]
            pub entity_type: krost::types::String,
            ///The name of the entity, or null if the default.
            #[kafka(versions = "0+", nullable = "0+")]
            pub entity_name: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OpData {
            ///The quota configuration key.
            #[kafka(versions = "0+")]
            pub key: krost::types::String,
            ///The value to set, otherwise ignored if the value is to be removed.
            #[kafka(versions = "0+")]
            pub value: float64,
            ///Whether the quota configuration value should be removed, otherwise set.
            #[kafka(versions = "0+")]
            pub remove: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntryData {
            ///The quota entity to alter.
            #[kafka(versions = "0+")]
            pub entity: krost::types::Array<EntityData>,
            ///An individual quota configuration entry to alter.
            #[kafka(versions = "0+")]
            pub ops: krost::types::Array<OpData>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_user_scram_credentials {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 50i16, versions = "0", flexible = "0+")]
        pub struct DescribeUserScramCredentialsRequest {
            ///The users to describe, or null/empty to describe all users.
            #[kafka(versions = "0+", nullable = "0+")]
            pub users: krost::types::Array<UserName>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct UserName {
            ///The user name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_user_scram_credentials {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 51i16, versions = "0", flexible = "0+")]
        pub struct AlterUserScramCredentialsRequest {
            ///The SCRAM credentials to remove.
            #[kafka(versions = "0+")]
            pub deletions: krost::types::Array<ScramCredentialDeletion>,
            ///The SCRAM credentials to update/insert.
            #[kafka(versions = "0+")]
            pub upsertions: krost::types::Array<ScramCredentialUpsertion>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ScramCredentialDeletion {
            ///The user name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The SCRAM mechanism.
            #[kafka(versions = "0+")]
            pub mechanism: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ScramCredentialUpsertion {
            ///The user name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The SCRAM mechanism.
            #[kafka(versions = "0+")]
            pub mechanism: krost::types::Int8,
            ///The number of iterations.
            #[kafka(versions = "0+")]
            pub iterations: krost::types::Int32,
            ///A random salt generated by the client.
            #[kafka(versions = "0+")]
            pub salt: krost::types::Bytes,
            ///The salted password.
            #[kafka(versions = "0+")]
            pub salted_password: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod vote {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 52i16, versions = "0", flexible = "0+")]
        pub struct VoteRequest {
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub cluster_id: krost::types::NullableString,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The bumped epoch of the candidate sending the request
            #[kafka(versions = "0+")]
            pub candidate_epoch: krost::types::Int32,
            ///The ID of the voter sending the request
            #[kafka(versions = "0+")]
            pub candidate_id: krost::types::Int32,
            ///The epoch of the last record written to the metadata log
            #[kafka(versions = "0+")]
            pub last_offset_epoch: krost::types::Int32,
            ///The offset of the last record written to the metadata log
            #[kafka(versions = "0+")]
            pub last_offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod begin_quorum_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 53i16, versions = "0")]
        pub struct BeginQuorumEpochRequest {
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub cluster_id: krost::types::NullableString,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The ID of the newly elected leader
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The epoch of the newly elected leader
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
        }
    }
    pub mod end_quorum_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 54i16, versions = "0")]
        pub struct EndQuorumEpochRequest {
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub cluster_id: krost::types::NullableString,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The current leader ID that is resigning
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The current epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///A sorted list of preferred successors to start the election
            #[kafka(versions = "0+")]
            pub preferred_successors: krost::types::Array<krost::types::Int32>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
        }
    }
    pub mod describe_quorum {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 55i16, versions = "0", flexible = "0+")]
        pub struct DescribeQuorumRequest {
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_partition {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 56i16, versions = "0-1", flexible = "0+")]
        pub struct AlterPartitionRequest {
            ///The ID of the requesting broker
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The epoch of the requesting broker
            #[kafka(versions = "0+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The leader epoch of this partition
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///The ISR for this partition
            #[kafka(versions = "0+")]
            pub new_isr: krost::types::Array<krost::types::Int32>,
            ///1 if the partition is recovering from an unclean leader election; 0 otherwise.
            #[kafka(versions = "1+", default = "0")]
            pub leader_recovery_state: krost::types::Int8,
            ///The expected epoch of the partition which is being updated. For legacy cluster this is the ZkVersion in the LeaderAndIsr request.
            #[kafka(versions = "0+")]
            pub partition_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The name of the topic to alter ISRs for
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod update_features {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 57i16, versions = "0-1", flexible = "0+")]
        pub struct UpdateFeaturesRequest {
            ///How long to wait in milliseconds before timing out the request.
            #[kafka(versions = "0+", default = "60000")]
            pub timeout_ms: krost::types::Int32,
            ///The list of updates to finalized features.
            #[kafka(versions = "0+")]
            pub feature_updates: krost::types::Array<FeatureUpdateKey>,
            ///True if we should validate the request, but not perform the upgrade or downgrade.
            #[kafka(versions = "1+", default = false)]
            pub validate_only: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct FeatureUpdateKey {
            ///The name of the finalized feature to be updated.
            #[kafka(versions = "0+")]
            pub feature: krost::types::String,
            ///The new maximum version level for the finalized feature. A value >= 1 is valid. A value < 1, is special, and can be used to request the deletion of the finalized feature.
            #[kafka(versions = "0+")]
            pub max_version_level: krost::types::Int16,
            ///DEPRECATED in version 1 (see DowngradeType). When set to true, the finalized feature version level is allowed to be downgraded/deleted. The downgrade request will fail if the new maximum version level is a value that's not lower than the existing maximum finalized version level.
            #[kafka(versions = "0")]
            pub allow_downgrade: krost::types::Bool,
            ///Determine which type of upgrade will be performed: 1 will perform an upgrade only (default), 2 is safe downgrades only (lossless), 3 is unsafe downgrades (lossy).
            #[kafka(versions = "1+", default = 1f64)]
            pub upgrade_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod envelope {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 58i16, versions = "0", flexible = "0+")]
        pub struct EnvelopeRequest {
            ///The embedded request header and data.
            #[kafka(versions = "0+")]
            pub request_data: krost::types::Bytes,
            ///Value of the initial client principal when the request is redirected by a broker.
            #[kafka(versions = "0+", nullable = "0+")]
            pub request_principal: krost::types::NullableBytes,
            ///The original client's address in bytes.
            #[kafka(versions = "0+")]
            pub client_host_address: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod fetch_snapshot {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 59i16, versions = "0", flexible = "0+")]
        pub struct FetchSnapshotRequest {
            ///The clusterId if known, this is used to validate metadata fetches prior to broker registration
            #[kafka(
                versions = "0+",
                tagged = "0+",
                tag = 0i32,
                nullable = "0+",
                default = "null",
            )]
            pub cluster_id: krost::types::NullableString,
            ///The broker ID of the follower
            #[kafka(versions = "0+", default = "-1")]
            pub replica_id: krost::types::Int32,
            ///The maximum bytes to fetch from all of the snapshots
            #[kafka(versions = "0+", default = "0x7fffffff")]
            pub max_bytes: krost::types::Int32,
            ///The topics to fetch
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicSnapshot>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct SnapshotId {
            #[kafka(versions = "0+")]
            pub end_offset: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionSnapshot {
            ///The partition index
            #[kafka(versions = "0+")]
            pub partition: krost::types::Int32,
            ///The current leader epoch of the partition, -1 for unknown leader epoch
            #[kafka(versions = "0+")]
            pub current_leader_epoch: krost::types::Int32,
            ///The snapshot endOffset and epoch to fetch
            #[kafka(versions = "0+")]
            pub snapshot_id: SnapshotId,
            ///The byte position within the snapshot to start fetching from
            #[kafka(versions = "0+")]
            pub position: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicSnapshot {
            ///The name of the topic to fetch
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partitions to fetch
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionSnapshot>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_cluster {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 60i16, versions = "0", flexible = "0+")]
        pub struct DescribeClusterRequest {
            ///Whether to include cluster authorized operations.
            #[kafka(versions = "0+")]
            pub include_cluster_authorized_operations: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_producers {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 61i16, versions = "0", flexible = "0+")]
        pub struct DescribeProducersRequest {
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicRequest>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicRequest {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The indexes of the partitions to list producers for.
            #[kafka(versions = "0+")]
            pub partition_indexes: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod broker_registration {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 62i16, versions = "0", flexible = "0+")]
        pub struct BrokerRegistrationRequest {
            ///The broker ID.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The cluster id of the broker process.
            #[kafka(versions = "0+")]
            pub cluster_id: krost::types::String,
            ///The incarnation id of the broker process.
            #[kafka(versions = "0+")]
            pub incarnation_id: krost::types::Uuid,
            ///The listeners of this broker
            #[kafka(versions = "0+")]
            pub listeners: krost::types::Array<Listener>,
            ///The features on this broker
            #[kafka(versions = "0+")]
            pub features: krost::types::Array<Feature>,
            ///The rack which this broker is in.
            #[kafka(versions = "0+", nullable = "0+")]
            pub rack: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct Listener {
            ///The name of the endpoint.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The hostname.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The port.
            #[kafka(versions = "0+")]
            pub port: uint16,
            ///The security protocol.
            #[kafka(versions = "0+")]
            pub security_protocol: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct Feature {
            ///The feature name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The minimum supported feature level.
            #[kafka(versions = "0+")]
            pub min_supported_version: krost::types::Int16,
            ///The maximum supported feature level.
            #[kafka(versions = "0+")]
            pub max_supported_version: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod broker_heartbeat {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 63i16, versions = "0", flexible = "0+")]
        pub struct BrokerHeartbeatRequest {
            ///The broker ID.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The broker epoch.
            #[kafka(versions = "0+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///The highest metadata offset which the broker has reached.
            #[kafka(versions = "0+")]
            pub current_metadata_offset: krost::types::Int64,
            ///True if the broker wants to be fenced, false otherwise.
            #[kafka(versions = "0+")]
            pub want_fence: krost::types::Bool,
            ///True if the broker wants to be shut down, false otherwise.
            #[kafka(versions = "0+")]
            pub want_shut_down: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod unregister_broker {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 64i16, versions = "0", flexible = "0+")]
        pub struct UnregisterBrokerRequest {
            ///The broker ID to unregister.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_transactions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 65i16, versions = "0", flexible = "0+")]
        pub struct DescribeTransactionsRequest {
            ///Array of transactionalIds to include in describe results. If empty, then no results will be returned.
            #[kafka(versions = "0+")]
            pub transactional_ids: krost::types::Array<krost::types::String>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_transactions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 66i16, versions = "0", flexible = "0+")]
        pub struct ListTransactionsRequest {
            ///The transaction states to filter by: if empty, all transactions are returned; if non-empty, then only transactions matching one of the filtered states will be returned
            #[kafka(versions = "0+")]
            pub state_filters: krost::types::Array<krost::types::String>,
            ///The producerIds to filter by: if empty, all transactions will be returned; if non-empty, only transactions which match one of the filtered producerIds will be returned
            #[kafka(versions = "0+")]
            pub producer_id_filters: krost::types::Array<krost::types::Int64>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod allocate_producer_ids {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 67i16, versions = "0", flexible = "0+")]
        pub struct AllocateProducerIdsRequest {
            ///The ID of the requesting broker
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The epoch of the requesting broker
            #[kafka(versions = "0+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, FromVariants)]
    pub enum RequestBody {
        ProduceRequest(produce::ProduceRequest),
        FetchRequest(fetch::FetchRequest),
        ListOffsetsRequest(list_offsets::ListOffsetsRequest),
        MetadataRequest(metadata::MetadataRequest),
        LeaderAndIsrRequest(leader_and_isr::LeaderAndIsrRequest),
        StopReplicaRequest(stop_replica::StopReplicaRequest),
        UpdateMetadataRequest(update_metadata::UpdateMetadataRequest),
        ControlledShutdownRequest(controlled_shutdown::ControlledShutdownRequest),
        OffsetCommitRequest(offset_commit::OffsetCommitRequest),
        OffsetFetchRequest(offset_fetch::OffsetFetchRequest),
        FindCoordinatorRequest(find_coordinator::FindCoordinatorRequest),
        JoinGroupRequest(join_group::JoinGroupRequest),
        HeartbeatRequest(heartbeat::HeartbeatRequest),
        LeaveGroupRequest(leave_group::LeaveGroupRequest),
        SyncGroupRequest(sync_group::SyncGroupRequest),
        DescribeGroupsRequest(describe_groups::DescribeGroupsRequest),
        ListGroupsRequest(list_groups::ListGroupsRequest),
        SaslHandshakeRequest(sasl_handshake::SaslHandshakeRequest),
        ApiVersionsRequest(api_versions::ApiVersionsRequest),
        CreateTopicsRequest(create_topics::CreateTopicsRequest),
        DeleteTopicsRequest(delete_topics::DeleteTopicsRequest),
        DeleteRecordsRequest(delete_records::DeleteRecordsRequest),
        InitProducerIdRequest(init_producer_id::InitProducerIdRequest),
        OffsetForLeaderEpochRequest(
            offset_for_leader_epoch::OffsetForLeaderEpochRequest,
        ),
        AddPartitionsToTxnRequest(add_partitions_to_txn::AddPartitionsToTxnRequest),
        AddOffsetsToTxnRequest(add_offsets_to_txn::AddOffsetsToTxnRequest),
        EndTxnRequest(end_txn::EndTxnRequest),
        WriteTxnMarkersRequest(write_txn_markers::WriteTxnMarkersRequest),
        TxnOffsetCommitRequest(txn_offset_commit::TxnOffsetCommitRequest),
        DescribeAclsRequest(describe_acls::DescribeAclsRequest),
        CreateAclsRequest(create_acls::CreateAclsRequest),
        DeleteAclsRequest(delete_acls::DeleteAclsRequest),
        DescribeConfigsRequest(describe_configs::DescribeConfigsRequest),
        AlterConfigsRequest(alter_configs::AlterConfigsRequest),
        AlterReplicaLogDirsRequest(alter_replica_log_dirs::AlterReplicaLogDirsRequest),
        DescribeLogDirsRequest(describe_log_dirs::DescribeLogDirsRequest),
        SaslAuthenticateRequest(sasl_authenticate::SaslAuthenticateRequest),
        CreatePartitionsRequest(create_partitions::CreatePartitionsRequest),
        CreateDelegationTokenRequest(
            create_delegation_token::CreateDelegationTokenRequest,
        ),
        RenewDelegationTokenRequest(renew_delegation_token::RenewDelegationTokenRequest),
        ExpireDelegationTokenRequest(
            expire_delegation_token::ExpireDelegationTokenRequest,
        ),
        DescribeDelegationTokenRequest(
            describe_delegation_token::DescribeDelegationTokenRequest,
        ),
        DeleteGroupsRequest(delete_groups::DeleteGroupsRequest),
        ElectLeadersRequest(elect_leaders::ElectLeadersRequest),
        IncrementalAlterConfigsRequest(
            incremental_alter_configs::IncrementalAlterConfigsRequest,
        ),
        AlterPartitionReassignmentsRequest(
            alter_partition_reassignments::AlterPartitionReassignmentsRequest,
        ),
        ListPartitionReassignmentsRequest(
            list_partition_reassignments::ListPartitionReassignmentsRequest,
        ),
        OffsetDeleteRequest(offset_delete::OffsetDeleteRequest),
        DescribeClientQuotasRequest(describe_client_quotas::DescribeClientQuotasRequest),
        AlterClientQuotasRequest(alter_client_quotas::AlterClientQuotasRequest),
        DescribeUserScramCredentialsRequest(
            describe_user_scram_credentials::DescribeUserScramCredentialsRequest,
        ),
        AlterUserScramCredentialsRequest(
            alter_user_scram_credentials::AlterUserScramCredentialsRequest,
        ),
        VoteRequest(vote::VoteRequest),
        BeginQuorumEpochRequest(begin_quorum_epoch::BeginQuorumEpochRequest),
        EndQuorumEpochRequest(end_quorum_epoch::EndQuorumEpochRequest),
        DescribeQuorumRequest(describe_quorum::DescribeQuorumRequest),
        AlterPartitionRequest(alter_partition::AlterPartitionRequest),
        UpdateFeaturesRequest(update_features::UpdateFeaturesRequest),
        EnvelopeRequest(envelope::EnvelopeRequest),
        FetchSnapshotRequest(fetch_snapshot::FetchSnapshotRequest),
        DescribeClusterRequest(describe_cluster::DescribeClusterRequest),
        DescribeProducersRequest(describe_producers::DescribeProducersRequest),
        BrokerRegistrationRequest(broker_registration::BrokerRegistrationRequest),
        BrokerHeartbeatRequest(broker_heartbeat::BrokerHeartbeatRequest),
        UnregisterBrokerRequest(unregister_broker::UnregisterBrokerRequest),
        DescribeTransactionsRequest(describe_transactions::DescribeTransactionsRequest),
        ListTransactionsRequest(list_transactions::ListTransactionsRequest),
        AllocateProducerIdsRequest(allocate_producer_ids::AllocateProducerIdsRequest),
    }
}
pub mod response {
    pub mod produce {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 0i16, versions = "0-9", flexible = "9+")]
        pub struct ProduceResponse {
            ///Each produce response
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<TopicProduceResponse>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+", default = "0")]
            pub throttle_time_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct BatchIndexAndErrorMessage {
            ///The batch index of the record that cause the batch to be dropped
            #[kafka(versions = "8+")]
            pub batch_index: krost::types::Int32,
            ///The error message of the record that caused the batch to be dropped
            #[kafka(versions = "8+", nullable = "8+", default = "null")]
            pub batch_index_error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionProduceResponse {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The base offset.
            #[kafka(versions = "0+")]
            pub base_offset: krost::types::Int64,
            ///The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
            #[kafka(versions = "2+", default = "-1")]
            pub log_append_time_ms: krost::types::Int64,
            ///The log start offset.
            #[kafka(versions = "5+", default = "-1")]
            pub log_start_offset: krost::types::Int64,
            ///The batch indices of records that caused the batch to be dropped
            #[kafka(versions = "8+")]
            pub record_errors: krost::types::Array<BatchIndexAndErrorMessage>,
            ///The global error message summarizing the common root cause of the records that caused the batch to be dropped
            #[kafka(versions = "8+", nullable = "8+", default = "null")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicProduceResponse {
            ///The topic name
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition that we produced to within the topic.
            #[kafka(versions = "0+")]
            pub partition_responses: krost::types::Array<PartitionProduceResponse>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod fetch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 1i16, versions = "0-13", flexible = "12+")]
        pub struct FetchResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top level response error code.
            #[kafka(versions = "7+")]
            pub error_code: krost::types::Int16,
            ///The fetch session ID, or 0 if this is not part of a fetch session.
            #[kafka(versions = "7+", default = "0")]
            pub session_id: krost::types::Int32,
            ///The response topics.
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<FetchableTopicResponse>,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EpochEndOffset {
            #[kafka(versions = "12+", default = "-1")]
            pub epoch: krost::types::Int32,
            #[kafka(versions = "12+", default = "-1")]
            pub end_offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct LeaderIdAndEpoch {
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "12+", default = "-1")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "12+", default = "-1")]
            pub leader_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct SnapshotId {
            #[kafka(versions = "0+", default = "-1")]
            pub end_offset: krost::types::Int64,
            #[kafka(versions = "0+", default = "-1")]
            pub epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AbortedTransaction {
            ///The producer id associated with the aborted transaction.
            #[kafka(versions = "4+")]
            pub producer_id: krost::types::Int64,
            ///The first offset in the aborted transaction.
            #[kafka(versions = "4+")]
            pub first_offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no fetch error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The current high water mark.
            #[kafka(versions = "0+")]
            pub high_watermark: krost::types::Int64,
            ///The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
            #[kafka(versions = "4+", default = "-1")]
            pub last_stable_offset: krost::types::Int64,
            ///The current log start offset.
            #[kafka(versions = "5+", default = "-1")]
            pub log_start_offset: krost::types::Int64,
            ///In case divergence is detected based on the `LastFetchedEpoch` and `FetchOffset` in the request, this field indicates the largest epoch and its end offset such that subsequent records are known to diverge
            #[kafka(versions = "12+", tagged = "12+", tag = 0i32)]
            pub diverging_epoch: EpochEndOffset,
            #[kafka(versions = "12+", tagged = "12+", tag = 1i32)]
            pub current_leader: LeaderIdAndEpoch,
            ///In the case of fetching an offset less than the LogStartOffset, this is the end offset and epoch that should be used in the FetchSnapshot request.
            #[kafka(versions = "12+", tagged = "12+", tag = 2i32)]
            pub snapshot_id: SnapshotId,
            ///The aborted transactions.
            #[kafka(versions = "4+", nullable = "4+")]
            pub aborted_transactions: krost::types::Array<AbortedTransaction>,
            ///The preferred read replica for the consumer to use on its next fetch request
            #[kafka(versions = "11+", default = "-1")]
            pub preferred_read_replica: krost::types::Int32,
            ///The record data.
            #[kafka(versions = "0+", nullable = "0+")]
            pub records: krost::record::RecordBatch,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct FetchableTopicResponse {
            ///The topic name.
            #[kafka(versions = "0-12")]
            pub topic: krost::types::String,
            ///The unique topic ID
            #[kafka(versions = "13+")]
            pub topic_id: krost::types::Uuid,
            ///The topic partitions.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "12+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_offsets {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 2i16, versions = "0-7", flexible = "6+")]
        pub struct ListOffsetsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "2+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each topic in the response.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<ListOffsetsTopicResponse>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListOffsetsPartitionResponse {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The partition error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The result offsets.
            #[kafka(versions = "0")]
            pub old_style_offsets: krost::types::Array<krost::types::Int64>,
            ///The timestamp associated with the returned offset.
            #[kafka(versions = "1+", default = "-1")]
            pub timestamp: krost::types::Int64,
            ///The returned offset.
            #[kafka(versions = "1+", default = "-1")]
            pub offset: krost::types::Int64,
            #[kafka(versions = "4+", default = "-1")]
            pub leader_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListOffsetsTopicResponse {
            ///The topic name
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition in the response.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<ListOffsetsPartitionResponse>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod metadata {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 3i16, versions = "0-12", flexible = "9+")]
        pub struct MetadataResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "3+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each broker in the response.
            #[kafka(versions = "0+")]
            pub brokers: krost::types::Array<MetadataResponseBroker>,
            ///The cluster ID that responding broker belongs to.
            #[kafka(versions = "2+", nullable = "2+", default = "null")]
            pub cluster_id: krost::types::NullableString,
            ///The ID of the controller broker.
            #[kafka(versions = "1+", default = "-1")]
            pub controller_id: krost::types::Int32,
            ///Each topic in the response.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<MetadataResponseTopic>,
            ///32-bit bitfield to represent authorized operations for this cluster.
            #[kafka(versions = "8-10", default = "-2147483648")]
            pub cluster_authorized_operations: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MetadataResponseBroker {
            ///The broker ID.
            #[kafka(versions = "0+")]
            pub node_id: krost::types::Int32,
            ///The broker hostname.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The broker port.
            #[kafka(versions = "0+")]
            pub port: krost::types::Int32,
            ///The rack of the broker, or null if it has not been assigned to a rack.
            #[kafka(versions = "1+", nullable = "1+", default = "null")]
            pub rack: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MetadataResponsePartition {
            ///The partition error, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The ID of the leader broker.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The leader epoch of this partition.
            #[kafka(versions = "7+", default = "-1")]
            pub leader_epoch: krost::types::Int32,
            ///The set of all nodes that host this partition.
            #[kafka(versions = "0+")]
            pub replica_nodes: krost::types::Array<krost::types::Int32>,
            ///The set of nodes that are in sync with the leader for this partition.
            #[kafka(versions = "0+")]
            pub isr_nodes: krost::types::Array<krost::types::Int32>,
            ///The set of offline replicas of this partition.
            #[kafka(versions = "5+")]
            pub offline_replicas: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MetadataResponseTopic {
            ///The topic error, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The topic name.
            #[kafka(versions = "0+", nullable = "12+")]
            pub name: krost::types::NullableString,
            ///The topic id.
            #[kafka(versions = "10+")]
            pub topic_id: krost::types::Uuid,
            ///True if the topic is internal.
            #[kafka(versions = "1+", default = "false")]
            pub is_internal: krost::types::Bool,
            ///Each partition in the topic.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<MetadataResponsePartition>,
            ///32-bit bitfield to represent authorized operations for this topic.
            #[kafka(versions = "8+", default = "-2147483648")]
            pub topic_authorized_operations: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "9+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod leader_and_isr {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 4i16, versions = "0-6", flexible = "4+")]
        pub struct LeaderAndIsrResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///Each partition in v0 to v4 message.
            #[kafka(versions = "0-4")]
            pub partition_errors: krost::types::Array<LeaderAndIsrPartitionError>,
            ///Each topic
            #[kafka(versions = "5+")]
            pub topics: krost::types::Array<LeaderAndIsrTopicError>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct LeaderAndIsrTopicError {
            ///The unique topic ID
            #[kafka(versions = "5+")]
            pub topic_id: krost::types::Uuid,
            ///Each partition.
            #[kafka(versions = "5+")]
            pub partition_errors: krost::types::Array<LeaderAndIsrPartitionError>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod stop_replica {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 5i16, versions = "0-3", flexible = "2+")]
        pub struct StopReplicaResponse {
            ///The top-level error code, or 0 if there was no top-level error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The responses for each partition.
            #[kafka(versions = "0+")]
            pub partition_errors: krost::types::Array<StopReplicaPartitionError>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct StopReplicaPartitionError {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The partition error code, or 0 if there was no partition error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod update_metadata {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 6i16, versions = "0-7", flexible = "6+")]
        pub struct UpdateMetadataResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod controlled_shutdown {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 7i16, versions = "0-3", flexible = "3+")]
        pub struct ControlledShutdownResponse {
            ///The top-level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The partitions that the broker still leads.
            #[kafka(versions = "0+")]
            pub remaining_partitions: krost::types::Array<RemainingPartition>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct RemainingPartition {
            ///The name of the topic.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            ///The index of the partition.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_commit {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 8i16, versions = "0-8", flexible = "8+")]
        pub struct OffsetCommitResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "3+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses for each topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetCommitResponseTopic>,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetCommitResponsePartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetCommitResponseTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The responses for each partition in the topic.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OffsetCommitResponsePartition>,
            ///The tagged fields.
            #[kafka(versions = "8+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_fetch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 9i16, versions = "0-8", flexible = "6+")]
        pub struct OffsetFetchResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "3+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses per topic.
            #[kafka(versions = "0-7")]
            pub topics: krost::types::Array<OffsetFetchResponseTopic>,
            ///The top-level error code, or 0 if there was no error.
            #[kafka(versions = "2-7", default = "0")]
            pub error_code: krost::types::Int16,
            ///The responses per group id.
            #[kafka(versions = "8+")]
            pub groups: krost::types::Array<OffsetFetchResponseGroup>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchResponsePartition {
            ///The partition index.
            #[kafka(versions = "0-7")]
            pub partition_index: krost::types::Int32,
            ///The committed message offset.
            #[kafka(versions = "0-7")]
            pub committed_offset: krost::types::Int64,
            ///The leader epoch.
            #[kafka(versions = "5-7", default = "-1")]
            pub committed_leader_epoch: krost::types::Int32,
            ///The partition metadata.
            #[kafka(versions = "0-7", nullable = "0-7")]
            pub metadata: krost::types::NullableString,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0-7")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchResponseTopic {
            ///The topic name.
            #[kafka(versions = "0-7")]
            pub name: krost::types::String,
            ///The responses per partition
            #[kafka(versions = "0-7")]
            pub partitions: krost::types::Array<OffsetFetchResponsePartition>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchResponsePartitions {
            ///The partition index.
            #[kafka(versions = "8+")]
            pub partition_index: krost::types::Int32,
            ///The committed message offset.
            #[kafka(versions = "8+")]
            pub committed_offset: krost::types::Int64,
            ///The leader epoch.
            #[kafka(versions = "8+", default = "-1")]
            pub committed_leader_epoch: krost::types::Int32,
            ///The partition metadata.
            #[kafka(versions = "8+", nullable = "8+")]
            pub metadata: krost::types::NullableString,
            ///The partition-level error code, or 0 if there was no error.
            #[kafka(versions = "8+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchResponseTopics {
            ///The topic name.
            #[kafka(versions = "8+")]
            pub name: krost::types::String,
            ///The responses per partition
            #[kafka(versions = "8+")]
            pub partitions: krost::types::Array<OffsetFetchResponsePartitions>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetFetchResponseGroup {
            ///The group ID.
            #[kafka(versions = "8+")]
            pub group_id: krost::types::String,
            ///The responses per topic.
            #[kafka(versions = "8+")]
            pub topics: krost::types::Array<OffsetFetchResponseTopics>,
            ///The group-level error code, or 0 if there was no error.
            #[kafka(versions = "8+", default = "0")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod find_coordinator {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 10i16, versions = "0-4", flexible = "3+")]
        pub struct FindCoordinatorResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0-3")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "1-3", nullable = "1-3")]
            pub error_message: krost::types::NullableString,
            ///The node id.
            #[kafka(versions = "0-3")]
            pub node_id: krost::types::Int32,
            ///The host name.
            #[kafka(versions = "0-3")]
            pub host: krost::types::String,
            ///The port.
            #[kafka(versions = "0-3")]
            pub port: krost::types::Int32,
            ///Each coordinator result in the response
            #[kafka(versions = "4+")]
            pub coordinators: krost::types::Array<Coordinator>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct Coordinator {
            ///The coordinator key.
            #[kafka(versions = "4+")]
            pub key: krost::types::String,
            ///The node id.
            #[kafka(versions = "4+")]
            pub node_id: krost::types::Int32,
            ///The host name.
            #[kafka(versions = "4+")]
            pub host: krost::types::String,
            ///The port.
            #[kafka(versions = "4+")]
            pub port: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "4+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "4+", nullable = "4+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod join_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 11i16, versions = "0-9", flexible = "6+")]
        pub struct JoinGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "2+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The generation ID of the group.
            #[kafka(versions = "0+", default = "-1")]
            pub generation_id: krost::types::Int32,
            ///The group protocol name.
            #[kafka(versions = "7+", nullable = "7+", default = "null")]
            pub protocol_type: krost::types::NullableString,
            ///The group protocol selected by the coordinator.
            #[kafka(versions = "0+", nullable = "7+")]
            pub protocol_name: krost::types::NullableString,
            ///The leader of the group.
            #[kafka(versions = "0+")]
            pub leader: krost::types::String,
            ///True if the leader must skip running the assignment.
            #[kafka(versions = "9+", default = "false")]
            pub skip_assignment: krost::types::Bool,
            ///The member ID assigned by the group coordinator.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            #[kafka(versions = "0+")]
            pub members: krost::types::Array<JoinGroupResponseMember>,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct JoinGroupResponseMember {
            ///The group member ID.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The group member metadata.
            #[kafka(versions = "0+")]
            pub metadata: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "6+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod heartbeat {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 12i16, versions = "0-4", flexible = "4+")]
        pub struct HeartbeatResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod leave_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 13i16, versions = "0-5", flexible = "4+")]
        pub struct LeaveGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///List of leaving member responses.
            #[kafka(versions = "3+")]
            pub members: krost::types::Array<MemberResponse>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct MemberResponse {
            ///The member ID to remove from the group.
            #[kafka(versions = "3+")]
            pub member_id: krost::types::String,
            ///The group instance ID to remove from the group.
            #[kafka(versions = "3+", nullable = "3+")]
            pub group_instance_id: krost::types::NullableString,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "3+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sync_group {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 14i16, versions = "0-5", flexible = "4+")]
        pub struct SyncGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The group protocol type.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub protocol_type: krost::types::NullableString,
            ///The group protocol name.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub protocol_name: krost::types::NullableString,
            ///The member assignment.
            #[kafka(versions = "0+")]
            pub assignment: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 15i16, versions = "0-5", flexible = "5+")]
        pub struct DescribeGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each described group.
            #[kafka(versions = "0+")]
            pub groups: krost::types::Array<DescribedGroup>,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribedGroupMember {
            ///The member ID assigned by the group coordinator.
            #[kafka(versions = "0+")]
            pub member_id: krost::types::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(versions = "4+", nullable = "4+", default = "null")]
            pub group_instance_id: krost::types::NullableString,
            ///The client ID used in the member's latest join group request.
            #[kafka(versions = "0+")]
            pub client_id: krost::types::String,
            ///The client host.
            #[kafka(versions = "0+")]
            pub client_host: krost::types::String,
            ///The metadata corresponding to the current group protocol in use.
            #[kafka(versions = "0+")]
            pub member_metadata: krost::types::Bytes,
            ///The current assignment provided by the group leader.
            #[kafka(versions = "0+")]
            pub member_assignment: krost::types::Bytes,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribedGroup {
            ///The describe error, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The group ID string.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The group state string, or the empty string.
            #[kafka(versions = "0+")]
            pub group_state: krost::types::String,
            ///The group protocol type, or the empty string.
            #[kafka(versions = "0+")]
            pub protocol_type: krost::types::String,
            ///The group protocol data, or the empty string.
            #[kafka(versions = "0+")]
            pub protocol_data: krost::types::String,
            ///The group members.
            #[kafka(versions = "0+")]
            pub members: krost::types::Array<DescribedGroupMember>,
            ///32-bit bitfield to represent authorized operations for this group.
            #[kafka(versions = "3+", default = "-2147483648")]
            pub authorized_operations: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 16i16, versions = "0-4", flexible = "3+")]
        pub struct ListGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///Each group in the response.
            #[kafka(versions = "0+")]
            pub groups: krost::types::Array<ListedGroup>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ListedGroup {
            ///The group ID.
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The group protocol type.
            #[kafka(versions = "0+")]
            pub protocol_type: krost::types::String,
            ///The group state name.
            #[kafka(versions = "4+")]
            pub group_state: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sasl_handshake {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 17i16, versions = "0-1")]
        pub struct SaslHandshakeResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The mechanisms enabled in the server.
            #[kafka(versions = "0+")]
            pub mechanisms: krost::types::Array<krost::types::String>,
        }
    }
    pub mod api_versions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 18i16, versions = "0-3", flexible = "3+")]
        pub struct ApiVersionsResponse {
            ///The top-level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The APIs supported by the broker.
            #[kafka(versions = "0+")]
            pub api_keys: krost::types::Array<ApiVersion>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Features supported by the broker.
            #[kafka(versions = "3+", tagged = "3+", tag = 0i32)]
            pub supported_features: krost::types::Array<SupportedFeatureKey>,
            ///The monotonically increasing epoch for the finalized features information. Valid values are >= 0. A value of -1 is special and represents unknown epoch.
            #[kafka(versions = "3+", tagged = "3+", tag = 1i32, default = "-1")]
            pub finalized_features_epoch: krost::types::Int64,
            ///List of cluster-wide finalized features. The information is valid only if FinalizedFeaturesEpoch >= 0.
            #[kafka(versions = "3+", tagged = "3+", tag = 2i32)]
            pub finalized_features: krost::types::Array<FinalizedFeatureKey>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ApiVersion {
            ///The API index.
            #[kafka(versions = "0+")]
            pub api_key: krost::types::Int16,
            ///The minimum supported version, inclusive.
            #[kafka(versions = "0+")]
            pub min_version: krost::types::Int16,
            ///The maximum supported version, inclusive.
            #[kafka(versions = "0+")]
            pub max_version: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct SupportedFeatureKey {
            ///The name of the feature.
            #[kafka(versions = "3+")]
            pub name: krost::types::String,
            ///The minimum supported version for the feature.
            #[kafka(versions = "3+")]
            pub min_version: krost::types::Int16,
            ///The maximum supported version for the feature.
            #[kafka(versions = "3+")]
            pub max_version: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct FinalizedFeatureKey {
            ///The name of the feature.
            #[kafka(versions = "3+")]
            pub name: krost::types::String,
            ///The cluster-wide finalized max version level for the feature.
            #[kafka(versions = "3+")]
            pub max_version_level: krost::types::Int16,
            ///The cluster-wide finalized min version level for the feature.
            #[kafka(versions = "3+")]
            pub min_version_level: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_topics {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 19i16, versions = "0-7", flexible = "5+")]
        pub struct CreateTopicsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "2+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Results for each topic we tried to create.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<CreatableTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatableTopicConfigs {
            ///The configuration name.
            #[kafka(versions = "5+")]
            pub name: krost::types::String,
            ///The configuration value.
            #[kafka(versions = "5+", nullable = "5+")]
            pub value: krost::types::NullableString,
            ///True if the configuration is read-only.
            #[kafka(versions = "5+")]
            pub read_only: krost::types::Bool,
            ///The configuration source.
            #[kafka(versions = "5+", default = "-1")]
            pub config_source: krost::types::Int8,
            ///True if this configuration is sensitive.
            #[kafka(versions = "5+")]
            pub is_sensitive: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatableTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The unique topic ID
            #[kafka(versions = "7+")]
            pub topic_id: krost::types::Uuid,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "1+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///Optional topic config error returned if configs are not returned in the response.
            #[kafka(versions = "5+", tagged = "5+", tag = 0i32)]
            pub topic_config_error_code: krost::types::Int16,
            ///Number of partitions of the topic.
            #[kafka(versions = "5+", default = "-1")]
            pub num_partitions: krost::types::Int32,
            ///Replication factor of the topic.
            #[kafka(versions = "5+", default = "-1")]
            pub replication_factor: krost::types::Int16,
            ///Configuration of the topic.
            #[kafka(versions = "5+", nullable = "5+")]
            pub configs: krost::types::Array<CreatableTopicConfigs>,
            ///The tagged fields.
            #[kafka(versions = "5+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_topics {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 20i16, versions = "0-6", flexible = "4+")]
        pub struct DeleteTopicsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "1+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each topic we tried to delete.
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<DeletableTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeletableTopicResult {
            ///The topic name
            #[kafka(versions = "0+", nullable = "6+")]
            pub name: krost::types::NullableString,
            ///the unique topic ID
            #[kafka(versions = "6+")]
            pub topic_id: krost::types::Uuid,
            ///The deletion error, or 0 if the deletion succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "5+", nullable = "5+", default = "null")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_records {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 21i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteRecordsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each topic that we wanted to delete records from.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<DeleteRecordsTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteRecordsPartitionResult {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The partition low water mark.
            #[kafka(versions = "0+")]
            pub low_watermark: krost::types::Int64,
            ///The deletion error code, or 0 if the deletion succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteRecordsTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition that we wanted to delete records from.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<DeleteRecordsPartitionResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod init_producer_id {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 22i16, versions = "0-4", flexible = "2+")]
        pub struct InitProducerIdResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The current producer id.
            #[kafka(versions = "0+", default = -1f64)]
            pub producer_id: krost::types::Int64,
            ///The current epoch associated with the producer id.
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_for_leader_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 23i16, versions = "0-4", flexible = "4+")]
        pub struct OffsetForLeaderEpochResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "2+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each topic we fetched offsets for.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetForLeaderTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EpochEndOffset {
            ///The error code 0, or if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition: krost::types::Int32,
            ///The leader epoch of the partition.
            #[kafka(versions = "1+", default = "-1")]
            pub leader_epoch: krost::types::Int32,
            ///The end offset of the epoch.
            #[kafka(versions = "0+", default = "-1")]
            pub end_offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetForLeaderTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            ///Each partition in the topic we fetched offsets for.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<EpochEndOffset>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod add_partitions_to_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 24i16, versions = "0-3", flexible = "3+")]
        pub struct AddPartitionsToTxnResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each topic.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<AddPartitionsToTxnTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AddPartitionsToTxnPartitionResult {
            ///The partition indexes.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The response error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AddPartitionsToTxnTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The results for each partition
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<AddPartitionsToTxnPartitionResult>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod add_offsets_to_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 25i16, versions = "0-3", flexible = "3+")]
        pub struct AddOffsetsToTxnResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The response error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod end_txn {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 26i16, versions = "0-3", flexible = "3+")]
        pub struct EndTxnResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod write_txn_markers {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 27i16, versions = "0-1", flexible = "1+")]
        pub struct WriteTxnMarkersResponse {
            ///The results for writing makers.
            #[kafka(versions = "0+")]
            pub markers: krost::types::Array<WritableTxnMarkerResult>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct WritableTxnMarkerPartitionResult {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct WritableTxnMarkerTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The results by partition.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<WritableTxnMarkerPartitionResult>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct WritableTxnMarkerResult {
            ///The current producer ID in use by the transactional ID.
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///The results by topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<WritableTxnMarkerTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod txn_offset_commit {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 28i16, versions = "0-3", flexible = "3+")]
        pub struct TxnOffsetCommitResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses for each topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TxnOffsetCommitResponseTopic>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TxnOffsetCommitResponsePartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TxnOffsetCommitResponseTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The responses for each partition in the topic.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<TxnOffsetCommitResponsePartition>,
            ///The tagged fields.
            #[kafka(versions = "3+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 29i16, versions = "0-2", flexible = "2+")]
        pub struct DescribeAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///Each Resource that is referenced in an ACL.
            #[kafka(versions = "0+")]
            pub resources: krost::types::Array<DescribeAclsResource>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AclDescription {
            ///The ACL principal.
            #[kafka(versions = "0+")]
            pub principal: krost::types::String,
            ///The ACL host.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The ACL operation.
            #[kafka(versions = "0+")]
            pub operation: krost::types::Int8,
            ///The ACL permission type.
            #[kafka(versions = "0+")]
            pub permission_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeAclsResource {
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The resource pattern type.
            #[kafka(versions = "1+", default = "3")]
            pub pattern_type: krost::types::Int8,
            ///The ACLs.
            #[kafka(versions = "0+")]
            pub acls: krost::types::Array<AclDescription>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 30i16, versions = "0-2", flexible = "2+")]
        pub struct CreateAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each ACL creation.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<AclCreationResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AclCreationResult {
            ///The result error, or zero if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The result message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_acls {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 31i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each filter.
            #[kafka(versions = "0+")]
            pub filter_results: krost::types::Array<DeleteAclsFilterResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteAclsMatchingAcl {
            ///The deletion error code, or 0 if the deletion succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The deletion error message, or null if the deletion succeeded.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The ACL resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The ACL resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The ACL resource pattern type.
            #[kafka(versions = "1+", default = "3")]
            pub pattern_type: krost::types::Int8,
            ///The ACL principal.
            #[kafka(versions = "0+")]
            pub principal: krost::types::String,
            ///The ACL host.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The ACL operation.
            #[kafka(versions = "0+")]
            pub operation: krost::types::Int8,
            ///The ACL permission type.
            #[kafka(versions = "0+")]
            pub permission_type: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeleteAclsFilterResult {
            ///The error code, or 0 if the filter succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if the filter succeeded.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The ACLs which matched this filter.
            #[kafka(versions = "0+")]
            pub matching_acls: krost::types::Array<DeleteAclsMatchingAcl>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 32i16, versions = "0-4", flexible = "4+")]
        pub struct DescribeConfigsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each resource.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<DescribeConfigsResult>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeConfigsSynonym {
            ///The synonym name.
            #[kafka(versions = "1+")]
            pub name: krost::types::String,
            ///The synonym value.
            #[kafka(versions = "1+", nullable = "0+")]
            pub value: krost::types::NullableString,
            ///The synonym source.
            #[kafka(versions = "1+")]
            pub source: krost::types::Int8,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeConfigsResourceResult {
            ///The configuration name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The configuration value.
            #[kafka(versions = "0+", nullable = "0+")]
            pub value: krost::types::NullableString,
            ///True if the configuration is read-only.
            #[kafka(versions = "0+")]
            pub read_only: krost::types::Bool,
            ///True if the configuration is not set.
            #[kafka(versions = "0")]
            pub is_default: krost::types::Bool,
            ///The configuration source.
            #[kafka(versions = "1+", default = "-1")]
            pub config_source: krost::types::Int8,
            ///True if this configuration is sensitive.
            #[kafka(versions = "0+")]
            pub is_sensitive: krost::types::Bool,
            ///The synonyms for this configuration key.
            #[kafka(versions = "1+")]
            pub synonyms: krost::types::Array<DescribeConfigsSynonym>,
            ///The configuration data type. Type can be one of the following values - BOOLEAN, STRING, INT, SHORT, LONG, DOUBLE, LIST, CLASS, PASSWORD
            #[kafka(versions = "3+", default = "0")]
            pub config_type: krost::types::Int8,
            ///The configuration documentation.
            #[kafka(versions = "3+", nullable = "0+")]
            pub documentation: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeConfigsResult {
            ///The error code, or 0 if we were able to successfully describe the configurations.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if we were able to successfully describe the configurations.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///Each listed configuration.
            #[kafka(versions = "0+")]
            pub configs: krost::types::Array<DescribeConfigsResourceResult>,
            ///The tagged fields.
            #[kafka(versions = "4+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 33i16, versions = "0-2", flexible = "2+")]
        pub struct AlterConfigsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses for each resource.
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<AlterConfigsResourceResponse>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterConfigsResourceResponse {
            ///The resource error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The resource error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_replica_log_dirs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 34i16, versions = "0-2", flexible = "2+")]
        pub struct AlterReplicaLogDirsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for each topic.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<AlterReplicaLogDirTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterReplicaLogDirPartitionResult {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterReplicaLogDirTopicResult {
            ///The name of the topic.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            ///The results for each partition.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<AlterReplicaLogDirPartitionResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_log_dirs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 35i16, versions = "0-3", flexible = "2+")]
        pub struct DescribeLogDirsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "3+")]
            pub error_code: krost::types::Int16,
            ///The log directories.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<DescribeLogDirsResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeLogDirsPartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The size of the log segments in this partition in bytes.
            #[kafka(versions = "0+")]
            pub partition_size: krost::types::Int64,
            ///The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)
            #[kafka(versions = "0+")]
            pub offset_lag: krost::types::Int64,
            ///True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
            #[kafka(versions = "0+")]
            pub is_future_key: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeLogDirsTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<DescribeLogDirsPartition>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeLogDirsResult {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The absolute log directory path.
            #[kafka(versions = "0+")]
            pub log_dir: krost::types::String,
            ///Each topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<DescribeLogDirsTopic>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod sasl_authenticate {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 36i16, versions = "0-2", flexible = "2+")]
        pub struct SaslAuthenticateResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The SASL authentication bytes from the server, as defined by the SASL mechanism.
            #[kafka(versions = "0+")]
            pub auth_bytes: krost::types::Bytes,
            ///The SASL authentication bytes from the server, as defined by the SASL mechanism.
            #[kafka(versions = "1+", default = "0")]
            pub session_lifetime_ms: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_partitions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 37i16, versions = "0-3", flexible = "2+")]
        pub struct CreatePartitionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The partition creation results for each topic.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<CreatePartitionsTopicResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CreatePartitionsTopicResult {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The result error, or zero if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The result message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod create_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 38i16, versions = "0-2", flexible = "2+")]
        pub struct CreateDelegationTokenResponse {
            ///The top-level error, or zero if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The principal type of the token owner.
            #[kafka(versions = "0+")]
            pub principal_type: krost::types::String,
            ///The name of the token owner.
            #[kafka(versions = "0+")]
            pub principal_name: krost::types::String,
            ///When this token was generated.
            #[kafka(versions = "0+")]
            pub issue_timestamp_ms: krost::types::Int64,
            ///When this token expires.
            #[kafka(versions = "0+")]
            pub expiry_timestamp_ms: krost::types::Int64,
            ///The maximum lifetime of this token.
            #[kafka(versions = "0+")]
            pub max_timestamp_ms: krost::types::Int64,
            ///The token UUID.
            #[kafka(versions = "0+")]
            pub token_id: krost::types::String,
            ///HMAC of the delegation token.
            #[kafka(versions = "0+")]
            pub hmac: krost::types::Bytes,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod renew_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 39i16, versions = "0-2", flexible = "2+")]
        pub struct RenewDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The timestamp in milliseconds at which this token expires.
            #[kafka(versions = "0+")]
            pub expiry_timestamp_ms: krost::types::Int64,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod expire_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 40i16, versions = "0-2", flexible = "2+")]
        pub struct ExpireDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The timestamp in milliseconds at which this token expires.
            #[kafka(versions = "0+")]
            pub expiry_timestamp_ms: krost::types::Int64,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_delegation_token {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 41i16, versions = "0-2", flexible = "2+")]
        pub struct DescribeDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tokens.
            #[kafka(versions = "0+")]
            pub tokens: krost::types::Array<DescribedDelegationToken>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribedDelegationTokenRenewer {
            ///The renewer principal type
            #[kafka(versions = "0+")]
            pub principal_type: krost::types::String,
            ///The renewer principal name
            #[kafka(versions = "0+")]
            pub principal_name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribedDelegationToken {
            ///The token principal type.
            #[kafka(versions = "0+")]
            pub principal_type: krost::types::String,
            ///The token principal name.
            #[kafka(versions = "0+")]
            pub principal_name: krost::types::String,
            ///The token issue timestamp in milliseconds.
            #[kafka(versions = "0+")]
            pub issue_timestamp: krost::types::Int64,
            ///The token expiry timestamp in milliseconds.
            #[kafka(versions = "0+")]
            pub expiry_timestamp: krost::types::Int64,
            ///The token maximum timestamp length in milliseconds.
            #[kafka(versions = "0+")]
            pub max_timestamp: krost::types::Int64,
            ///The token ID.
            #[kafka(versions = "0+")]
            pub token_id: krost::types::String,
            ///The token HMAC.
            #[kafka(versions = "0+")]
            pub hmac: krost::types::Bytes,
            ///Those who are able to renew this token before it expires.
            #[kafka(versions = "0+")]
            pub renewers: krost::types::Array<DescribedDelegationTokenRenewer>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod delete_groups {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 42i16, versions = "0-2", flexible = "2+")]
        pub struct DeleteGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The deletion results
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<DeletableGroupResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DeletableGroupResult {
            ///The group id
            #[kafka(versions = "0+")]
            pub group_id: krost::types::String,
            ///The deletion error, or 0 if the deletion succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod elect_leaders {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 43i16, versions = "0-2", flexible = "2+")]
        pub struct ElectLeadersResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top level response error code.
            #[kafka(versions = "1+")]
            pub error_code: krost::types::Int16,
            ///The election results, or an empty array if the requester did not have permission and the request asks for all partitions.
            #[kafka(versions = "0+")]
            pub replica_election_results: krost::types::Array<ReplicaElectionResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionResult {
            ///The partition id
            #[kafka(versions = "0+")]
            pub partition_id: krost::types::Int32,
            ///The result error, or zero if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The result message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ReplicaElectionResult {
            ///The topic name
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            ///The results for each partition
            #[kafka(versions = "0+")]
            pub partition_result: krost::types::Array<PartitionResult>,
            ///The tagged fields.
            #[kafka(versions = "2+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod incremental_alter_configs {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 44i16, versions = "0-1", flexible = "1+")]
        pub struct IncrementalAlterConfigsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses for each resource.
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<AlterConfigsResourceResponse>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterConfigsResourceResponse {
            ///The resource error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The resource error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The resource type.
            #[kafka(versions = "0+")]
            pub resource_type: krost::types::Int8,
            ///The resource name.
            #[kafka(versions = "0+")]
            pub resource_name: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_partition_reassignments {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 45i16, versions = "0", flexible = "0+")]
        pub struct AlterPartitionReassignmentsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top-level error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The responses to topics to reassign.
            #[kafka(versions = "0+")]
            pub responses: krost::types::Array<ReassignableTopicResponse>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ReassignablePartitionResponse {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code for this partition, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message for this partition, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ReassignableTopicResponse {
            ///The topic name
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The responses to partitions to reassign
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<ReassignablePartitionResponse>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_partition_reassignments {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 46i16, versions = "0", flexible = "0+")]
        pub struct ListPartitionReassignmentsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top-level error code, or 0 if there was no error
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The ongoing reassignments for each topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OngoingTopicReassignment>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OngoingPartitionReassignment {
            ///The index of the partition.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The current replica set.
            #[kafka(versions = "0+")]
            pub replicas: krost::types::Array<krost::types::Int32>,
            ///The set of replicas we are currently adding.
            #[kafka(versions = "0+")]
            pub adding_replicas: krost::types::Array<krost::types::Int32>,
            ///The set of replicas we are currently removing.
            #[kafka(versions = "0+")]
            pub removing_replicas: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OngoingTopicReassignment {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The ongoing reassignments for each partition.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OngoingPartitionReassignment>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod offset_delete {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 47i16, versions = "0")]
        pub struct OffsetDeleteResponse {
            ///The top-level error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The responses for each topic.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<OffsetDeleteResponseTopic>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetDeleteResponsePartition {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct OffsetDeleteResponseTopic {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The responses for each partition in the topic.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<OffsetDeleteResponsePartition>,
        }
    }
    pub mod describe_client_quotas {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 48i16, versions = "0-1", flexible = "1+")]
        pub struct DescribeClientQuotasResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or `0` if the quota description succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or `null` if the quota description succeeded.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///A result entry.
            #[kafka(versions = "0+", nullable = "0+")]
            pub entries: krost::types::Array<EntryData>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(versions = "0+")]
            pub entity_type: krost::types::String,
            ///The entity name, or null if the default.
            #[kafka(versions = "0+", nullable = "0+")]
            pub entity_name: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ValueData {
            ///The quota configuration key.
            #[kafka(versions = "0+")]
            pub key: krost::types::String,
            ///The quota configuration value.
            #[kafka(versions = "0+")]
            pub value: float64,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntryData {
            ///The quota entity description.
            #[kafka(versions = "0+")]
            pub entity: krost::types::Array<EntityData>,
            ///The quota values for the entity.
            #[kafka(versions = "0+")]
            pub values: krost::types::Array<ValueData>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_client_quotas {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 49i16, versions = "0-1", flexible = "1+")]
        pub struct AlterClientQuotasResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The quota configuration entries to alter.
            #[kafka(versions = "0+")]
            pub entries: krost::types::Array<EntryData>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(versions = "0+")]
            pub entity_type: krost::types::String,
            ///The name of the entity, or null if the default.
            #[kafka(versions = "0+", nullable = "0+")]
            pub entity_name: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct EntryData {
            ///The error code, or `0` if the quota alteration succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, or `null` if the quota alteration succeeded.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The quota entity to alter.
            #[kafka(versions = "0+")]
            pub entity: krost::types::Array<EntityData>,
            ///The tagged fields.
            #[kafka(versions = "1+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_user_scram_credentials {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 50i16, versions = "0", flexible = "0+")]
        pub struct DescribeUserScramCredentialsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The message-level error code, 0 except for user authorization or infrastructure issues.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The message-level error message, if any.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The results for descriptions, one per user.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<DescribeUserScramCredentialsResult>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct CredentialInfo {
            ///The SCRAM mechanism.
            #[kafka(versions = "0+")]
            pub mechanism: krost::types::Int8,
            ///The number of iterations used in the SCRAM credential.
            #[kafka(versions = "0+")]
            pub iterations: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeUserScramCredentialsResult {
            ///The user name.
            #[kafka(versions = "0+")]
            pub user: krost::types::String,
            ///The user-level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The user-level error message, if any.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The mechanism and related information associated with the user's SCRAM credentials.
            #[kafka(versions = "0+")]
            pub credential_infos: krost::types::Array<CredentialInfo>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_user_scram_credentials {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 51i16, versions = "0", flexible = "0+")]
        pub struct AlterUserScramCredentialsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The results for deletions and alterations, one per affected user.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<AlterUserScramCredentialsResult>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct AlterUserScramCredentialsResult {
            ///The user name.
            #[kafka(versions = "0+")]
            pub user: krost::types::String,
            ///The error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The error message, if any.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod vote {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 52i16, versions = "0", flexible = "0+")]
        pub struct VoteResponse {
            ///The top level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///True if the vote was granted and false otherwise
            #[kafka(versions = "0+")]
            pub vote_granted: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod begin_quorum_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 53i16, versions = "0")]
        pub struct BeginQuorumEpochResponse {
            ///The top level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
        }
    }
    pub mod end_quorum_epoch {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 54i16, versions = "0")]
        pub struct EndQuorumEpochResponse {
            ///The top level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
        }
    }
    pub mod describe_quorum {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 55i16, versions = "0", flexible = "0+")]
        pub struct DescribeQuorumResponse {
            ///The top level error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub high_watermark: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub current_voters: krost::types::Array<ReplicaState>,
            #[kafka(versions = "0+")]
            pub observers: krost::types::Array<ReplicaState>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(versions = "0+")]
            pub topic_name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod alter_partition {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 56i16, versions = "0-1", flexible = "0+")]
        pub struct AlterPartitionResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top level response error code
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionData {
            ///The partition index
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The partition level error code
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The broker ID of the leader.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The leader epoch.
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///The in-sync replica IDs.
            #[kafka(versions = "0+")]
            pub isr: krost::types::Array<krost::types::Int32>,
            ///1 if the partition is recovering from an unclean leader election; 0 otherwise.
            #[kafka(versions = "1+", default = "0")]
            pub leader_recovery_state: krost::types::Int8,
            ///The current epoch for the partition for KRaft controllers. The current ZK version for the legacy controllers.
            #[kafka(versions = "0+")]
            pub partition_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            ///The name of the topic
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod update_features {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 57i16, versions = "0-1", flexible = "0+")]
        pub struct UpdateFeaturesResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top-level error code, or `0` if there was no top-level error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The top-level error message, or `null` if there was no top-level error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///Results for each feature update.
            #[kafka(versions = "0+")]
            pub results: krost::types::Array<UpdatableFeatureResult>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct UpdatableFeatureResult {
            ///The name of the finalized feature.
            #[kafka(versions = "0+")]
            pub feature: krost::types::String,
            ///The feature update error code or `0` if the feature update succeeded.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The feature update error, or `null` if the feature update succeeded.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod envelope {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 58i16, versions = "0", flexible = "0+")]
        pub struct EnvelopeResponse {
            ///The embedded response header and data.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub response_data: krost::types::NullableBytes,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod fetch_snapshot {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 59i16, versions = "0", flexible = "0+")]
        pub struct FetchSnapshotResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top level response error code.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The topics to fetch.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicSnapshot>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct SnapshotId {
            #[kafka(versions = "0+")]
            pub end_offset: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct LeaderIdAndEpoch {
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(versions = "0+")]
            pub leader_id: krost::types::Int32,
            ///The latest known leader epoch
            #[kafka(versions = "0+")]
            pub leader_epoch: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionSnapshot {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub index: krost::types::Int32,
            ///The error code, or 0 if there was no fetch error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The snapshot endOffset and epoch fetched
            #[kafka(versions = "0+")]
            pub snapshot_id: SnapshotId,
            #[kafka(versions = "0+", tagged = "0+", tag = 0i32)]
            pub current_leader: LeaderIdAndEpoch,
            ///The total size of the snapshot.
            #[kafka(versions = "0+")]
            pub size: krost::types::Int64,
            ///The starting byte position within the snapshot included in the Bytes field.
            #[kafka(versions = "0+")]
            pub position: krost::types::Int64,
            ///Snapshot data in records format which may not be aligned on an offset boundary
            #[kafka(versions = "0+")]
            pub unaligned_records: krost::record::RecordBatch,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicSnapshot {
            ///The name of the topic to fetch.
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///The partitions to fetch.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionSnapshot>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_cluster {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 60i16, versions = "0", flexible = "0+")]
        pub struct DescribeClusterResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top-level error code, or 0 if there was no error
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub error_message: krost::types::NullableString,
            ///The cluster ID that responding broker belongs to.
            #[kafka(versions = "0+")]
            pub cluster_id: krost::types::String,
            ///The ID of the controller broker.
            #[kafka(versions = "0+", default = "-1")]
            pub controller_id: krost::types::Int32,
            ///Each broker in the response.
            #[kafka(versions = "0+")]
            pub brokers: krost::types::Array<DescribeClusterBroker>,
            ///32-bit bitfield to represent authorized operations for this cluster.
            #[kafka(versions = "0+", default = "-2147483648")]
            pub cluster_authorized_operations: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct DescribeClusterBroker {
            ///The broker ID.
            #[kafka(versions = "0+")]
            pub broker_id: krost::types::Int32,
            ///The broker hostname.
            #[kafka(versions = "0+")]
            pub host: krost::types::String,
            ///The broker port.
            #[kafka(versions = "0+")]
            pub port: krost::types::Int32,
            ///The rack of the broker, or null if it has not been assigned to a rack.
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub rack: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_producers {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 61i16, versions = "0", flexible = "0+")]
        pub struct DescribeProducersResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///Each topic in the response.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicResponse>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct ProducerState {
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int32,
            #[kafka(versions = "0+", default = "-1")]
            pub last_sequence: krost::types::Int32,
            #[kafka(versions = "0+", default = "-1")]
            pub last_timestamp: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub coordinator_epoch: krost::types::Int32,
            #[kafka(versions = "0+", default = "-1")]
            pub current_txn_start_offset: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct PartitionResponse {
            ///The partition index.
            #[kafka(versions = "0+")]
            pub partition_index: krost::types::Int32,
            ///The partition error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The partition error message, which may be null if no additional details are available
            #[kafka(versions = "0+", nullable = "0+", default = "null")]
            pub error_message: krost::types::NullableString,
            #[kafka(versions = "0+")]
            pub active_producers: krost::types::Array<ProducerState>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicResponse {
            ///The topic name
            #[kafka(versions = "0+")]
            pub name: krost::types::String,
            ///Each partition in the response.
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<PartitionResponse>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod broker_registration {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 62i16, versions = "0", flexible = "0+")]
        pub struct BrokerRegistrationResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The broker's assigned epoch, or -1 if none was assigned.
            #[kafka(versions = "0+", default = "-1")]
            pub broker_epoch: krost::types::Int64,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod broker_heartbeat {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 63i16, versions = "0", flexible = "0+")]
        pub struct BrokerHeartbeatResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///True if the broker has approximately caught up with the latest metadata.
            #[kafka(versions = "0+", default = "false")]
            pub is_caught_up: krost::types::Bool,
            ///True if the broker is fenced.
            #[kafka(versions = "0+", default = "true")]
            pub is_fenced: krost::types::Bool,
            ///True if the broker should proceed with its shutdown.
            #[kafka(versions = "0+")]
            pub should_shut_down: krost::types::Bool,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod unregister_broker {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 64i16, versions = "0", flexible = "0+")]
        pub struct UnregisterBrokerResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The top-level error message, or `null` if there was no top-level error.
            #[kafka(versions = "0+", nullable = "0+")]
            pub error_message: krost::types::NullableString,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod describe_transactions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 65i16, versions = "0", flexible = "0+")]
        pub struct DescribeTransactionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub transaction_states: krost::types::Array<TransactionState>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TopicData {
            #[kafka(versions = "0+")]
            pub topic: krost::types::String,
            #[kafka(versions = "0+")]
            pub partitions: krost::types::Array<krost::types::Int32>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TransactionState {
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            #[kafka(versions = "0+")]
            pub transaction_state: krost::types::String,
            #[kafka(versions = "0+")]
            pub transaction_timeout_ms: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub transaction_start_time_ms: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            #[kafka(versions = "0+")]
            pub producer_epoch: krost::types::Int16,
            ///The set of partitions included in the current transaction (if active). When a transaction is preparing to commit or abort, this will include only partitions which do not have markers.
            #[kafka(versions = "0+")]
            pub topics: krost::types::Array<TopicData>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod list_transactions {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 66i16, versions = "0", flexible = "0+")]
        pub struct ListTransactionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///Set of state filters provided in the request which were unknown to the transaction coordinator
            #[kafka(versions = "0+")]
            pub unknown_state_filters: krost::types::Array<krost::types::String>,
            #[kafka(versions = "0+")]
            pub transaction_states: krost::types::Array<TransactionState>,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        pub struct TransactionState {
            #[kafka(versions = "0+")]
            pub transactional_id: krost::types::String,
            #[kafka(versions = "0+")]
            pub producer_id: krost::types::Int64,
            ///The current transaction state of the producer
            #[kafka(versions = "0+")]
            pub transaction_state: krost::types::String,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    pub mod allocate_producer_ids {
        #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
        #[kafka(apikey = 67i16, versions = "0", flexible = "0+")]
        pub struct AllocateProducerIdsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(versions = "0+")]
            pub throttle_time_ms: krost::types::Int32,
            ///The top level response error code
            #[kafka(versions = "0+")]
            pub error_code: krost::types::Int16,
            ///The first producer ID in this range, inclusive
            #[kafka(versions = "0+")]
            pub producer_id_start: krost::types::Int64,
            ///The number of producer IDs in this range
            #[kafka(versions = "0+")]
            pub producer_id_len: krost::types::Int32,
            ///The tagged fields.
            #[kafka(versions = "0+")]
            pub _tagged_fields: krost::types::TaggedFields,
        }
    }
    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, FromVariants)]
    pub enum ResponseBody {
        ProduceResponse(produce::ProduceResponse),
        FetchResponse(fetch::FetchResponse),
        ListOffsetsResponse(list_offsets::ListOffsetsResponse),
        MetadataResponse(metadata::MetadataResponse),
        LeaderAndIsrResponse(leader_and_isr::LeaderAndIsrResponse),
        StopReplicaResponse(stop_replica::StopReplicaResponse),
        UpdateMetadataResponse(update_metadata::UpdateMetadataResponse),
        ControlledShutdownResponse(controlled_shutdown::ControlledShutdownResponse),
        OffsetCommitResponse(offset_commit::OffsetCommitResponse),
        OffsetFetchResponse(offset_fetch::OffsetFetchResponse),
        FindCoordinatorResponse(find_coordinator::FindCoordinatorResponse),
        JoinGroupResponse(join_group::JoinGroupResponse),
        HeartbeatResponse(heartbeat::HeartbeatResponse),
        LeaveGroupResponse(leave_group::LeaveGroupResponse),
        SyncGroupResponse(sync_group::SyncGroupResponse),
        DescribeGroupsResponse(describe_groups::DescribeGroupsResponse),
        ListGroupsResponse(list_groups::ListGroupsResponse),
        SaslHandshakeResponse(sasl_handshake::SaslHandshakeResponse),
        ApiVersionsResponse(api_versions::ApiVersionsResponse),
        CreateTopicsResponse(create_topics::CreateTopicsResponse),
        DeleteTopicsResponse(delete_topics::DeleteTopicsResponse),
        DeleteRecordsResponse(delete_records::DeleteRecordsResponse),
        InitProducerIdResponse(init_producer_id::InitProducerIdResponse),
        OffsetForLeaderEpochResponse(
            offset_for_leader_epoch::OffsetForLeaderEpochResponse,
        ),
        AddPartitionsToTxnResponse(add_partitions_to_txn::AddPartitionsToTxnResponse),
        AddOffsetsToTxnResponse(add_offsets_to_txn::AddOffsetsToTxnResponse),
        EndTxnResponse(end_txn::EndTxnResponse),
        WriteTxnMarkersResponse(write_txn_markers::WriteTxnMarkersResponse),
        TxnOffsetCommitResponse(txn_offset_commit::TxnOffsetCommitResponse),
        DescribeAclsResponse(describe_acls::DescribeAclsResponse),
        CreateAclsResponse(create_acls::CreateAclsResponse),
        DeleteAclsResponse(delete_acls::DeleteAclsResponse),
        DescribeConfigsResponse(describe_configs::DescribeConfigsResponse),
        AlterConfigsResponse(alter_configs::AlterConfigsResponse),
        AlterReplicaLogDirsResponse(alter_replica_log_dirs::AlterReplicaLogDirsResponse),
        DescribeLogDirsResponse(describe_log_dirs::DescribeLogDirsResponse),
        SaslAuthenticateResponse(sasl_authenticate::SaslAuthenticateResponse),
        CreatePartitionsResponse(create_partitions::CreatePartitionsResponse),
        CreateDelegationTokenResponse(
            create_delegation_token::CreateDelegationTokenResponse,
        ),
        RenewDelegationTokenResponse(
            renew_delegation_token::RenewDelegationTokenResponse,
        ),
        ExpireDelegationTokenResponse(
            expire_delegation_token::ExpireDelegationTokenResponse,
        ),
        DescribeDelegationTokenResponse(
            describe_delegation_token::DescribeDelegationTokenResponse,
        ),
        DeleteGroupsResponse(delete_groups::DeleteGroupsResponse),
        ElectLeadersResponse(elect_leaders::ElectLeadersResponse),
        IncrementalAlterConfigsResponse(
            incremental_alter_configs::IncrementalAlterConfigsResponse,
        ),
        AlterPartitionReassignmentsResponse(
            alter_partition_reassignments::AlterPartitionReassignmentsResponse,
        ),
        ListPartitionReassignmentsResponse(
            list_partition_reassignments::ListPartitionReassignmentsResponse,
        ),
        OffsetDeleteResponse(offset_delete::OffsetDeleteResponse),
        DescribeClientQuotasResponse(
            describe_client_quotas::DescribeClientQuotasResponse,
        ),
        AlterClientQuotasResponse(alter_client_quotas::AlterClientQuotasResponse),
        DescribeUserScramCredentialsResponse(
            describe_user_scram_credentials::DescribeUserScramCredentialsResponse,
        ),
        AlterUserScramCredentialsResponse(
            alter_user_scram_credentials::AlterUserScramCredentialsResponse,
        ),
        VoteResponse(vote::VoteResponse),
        BeginQuorumEpochResponse(begin_quorum_epoch::BeginQuorumEpochResponse),
        EndQuorumEpochResponse(end_quorum_epoch::EndQuorumEpochResponse),
        DescribeQuorumResponse(describe_quorum::DescribeQuorumResponse),
        AlterPartitionResponse(alter_partition::AlterPartitionResponse),
        UpdateFeaturesResponse(update_features::UpdateFeaturesResponse),
        EnvelopeResponse(envelope::EnvelopeResponse),
        FetchSnapshotResponse(fetch_snapshot::FetchSnapshotResponse),
        DescribeClusterResponse(describe_cluster::DescribeClusterResponse),
        DescribeProducersResponse(describe_producers::DescribeProducersResponse),
        BrokerRegistrationResponse(broker_registration::BrokerRegistrationResponse),
        BrokerHeartbeatResponse(broker_heartbeat::BrokerHeartbeatResponse),
        UnregisterBrokerResponse(unregister_broker::UnregisterBrokerResponse),
        DescribeTransactionsResponse(
            describe_transactions::DescribeTransactionsResponse,
        ),
        ListTransactionsResponse(list_transactions::ListTransactionsResponse),
        AllocateProducerIdsResponse(allocate_producer_ids::AllocateProducerIdsResponse),
    }
}
pub mod header {
    #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
    #[kafka(versions = "0-1", flexible = "1+")]
    pub struct ResponseHeader {
        ///The correlation ID of this response.
        #[kafka(versions = "0+")]
        pub correlation_id: krost::types::Int32,
        ///The tagged fields.
        #[kafka(versions = "1+")]
        pub _tagged_fields: krost::types::TaggedFields,
    }
    #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
    #[kafka(versions = "0-2", flexible = "2+")]
    pub struct RequestHeader {
        ///The API key of this request.
        #[kafka(versions = "0+")]
        pub request_api_key: krost::types::Int16,
        ///The API version of this request.
        #[kafka(versions = "0+")]
        pub request_api_version: krost::types::Int16,
        ///The correlation ID of this request.
        #[kafka(versions = "0+")]
        pub correlation_id: krost::types::Int32,
        ///The client ID string.
        #[kafka(versions = "1+", nullable = "1+")]
        pub client_id: krost::types::NullableString,
        ///The tagged fields.
        #[kafka(versions = "2+")]
        pub _tagged_fields: krost::types::TaggedFields,
    }
}
