#![allow(dead_code)]
use krost_derive::Krost;
use krost::KrostType;
use from_variants::FromVariants;
pub mod request {
    pub mod produce {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 0i16, added = 0i16, removed = 9i16)]
        pub struct ProduceRequest {
            ///The transactional ID, or null if the producer is not transactional.
            #[kafka(added = 3i16, default = "null")]
            pub transactional_id: Option<krost::primitive::String>,
            ///The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
            #[kafka(added = 0i16)]
            pub acks: krost::primitive::Int16,
            ///The timeout to await a response in milliseconds.
            #[kafka(added = 0i16)]
            pub timeout_ms: krost::primitive::Int32,
            ///Each topic to produce to.
            #[kafka(added = 0i16)]
            pub topic_data: Vec<TopicProduceData>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionProduceData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub index: krost::primitive::Int32,
            ///The record data to be produced.
            #[kafka(added = 0i16)]
            pub records: Option<krost::record::RecordBatch>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicProduceData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition to produce to.
            #[kafka(added = 0i16)]
            pub partition_data: Vec<PartitionProduceData>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod fetch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 1i16, added = 0i16, removed = 13i16)]
        pub struct FetchRequest {
            ///The clusterId if known. This is used to validate metadata fetches prior to broker registration.
            #[kafka(added = 12i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            ///The broker ID of the follower, of -1 if this request is from a consumer.
            #[kafka(added = 0i16)]
            pub replica_id: krost::primitive::Int32,
            ///The maximum time in milliseconds to wait for the response.
            #[kafka(added = 0i16)]
            pub max_wait_ms: krost::primitive::Int32,
            ///The minimum bytes to accumulate in the response.
            #[kafka(added = 0i16)]
            pub min_bytes: krost::primitive::Int32,
            ///The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
            #[kafka(added = 3i16, default = "0x7fffffff")]
            pub max_bytes: krost::primitive::Int32,
            ///This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
            #[kafka(added = 4i16, default = "0")]
            pub isolation_level: krost::primitive::Int8,
            ///The fetch session ID.
            #[kafka(added = 7i16, default = "0")]
            pub session_id: krost::primitive::Int32,
            ///The fetch session epoch, which is used for ordering requests in a session.
            #[kafka(added = 7i16, default = "-1")]
            pub session_epoch: krost::primitive::Int32,
            ///The topics to fetch.
            #[kafka(added = 0i16)]
            pub topics: Vec<FetchTopic>,
            ///In an incremental fetch request, the partitions to remove.
            #[kafka(added = 7i16)]
            pub forgotten_topics_data: Vec<ForgottenTopic>,
            ///Rack ID of the consumer making this request
            #[kafka(added = 11i16, default = "")]
            pub rack_id: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct FetchPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition: krost::primitive::Int32,
            ///The current leader epoch of the partition.
            #[kafka(added = 9i16, default = "-1")]
            pub current_leader_epoch: krost::primitive::Int32,
            ///The message offset.
            #[kafka(added = 0i16)]
            pub fetch_offset: krost::primitive::Int64,
            ///The epoch of the last fetched record or -1 if there is none
            #[kafka(added = 12i16, default = "-1")]
            pub last_fetched_epoch: krost::primitive::Int32,
            ///The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
            #[kafka(added = 5i16, default = "-1")]
            pub log_start_offset: krost::primitive::Int64,
            ///The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
            #[kafka(added = 0i16)]
            pub partition_max_bytes: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct FetchTopic {
            ///The name of the topic to fetch.
            #[kafka(added = 0i16, removed = 12i16)]
            pub topic: krost::primitive::String,
            ///The unique topic ID
            #[kafka(added = 13i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The partitions to fetch.
            #[kafka(added = 0i16)]
            pub partitions: Vec<FetchPartition>,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ForgottenTopic {
            ///The topic name.
            #[kafka(added = 7i16, removed = 12i16)]
            pub topic: krost::primitive::String,
            ///The unique topic ID
            #[kafka(added = 13i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The partitions indexes to forget.
            #[kafka(added = 7i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_offsets {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 2i16, added = 0i16, removed = 7i16)]
        pub struct ListOffsetsRequest {
            ///The broker ID of the requestor, or -1 if this request is being made by a normal consumer.
            #[kafka(added = 0i16)]
            pub replica_id: krost::primitive::Int32,
            ///This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
            #[kafka(added = 2i16)]
            pub isolation_level: krost::primitive::Int8,
            ///Each topic in the request.
            #[kafka(added = 0i16)]
            pub topics: Vec<ListOffsetsTopic>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListOffsetsPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The current leader epoch.
            #[kafka(added = 4i16, default = "-1")]
            pub current_leader_epoch: krost::primitive::Int32,
            ///The current timestamp.
            #[kafka(added = 0i16)]
            pub timestamp: krost::primitive::Int64,
            ///The maximum number of offsets to report.
            #[kafka(added = 0i16, default = "1")]
            pub max_num_offsets: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListOffsetsTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition in the request.
            #[kafka(added = 0i16)]
            pub partitions: Vec<ListOffsetsPartition>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod metadata {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 3i16, added = 0i16, removed = 12i16)]
        pub struct MetadataRequest {
            ///The topics to fetch metadata for.
            #[kafka(added = 0i16)]
            pub topics: Option<Vec<MetadataRequestTopic>>,
            ///If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
            #[kafka(added = 4i16, default = "true")]
            pub allow_auto_topic_creation: krost::primitive::Bool,
            ///Whether to include cluster authorized operations.
            #[kafka(added = 8i16, removed = 10i16)]
            pub include_cluster_authorized_operations: krost::primitive::Bool,
            ///Whether to include topic authorized operations.
            #[kafka(added = 8i16)]
            pub include_topic_authorized_operations: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MetadataRequestTopic {
            ///The topic id.
            #[kafka(added = 10i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod leader_and_isr {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 4i16, added = 0i16, removed = 6i16)]
        pub struct LeaderAndIsrRequest {
            ///The current controller ID.
            #[kafka(added = 0i16)]
            pub controller_id: krost::primitive::Int32,
            ///The current controller epoch.
            #[kafka(added = 0i16)]
            pub controller_epoch: krost::primitive::Int32,
            ///The current broker epoch.
            #[kafka(added = 2i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///The type that indicates whether all topics are included in the request
            #[kafka(added = 5i16)]
            pub r#type: krost::primitive::Int8,
            ///The state of each partition, in a v0 or v1 message.
            #[kafka(added = 0i16, removed = 1i16)]
            pub ungrouped_partition_states: Vec<LeaderAndIsrPartitionState>,
            ///Each topic.
            #[kafka(added = 2i16)]
            pub topic_states: Vec<LeaderAndIsrTopicState>,
            ///The current live leaders.
            #[kafka(added = 0i16)]
            pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct LeaderAndIsrTopicState {
            ///The topic name.
            #[kafka(added = 2i16)]
            pub topic_name: krost::primitive::String,
            ///The unique topic ID.
            #[kafka(added = 5i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The state of each partition
            #[kafka(added = 2i16)]
            pub partition_states: Vec<LeaderAndIsrPartitionState>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct LeaderAndIsrLiveLeader {
            ///The leader's broker ID.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The leader's hostname.
            #[kafka(added = 0i16)]
            pub host_name: krost::primitive::String,
            ///The leader's port.
            #[kafka(added = 0i16)]
            pub port: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod stop_replica {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 5i16, added = 0i16, removed = 3i16)]
        pub struct StopReplicaRequest {
            ///The controller id.
            #[kafka(added = 0i16)]
            pub controller_id: krost::primitive::Int32,
            ///The controller epoch.
            #[kafka(added = 0i16)]
            pub controller_epoch: krost::primitive::Int32,
            ///The broker epoch.
            #[kafka(added = 1i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///Whether these partitions should be deleted.
            #[kafka(added = 0i16, removed = 2i16)]
            pub delete_partitions: krost::primitive::Bool,
            ///The partitions to stop.
            #[kafka(added = 0i16)]
            pub ungrouped_partitions: Vec<StopReplicaPartitionV0>,
            ///The topics to stop.
            #[kafka(added = 1i16, removed = 2i16)]
            pub topics: Vec<StopReplicaTopicV1>,
            ///Each topic.
            #[kafka(added = 3i16)]
            pub topic_states: Vec<StopReplicaTopicState>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct StopReplicaPartitionV0 {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct StopReplicaTopicV1 {
            ///The topic name.
            #[kafka(added = 1i16, removed = 2i16)]
            pub name: krost::primitive::String,
            ///The partition indexes.
            #[kafka(added = 1i16, removed = 2i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct StopReplicaPartitionState {
            ///The partition index.
            #[kafka(added = 3i16)]
            pub partition_index: krost::primitive::Int32,
            ///The leader epoch.
            #[kafka(added = 3i16, default = "-1")]
            pub leader_epoch: krost::primitive::Int32,
            ///Whether this partition should be deleted.
            #[kafka(added = 3i16)]
            pub delete_partition: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct StopReplicaTopicState {
            ///The topic name.
            #[kafka(added = 3i16)]
            pub topic_name: krost::primitive::String,
            ///The state of each partition
            #[kafka(added = 3i16)]
            pub partition_states: Vec<StopReplicaPartitionState>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod update_metadata {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 6i16, added = 0i16, removed = 7i16)]
        pub struct UpdateMetadataRequest {
            ///The controller id.
            #[kafka(added = 0i16)]
            pub controller_id: krost::primitive::Int32,
            ///The controller epoch.
            #[kafka(added = 0i16)]
            pub controller_epoch: krost::primitive::Int32,
            ///The broker epoch.
            #[kafka(added = 5i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///In older versions of this RPC, each partition that we would like to update.
            #[kafka(added = 0i16, removed = 4i16)]
            pub ungrouped_partition_states: Vec<UpdateMetadataPartitionState>,
            ///In newer versions of this RPC, each topic that we would like to update.
            #[kafka(added = 5i16)]
            pub topic_states: Vec<UpdateMetadataTopicState>,
            #[kafka(added = 0i16)]
            pub live_brokers: Vec<UpdateMetadataBroker>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct UpdateMetadataTopicState {
            ///The topic name.
            #[kafka(added = 5i16)]
            pub topic_name: krost::primitive::String,
            ///The topic id.
            #[kafka(added = 7i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The partition that we would like to update.
            #[kafka(added = 5i16)]
            pub partition_states: Vec<UpdateMetadataPartitionState>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct UpdateMetadataEndpoint {
            ///The port of this endpoint
            #[kafka(added = 1i16)]
            pub port: krost::primitive::Int32,
            ///The hostname of this endpoint
            #[kafka(added = 1i16)]
            pub host: krost::primitive::String,
            ///The listener name.
            #[kafka(added = 3i16)]
            pub listener: krost::primitive::String,
            ///The security protocol type.
            #[kafka(added = 1i16)]
            pub security_protocol: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct UpdateMetadataBroker {
            ///The broker id.
            #[kafka(added = 0i16)]
            pub id: krost::primitive::Int32,
            ///The broker hostname.
            #[kafka(added = 0i16)]
            pub v0_host: krost::primitive::String,
            ///The broker port.
            #[kafka(added = 0i16)]
            pub v0_port: krost::primitive::Int32,
            ///The broker endpoints.
            #[kafka(added = 1i16)]
            pub endpoints: Vec<UpdateMetadataEndpoint>,
            ///The rack which this broker belongs to.
            #[kafka(added = 2i16)]
            pub rack: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod controlled_shutdown {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 7i16, added = 0i16, removed = 3i16)]
        pub struct ControlledShutdownRequest {
            ///The id of the broker for which controlled shutdown has been requested.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The broker epoch.
            #[kafka(added = 2i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_commit {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 8i16, added = 0i16, removed = 8i16)]
        pub struct OffsetCommitRequest {
            ///The unique group identifier.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The generation of the group.
            #[kafka(added = 1i16, default = "-1")]
            pub generation_id: krost::primitive::Int32,
            ///The member ID assigned by the group coordinator.
            #[kafka(added = 1i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 7i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The time period in ms to retain the offset.
            #[kafka(added = 2i16, removed = 4i16, default = "-1")]
            pub retention_time_ms: krost::primitive::Int64,
            ///The topics to commit offsets for.
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetCommitRequestTopic>,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetCommitRequestPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The message offset to be committed.
            #[kafka(added = 0i16)]
            pub committed_offset: krost::primitive::Int64,
            ///The leader epoch of this partition.
            #[kafka(added = 6i16, default = "-1")]
            pub committed_leader_epoch: krost::primitive::Int32,
            ///The timestamp of the commit.
            #[kafka(added = 1i16, default = "-1")]
            pub commit_timestamp: krost::primitive::Int64,
            ///Any associated metadata the client wants to keep.
            #[kafka(added = 0i16)]
            pub committed_metadata: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetCommitRequestTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition to commit offsets for.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OffsetCommitRequestPartition>,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_fetch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 9i16, added = 0i16, removed = 8i16)]
        pub struct OffsetFetchRequest {
            ///The group to fetch offsets for.
            #[kafka(added = 0i16, removed = 7i16)]
            pub group_id: krost::primitive::String,
            ///Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
            #[kafka(added = 0i16, removed = 7i16)]
            pub topics: Option<Vec<OffsetFetchRequestTopic>>,
            ///Each group we would like to fetch offsets for
            #[kafka(added = 8i16)]
            pub groups: Vec<OffsetFetchRequestGroup>,
            ///Whether broker should hold on returning unstable offsets but set a retriable error code for the partitions.
            #[kafka(added = 7i16, default = "false")]
            pub require_stable: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchRequestTopic {
            ///The topic name.
            #[kafka(added = 0i16, removed = 7i16)]
            pub name: krost::primitive::String,
            ///The partition indexes we would like to fetch offsets for.
            #[kafka(added = 0i16, removed = 7i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchRequestTopics {
            ///The topic name.
            #[kafka(added = 8i16)]
            pub name: krost::primitive::String,
            ///The partition indexes we would like to fetch offsets for.
            #[kafka(added = 8i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchRequestGroup {
            ///The group ID.
            #[kafka(added = 8i16)]
            pub group_id: krost::primitive::String,
            ///Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
            #[kafka(added = 8i16)]
            pub topics: Option<Vec<OffsetFetchRequestTopics>>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod find_coordinator {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 10i16, added = 0i16, removed = 4i16)]
        pub struct FindCoordinatorRequest {
            ///The coordinator key.
            #[kafka(added = 0i16, removed = 3i16)]
            pub key: krost::primitive::String,
            ///The coordinator key type. (Group, transaction, etc.)
            #[kafka(added = 1i16, default = "0")]
            pub key_type: krost::primitive::Int8,
            ///The coordinator keys.
            #[kafka(added = 4i16)]
            pub coordinator_keys: Vec<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod join_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 11i16, added = 0i16, removed = 9i16)]
        pub struct JoinGroupRequest {
            ///The group identifier.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
            #[kafka(added = 0i16)]
            pub session_timeout_ms: krost::primitive::Int32,
            ///The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
            #[kafka(added = 1i16, default = "-1")]
            pub rebalance_timeout_ms: krost::primitive::Int32,
            ///The member id assigned by the group coordinator.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 5i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The unique name the for class of protocols implemented by the group we want to join.
            #[kafka(added = 0i16)]
            pub protocol_type: krost::primitive::String,
            ///The list of protocols that the member supports.
            #[kafka(added = 0i16)]
            pub protocols: Vec<JoinGroupRequestProtocol>,
            ///The reason why the member (re-)joins the group.
            #[kafka(added = 8i16, default = "null")]
            pub reason: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct JoinGroupRequestProtocol {
            ///The protocol name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The protocol metadata.
            #[kafka(added = 0i16)]
            pub metadata: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod heartbeat {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 12i16, added = 0i16, removed = 4i16)]
        pub struct HeartbeatRequest {
            ///The group id.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The generation of the group.
            #[kafka(added = 0i16)]
            pub generation_id: krost::primitive::Int32,
            ///The member ID.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 3i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod leave_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 13i16, added = 0i16, removed = 5i16)]
        pub struct LeaveGroupRequest {
            ///The ID of the group to leave.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The member ID to remove from the group.
            #[kafka(added = 0i16, removed = 2i16)]
            pub member_id: krost::primitive::String,
            ///List of leaving member identities.
            #[kafka(added = 3i16)]
            pub members: Vec<MemberIdentity>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MemberIdentity {
            ///The member ID to remove from the group.
            #[kafka(added = 3i16)]
            pub member_id: krost::primitive::String,
            ///The group instance ID to remove from the group.
            #[kafka(added = 3i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The reason why the member left the group.
            #[kafka(added = 5i16, default = "null")]
            pub reason: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sync_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 14i16, added = 0i16, removed = 5i16)]
        pub struct SyncGroupRequest {
            ///The unique group identifier.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The generation of the group.
            #[kafka(added = 0i16)]
            pub generation_id: krost::primitive::Int32,
            ///The member ID assigned by the group.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 3i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The group protocol type.
            #[kafka(added = 5i16, default = "null")]
            pub protocol_type: Option<krost::primitive::String>,
            ///The group protocol name.
            #[kafka(added = 5i16, default = "null")]
            pub protocol_name: Option<krost::primitive::String>,
            ///Each assignment.
            #[kafka(added = 0i16)]
            pub assignments: Vec<SyncGroupRequestAssignment>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct SyncGroupRequestAssignment {
            ///The ID of the member to assign.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The member assignment.
            #[kafka(added = 0i16)]
            pub assignment: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 15i16, added = 0i16, removed = 5i16)]
        pub struct DescribeGroupsRequest {
            ///The names of the groups to describe
            #[kafka(added = 0i16)]
            pub groups: Vec<krost::primitive::String>,
            ///Whether to include authorized operations.
            #[kafka(added = 3i16)]
            pub include_authorized_operations: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 16i16, added = 0i16, removed = 4i16)]
        pub struct ListGroupsRequest {
            ///The states of the groups we want to list. If empty all groups are returned with their state.
            #[kafka(added = 4i16)]
            pub states_filter: Vec<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sasl_handshake {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 17i16, added = 0i16, removed = 1i16)]
        pub struct SaslHandshakeRequest {
            ///The SASL mechanism chosen by the client.
            #[kafka(added = 0i16)]
            pub mechanism: krost::primitive::String,
        }
    }
    pub mod api_versions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 18i16, added = 0i16, removed = 3i16)]
        pub struct ApiVersionsRequest {
            ///The name of the client.
            #[kafka(added = 3i16)]
            pub client_software_name: krost::primitive::String,
            ///The version of the client.
            #[kafka(added = 3i16)]
            pub client_software_version: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_topics {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 19i16, added = 0i16, removed = 7i16)]
        pub struct CreateTopicsRequest {
            ///The topics to create.
            #[kafka(added = 0i16)]
            pub topics: Vec<CreatableTopic>,
            ///How long to wait in milliseconds before timing out the request.
            #[kafka(added = 0i16, default = "60000")]
            pub timeout_ms: krost::primitive::Int32,
            ///If true, check that the topics can be created as specified, but don't create anything.
            #[kafka(added = 1i16, default = "false")]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatableReplicaAssignment {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The brokers to place the partition on.
            #[kafka(added = 0i16)]
            pub broker_ids: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreateableTopicConfig {
            ///The configuration name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The configuration value.
            #[kafka(added = 0i16)]
            pub value: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatableTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The number of partitions to create in the topic, or -1 if we are either specifying a manual partition assignment or using the default partitions.
            #[kafka(added = 0i16)]
            pub num_partitions: krost::primitive::Int32,
            ///The number of replicas to create for each partition in the topic, or -1 if we are either specifying a manual partition assignment or using the default replication factor.
            #[kafka(added = 0i16)]
            pub replication_factor: krost::primitive::Int16,
            ///The manual partition assignment, or the empty array if we are using automatic assignment.
            #[kafka(added = 0i16)]
            pub assignments: Vec<CreatableReplicaAssignment>,
            ///The custom topic configurations to set.
            #[kafka(added = 0i16)]
            pub configs: Vec<CreateableTopicConfig>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_topics {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 20i16, added = 0i16, removed = 6i16)]
        pub struct DeleteTopicsRequest {
            ///The name or topic ID of the topic
            #[kafka(added = 6i16)]
            pub topics: Vec<DeleteTopicState>,
            ///The names of the topics to delete
            #[kafka(added = 0i16, removed = 5i16)]
            pub topic_names: Vec<krost::primitive::String>,
            ///The length of time in milliseconds to wait for the deletions to complete.
            #[kafka(added = 0i16)]
            pub timeout_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteTopicState {
            ///The topic name
            #[kafka(added = 6i16, default = "null")]
            pub name: Option<krost::primitive::String>,
            ///The unique topic ID
            #[kafka(added = 6i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_records {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 21i16, added = 0i16, removed = 2i16)]
        pub struct DeleteRecordsRequest {
            ///Each topic that we want to delete records from.
            #[kafka(added = 0i16)]
            pub topics: Vec<DeleteRecordsTopic>,
            ///How long to wait for the deletion to complete, in milliseconds.
            #[kafka(added = 0i16)]
            pub timeout_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteRecordsPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The deletion offset.
            #[kafka(added = 0i16)]
            pub offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteRecordsTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition that we want to delete records from.
            #[kafka(added = 0i16)]
            pub partitions: Vec<DeleteRecordsPartition>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod init_producer_id {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 22i16, added = 0i16, removed = 4i16)]
        pub struct InitProducerIdRequest {
            ///The transactional id, or null if the producer is not transactional.
            #[kafka(added = 0i16)]
            pub transactional_id: Option<krost::primitive::String>,
            ///The time in ms to wait before aborting idle transactions sent by this producer. This is only relevant if a TransactionalId has been defined.
            #[kafka(added = 0i16)]
            pub transaction_timeout_ms: krost::primitive::Int32,
            ///The producer id. This is used to disambiguate requests if a transactional id is reused following its expiration.
            #[kafka(added = 3i16, default = "-1")]
            pub producer_id: krost::primitive::Int64,
            ///The producer's current epoch. This will be checked against the producer epoch on the broker, and the request will return an error if they do not match.
            #[kafka(added = 3i16, default = "-1")]
            pub producer_epoch: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_for_leader_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 23i16, added = 0i16, removed = 4i16)]
        pub struct OffsetForLeaderEpochRequest {
            ///The broker ID of the follower, of -1 if this request is from a consumer.
            #[kafka(added = 3i16, default = -2f64)]
            pub replica_id: krost::primitive::Int32,
            ///Each topic to get offsets for.
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetForLeaderTopic>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetForLeaderPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition: krost::primitive::Int32,
            ///An epoch used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
            #[kafka(added = 2i16, default = "-1")]
            pub current_leader_epoch: krost::primitive::Int32,
            ///The epoch to look up an offset for.
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetForLeaderTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            ///Each partition to get offsets for.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OffsetForLeaderPartition>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod add_partitions_to_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 24i16, added = 0i16, removed = 3i16)]
        pub struct AddPartitionsToTxnRequest {
            ///The transactional id corresponding to the transaction.
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            ///Current producer id in use by the transactional id.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///Current epoch associated with the producer id.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The partitions to add to the transaction.
            #[kafka(added = 0i16)]
            pub topics: Vec<AddPartitionsToTxnTopic>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AddPartitionsToTxnTopic {
            ///The name of the topic.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partition indexes to add to the transaction
            #[kafka(added = 0i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod add_offsets_to_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 25i16, added = 0i16, removed = 3i16)]
        pub struct AddOffsetsToTxnRequest {
            ///The transactional id corresponding to the transaction.
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            ///Current producer id in use by the transactional id.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///Current epoch associated with the producer id.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The unique group identifier.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod end_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 26i16, added = 0i16, removed = 3i16)]
        pub struct EndTxnRequest {
            ///The ID of the transaction to end.
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            ///The producer ID.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///The current epoch associated with the producer.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///True if the transaction was committed, false if it was aborted.
            #[kafka(added = 0i16)]
            pub committed: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod write_txn_markers {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 27i16, added = 0i16, removed = 1i16)]
        pub struct WriteTxnMarkersRequest {
            ///The transaction markers to be written.
            #[kafka(added = 0i16)]
            pub markers: Vec<WritableTxnMarker>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct WritableTxnMarkerTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The indexes of the partitions to write transaction markers for.
            #[kafka(added = 0i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct WritableTxnMarker {
            ///The current producer ID.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///The current epoch associated with the producer ID.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
            #[kafka(added = 0i16)]
            pub transaction_result: krost::primitive::Bool,
            ///Each topic that we want to write transaction marker(s) for.
            #[kafka(added = 0i16)]
            pub topics: Vec<WritableTxnMarkerTopic>,
            ///Epoch associated with the transaction state partition hosted by this transaction coordinator
            #[kafka(added = 0i16)]
            pub coordinator_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod txn_offset_commit {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 28i16, added = 0i16, removed = 3i16)]
        pub struct TxnOffsetCommitRequest {
            ///The ID of the transaction.
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            ///The ID of the group.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The current producer ID in use by the transactional ID.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///The current epoch associated with the producer ID.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The generation of the consumer.
            #[kafka(added = 3i16, default = "-1")]
            pub generation_id: krost::primitive::Int32,
            ///The member ID assigned by the group coordinator.
            #[kafka(added = 3i16, default = "")]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 3i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///Each topic that we want to commit offsets for.
            #[kafka(added = 0i16)]
            pub topics: Vec<TxnOffsetCommitRequestTopic>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TxnOffsetCommitRequestPartition {
            ///The index of the partition within the topic.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The message offset to be committed.
            #[kafka(added = 0i16)]
            pub committed_offset: krost::primitive::Int64,
            ///The leader epoch of the last consumed record.
            #[kafka(added = 2i16, default = "-1")]
            pub committed_leader_epoch: krost::primitive::Int32,
            ///Any associated metadata the client wants to keep.
            #[kafka(added = 0i16)]
            pub committed_metadata: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TxnOffsetCommitRequestTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partitions inside the topic that we want to committ offsets for.
            #[kafka(added = 0i16)]
            pub partitions: Vec<TxnOffsetCommitRequestPartition>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 29i16, added = 0i16, removed = 2i16)]
        pub struct DescribeAclsRequest {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type_filter: krost::primitive::Int8,
            ///The resource name, or null to match any resource name.
            #[kafka(added = 0i16)]
            pub resource_name_filter: Option<krost::primitive::String>,
            ///The resource pattern to match.
            #[kafka(added = 1i16, default = "3")]
            pub pattern_type_filter: krost::primitive::Int8,
            ///The principal to match, or null to match any principal.
            #[kafka(added = 0i16)]
            pub principal_filter: Option<krost::primitive::String>,
            ///The host to match, or null to match any host.
            #[kafka(added = 0i16)]
            pub host_filter: Option<krost::primitive::String>,
            ///The operation to match.
            #[kafka(added = 0i16)]
            pub operation: krost::primitive::Int8,
            ///The permission type to match.
            #[kafka(added = 0i16)]
            pub permission_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 30i16, added = 0i16, removed = 2i16)]
        pub struct CreateAclsRequest {
            ///The ACLs that we want to create.
            #[kafka(added = 0i16)]
            pub creations: Vec<AclCreation>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AclCreation {
            ///The type of the resource.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name for the ACL.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The pattern type for the ACL.
            #[kafka(added = 1i16, default = "3")]
            pub resource_pattern_type: krost::primitive::Int8,
            ///The principal for the ACL.
            #[kafka(added = 0i16)]
            pub principal: krost::primitive::String,
            ///The host for the ACL.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The operation type for the ACL (read, write, etc.).
            #[kafka(added = 0i16)]
            pub operation: krost::primitive::Int8,
            ///The permission type for the ACL (allow, deny, etc.).
            #[kafka(added = 0i16)]
            pub permission_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 31i16, added = 0i16, removed = 2i16)]
        pub struct DeleteAclsRequest {
            ///The filters to use when deleting ACLs.
            #[kafka(added = 0i16)]
            pub filters: Vec<DeleteAclsFilter>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteAclsFilter {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type_filter: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name_filter: Option<krost::primitive::String>,
            ///The pattern type.
            #[kafka(added = 1i16, default = "3")]
            pub pattern_type_filter: krost::primitive::Int8,
            ///The principal filter, or null to accept all principals.
            #[kafka(added = 0i16)]
            pub principal_filter: Option<krost::primitive::String>,
            ///The host filter, or null to accept all hosts.
            #[kafka(added = 0i16)]
            pub host_filter: Option<krost::primitive::String>,
            ///The ACL operation.
            #[kafka(added = 0i16)]
            pub operation: krost::primitive::Int8,
            ///The permission type.
            #[kafka(added = 0i16)]
            pub permission_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 32i16, added = 0i16, removed = 4i16)]
        pub struct DescribeConfigsRequest {
            ///The resources whose configurations we want to describe.
            #[kafka(added = 0i16)]
            pub resources: Vec<DescribeConfigsResource>,
            ///True if we should include all synonyms.
            #[kafka(added = 1i16, default = "false")]
            pub include_synonyms: krost::primitive::Bool,
            ///True if we should include configuration documentation.
            #[kafka(added = 3i16, default = "false")]
            pub include_documentation: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeConfigsResource {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The configuration keys to list, or null to list all configuration keys.
            #[kafka(added = 0i16)]
            pub configuration_keys: Option<Vec<krost::primitive::String>>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 33i16, added = 0i16, removed = 2i16)]
        pub struct AlterConfigsRequest {
            ///The updates for each resource.
            #[kafka(added = 0i16)]
            pub resources: Vec<AlterConfigsResource>,
            ///True if we should validate the request, but not change the configurations.
            #[kafka(added = 0i16)]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterableConfig {
            ///The configuration key name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The value to set for the configuration key.
            #[kafka(added = 0i16)]
            pub value: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterConfigsResource {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The configurations.
            #[kafka(added = 0i16)]
            pub configs: Vec<AlterableConfig>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_replica_log_dirs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 34i16, added = 0i16, removed = 2i16)]
        pub struct AlterReplicaLogDirsRequest {
            ///The alterations to make for each directory.
            #[kafka(added = 0i16)]
            pub dirs: Vec<AlterReplicaLogDir>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterReplicaLogDirTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partition indexes.
            #[kafka(added = 0i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterReplicaLogDir {
            ///The absolute directory path.
            #[kafka(added = 0i16)]
            pub path: krost::primitive::String,
            ///The topics to add to the directory.
            #[kafka(added = 0i16)]
            pub topics: Vec<AlterReplicaLogDirTopic>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_log_dirs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 35i16, added = 0i16, removed = 3i16)]
        pub struct DescribeLogDirsRequest {
            ///Each topic that we want to describe log directories for, or null for all topics.
            #[kafka(added = 0i16)]
            pub topics: Option<Vec<DescribableLogDirTopic>>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribableLogDirTopic {
            ///The topic name
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            ///The partition indexes.
            #[kafka(added = 0i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sasl_authenticate {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 36i16, added = 0i16, removed = 2i16)]
        pub struct SaslAuthenticateRequest {
            ///The SASL authentication bytes from the client, as defined by the SASL mechanism.
            #[kafka(added = 0i16)]
            pub auth_bytes: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_partitions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 37i16, added = 0i16, removed = 3i16)]
        pub struct CreatePartitionsRequest {
            ///Each topic that we want to create new partitions inside.
            #[kafka(added = 0i16)]
            pub topics: Vec<CreatePartitionsTopic>,
            ///The time in ms to wait for the partitions to be created.
            #[kafka(added = 0i16)]
            pub timeout_ms: krost::primitive::Int32,
            ///If true, then validate the request, but don't actually increase the number of partitions.
            #[kafka(added = 0i16)]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatePartitionsAssignment {
            ///The assigned broker IDs.
            #[kafka(added = 0i16)]
            pub broker_ids: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatePartitionsTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The new partition count.
            #[kafka(added = 0i16)]
            pub count: krost::primitive::Int32,
            ///The new partition assignments.
            #[kafka(added = 0i16)]
            pub assignments: Option<Vec<CreatePartitionsAssignment>>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 38i16, added = 0i16, removed = 2i16)]
        pub struct CreateDelegationTokenRequest {
            ///A list of those who are allowed to renew this token before it expires.
            #[kafka(added = 0i16)]
            pub renewers: Vec<CreatableRenewers>,
            ///The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
            #[kafka(added = 0i16)]
            pub max_lifetime_ms: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatableRenewers {
            ///The type of the Kafka principal.
            #[kafka(added = 0i16)]
            pub principal_type: krost::primitive::String,
            ///The name of the Kafka principal.
            #[kafka(added = 0i16)]
            pub principal_name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod renew_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 39i16, added = 0i16, removed = 2i16)]
        pub struct RenewDelegationTokenRequest {
            ///The HMAC of the delegation token to be renewed.
            #[kafka(added = 0i16)]
            pub hmac: Vec<u8>,
            ///The renewal time period in milliseconds.
            #[kafka(added = 0i16)]
            pub renew_period_ms: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod expire_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 40i16, added = 0i16, removed = 2i16)]
        pub struct ExpireDelegationTokenRequest {
            ///The HMAC of the delegation token to be expired.
            #[kafka(added = 0i16)]
            pub hmac: Vec<u8>,
            ///The expiry time period in milliseconds.
            #[kafka(added = 0i16)]
            pub expiry_time_period_ms: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 41i16, added = 0i16, removed = 2i16)]
        pub struct DescribeDelegationTokenRequest {
            ///Each owner that we want to describe delegation tokens for, or null to describe all tokens.
            #[kafka(added = 0i16)]
            pub owners: Option<Vec<DescribeDelegationTokenOwner>>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeDelegationTokenOwner {
            ///The owner principal type.
            #[kafka(added = 0i16)]
            pub principal_type: krost::primitive::String,
            ///The owner principal name.
            #[kafka(added = 0i16)]
            pub principal_name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 42i16, added = 0i16, removed = 2i16)]
        pub struct DeleteGroupsRequest {
            ///The group names to delete.
            #[kafka(added = 0i16)]
            pub groups_names: Vec<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod elect_leaders {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 43i16, added = 0i16, removed = 2i16)]
        pub struct ElectLeadersRequest {
            ///Type of elections to conduct for the partition. A value of '0' elects the preferred replica. A value of '1' elects the first live replica if there are no in-sync replica.
            #[kafka(added = 1i16)]
            pub election_type: krost::primitive::Int8,
            ///The topic partitions to elect leaders.
            #[kafka(added = 0i16)]
            pub topic_partitions: Option<Vec<TopicPartitions>>,
            ///The time in ms to wait for the election to complete.
            #[kafka(added = 0i16, default = "60000")]
            pub timeout_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicPartitions {
            ///The name of a topic.
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            ///The partitions of this topic whose leader should be elected.
            #[kafka(added = 0i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod incremental_alter_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 44i16, added = 0i16, removed = 1i16)]
        pub struct IncrementalAlterConfigsRequest {
            ///The incremental updates for each resource.
            #[kafka(added = 0i16)]
            pub resources: Vec<AlterConfigsResource>,
            ///True if we should validate the request, but not change the configurations.
            #[kafka(added = 0i16)]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterableConfig {
            ///The configuration key name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The type (Set, Delete, Append, Subtract) of operation.
            #[kafka(added = 0i16)]
            pub config_operation: krost::primitive::Int8,
            ///The value to set for the configuration key.
            #[kafka(added = 0i16)]
            pub value: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterConfigsResource {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The configurations.
            #[kafka(added = 0i16)]
            pub configs: Vec<AlterableConfig>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_partition_reassignments {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 45i16, added = 0i16)]
        pub struct AlterPartitionReassignmentsRequest {
            ///The time in ms to wait for the request to complete.
            #[kafka(added = 0i16, default = "60000")]
            pub timeout_ms: krost::primitive::Int32,
            ///The topics to reassign.
            #[kafka(added = 0i16)]
            pub topics: Vec<ReassignableTopic>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ReassignablePartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The replicas to place the partitions on, or null to cancel a pending reassignment for this partition.
            #[kafka(added = 0i16, default = "null")]
            pub replicas: Option<Vec<krost::primitive::Int32>>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ReassignableTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partitions to reassign.
            #[kafka(added = 0i16)]
            pub partitions: Vec<ReassignablePartition>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_partition_reassignments {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 46i16, added = 0i16)]
        pub struct ListPartitionReassignmentsRequest {
            ///The time in ms to wait for the request to complete.
            #[kafka(added = 0i16, default = "60000")]
            pub timeout_ms: krost::primitive::Int32,
            ///The topics to list partition reassignments for, or null to list everything.
            #[kafka(added = 0i16, default = "null")]
            pub topics: Option<Vec<ListPartitionReassignmentsTopics>>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListPartitionReassignmentsTopics {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partitions to list partition reassignments for.
            #[kafka(added = 0i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_delete {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 47i16, added = 0i16)]
        pub struct OffsetDeleteRequest {
            ///The unique group identifier.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The topics to delete offsets for
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetDeleteRequestTopic>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetDeleteRequestPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetDeleteRequestTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition to delete offsets for.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OffsetDeleteRequestPartition>,
        }
    }
    pub mod describe_client_quotas {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 48i16, added = 0i16, removed = 1i16)]
        pub struct DescribeClientQuotasRequest {
            ///Filter components to apply to quota entities.
            #[kafka(added = 0i16)]
            pub components: Vec<ComponentData>,
            ///Whether the match is strict, i.e. should exclude entities with unspecified entity types.
            #[kafka(added = 0i16)]
            pub strict: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ComponentData {
            ///The entity type that the filter component applies to.
            #[kafka(added = 0i16)]
            pub entity_type: krost::primitive::String,
            ///How to match the entity {0 = exact name, 1 = default name, 2 = any specified name}.
            #[kafka(added = 0i16)]
            pub match_type: krost::primitive::Int8,
            ///The string to match against, or null if unused for the match type.
            #[kafka(added = 0i16)]
            pub r#match: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_client_quotas {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 49i16, added = 0i16, removed = 1i16)]
        pub struct AlterClientQuotasRequest {
            ///The quota configuration entries to alter.
            #[kafka(added = 0i16)]
            pub entries: Vec<EntryData>,
            ///Whether the alteration should be validated, but not performed.
            #[kafka(added = 0i16)]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(added = 0i16)]
            pub entity_type: krost::primitive::String,
            ///The name of the entity, or null if the default.
            #[kafka(added = 0i16)]
            pub entity_name: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OpData {
            ///The quota configuration key.
            #[kafka(added = 0i16)]
            pub key: krost::primitive::String,
            ///The value to set, otherwise ignored if the value is to be removed.
            #[kafka(added = 0i16)]
            pub value: float64,
            ///Whether the quota configuration value should be removed, otherwise set.
            #[kafka(added = 0i16)]
            pub remove: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntryData {
            ///The quota entity to alter.
            #[kafka(added = 0i16)]
            pub entity: Vec<EntityData>,
            ///An individual quota configuration entry to alter.
            #[kafka(added = 0i16)]
            pub ops: Vec<OpData>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_user_scram_credentials {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 50i16, added = 0i16)]
        pub struct DescribeUserScramCredentialsRequest {
            ///The users to describe, or null/empty to describe all users.
            #[kafka(added = 0i16)]
            pub users: Option<Vec<UserName>>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct UserName {
            ///The user name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_user_scram_credentials {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 51i16, added = 0i16)]
        pub struct AlterUserScramCredentialsRequest {
            ///The SCRAM credentials to remove.
            #[kafka(added = 0i16)]
            pub deletions: Vec<ScramCredentialDeletion>,
            ///The SCRAM credentials to update/insert.
            #[kafka(added = 0i16)]
            pub upsertions: Vec<ScramCredentialUpsertion>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ScramCredentialDeletion {
            ///The user name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The SCRAM mechanism.
            #[kafka(added = 0i16)]
            pub mechanism: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ScramCredentialUpsertion {
            ///The user name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The SCRAM mechanism.
            #[kafka(added = 0i16)]
            pub mechanism: krost::primitive::Int8,
            ///The number of iterations.
            #[kafka(added = 0i16)]
            pub iterations: krost::primitive::Int32,
            ///A random salt generated by the client.
            #[kafka(added = 0i16)]
            pub salt: Vec<u8>,
            ///The salted password.
            #[kafka(added = 0i16)]
            pub salted_password: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod vote {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 52i16, added = 0i16)]
        pub struct VoteRequest {
            #[kafka(added = 0i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The bumped epoch of the candidate sending the request
            #[kafka(added = 0i16)]
            pub candidate_epoch: krost::primitive::Int32,
            ///The ID of the voter sending the request
            #[kafka(added = 0i16)]
            pub candidate_id: krost::primitive::Int32,
            ///The epoch of the last record written to the metadata log
            #[kafka(added = 0i16)]
            pub last_offset_epoch: krost::primitive::Int32,
            ///The offset of the last record written to the metadata log
            #[kafka(added = 0i16)]
            pub last_offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod begin_quorum_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 53i16, added = 0i16)]
        pub struct BeginQuorumEpochRequest {
            #[kafka(added = 0i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The ID of the newly elected leader
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The epoch of the newly elected leader
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
        }
    }
    pub mod end_quorum_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 54i16, added = 0i16)]
        pub struct EndQuorumEpochRequest {
            #[kafka(added = 0i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The current leader ID that is resigning
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The current epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///A sorted list of preferred successors to start the election
            #[kafka(added = 0i16)]
            pub preferred_successors: Vec<krost::primitive::Int32>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
        }
    }
    pub mod describe_quorum {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 55i16, added = 0i16)]
        pub struct DescribeQuorumRequest {
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_partition {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 56i16, added = 0i16, removed = 1i16)]
        pub struct AlterPartitionRequest {
            ///The ID of the requesting broker
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The epoch of the requesting broker
            #[kafka(added = 0i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The leader epoch of this partition
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///The ISR for this partition
            #[kafka(added = 0i16)]
            pub new_isr: Vec<krost::primitive::Int32>,
            ///1 if the partition is recovering from an unclean leader election; 0 otherwise.
            #[kafka(added = 1i16, default = "0")]
            pub leader_recovery_state: krost::primitive::Int8,
            ///The expected epoch of the partition which is being updated. For legacy cluster this is the ZkVersion in the LeaderAndIsr request.
            #[kafka(added = 0i16)]
            pub partition_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The name of the topic to alter ISRs for
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod update_features {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 57i16, added = 0i16, removed = 1i16)]
        pub struct UpdateFeaturesRequest {
            ///How long to wait in milliseconds before timing out the request.
            #[kafka(added = 0i16, default = "60000")]
            pub timeout_ms: krost::primitive::Int32,
            ///The list of updates to finalized features.
            #[kafka(added = 0i16)]
            pub feature_updates: Vec<FeatureUpdateKey>,
            ///True if we should validate the request, but not perform the upgrade or downgrade.
            #[kafka(added = 1i16, default = false)]
            pub validate_only: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct FeatureUpdateKey {
            ///The name of the finalized feature to be updated.
            #[kafka(added = 0i16)]
            pub feature: krost::primitive::String,
            ///The new maximum version level for the finalized feature. A value >= 1 is valid. A value < 1, is special, and can be used to request the deletion of the finalized feature.
            #[kafka(added = 0i16)]
            pub max_version_level: krost::primitive::Int16,
            ///DEPRECATED in version 1 (see DowngradeType). When set to true, the finalized feature version level is allowed to be downgraded/deleted. The downgrade request will fail if the new maximum version level is a value that's not lower than the existing maximum finalized version level.
            #[kafka(added = 0i16)]
            pub allow_downgrade: krost::primitive::Bool,
            ///Determine which type of upgrade will be performed: 1 will perform an upgrade only (default), 2 is safe downgrades only (lossless), 3 is unsafe downgrades (lossy).
            #[kafka(added = 1i16, default = 1f64)]
            pub upgrade_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod envelope {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 58i16, added = 0i16)]
        pub struct EnvelopeRequest {
            ///The embedded request header and data.
            #[kafka(added = 0i16)]
            pub request_data: Vec<u8>,
            ///Value of the initial client principal when the request is redirected by a broker.
            #[kafka(added = 0i16)]
            pub request_principal: Option<Vec<u8>>,
            ///The original client's address in bytes.
            #[kafka(added = 0i16)]
            pub client_host_address: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod fetch_snapshot {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 59i16, added = 0i16)]
        pub struct FetchSnapshotRequest {
            ///The clusterId if known, this is used to validate metadata fetches prior to broker registration
            #[kafka(added = 0i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            ///The broker ID of the follower
            #[kafka(added = 0i16, default = "-1")]
            pub replica_id: krost::primitive::Int32,
            ///The maximum bytes to fetch from all of the snapshots
            #[kafka(added = 0i16, default = "0x7fffffff")]
            pub max_bytes: krost::primitive::Int32,
            ///The topics to fetch
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicSnapshot>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct SnapshotId {
            #[kafka(added = 0i16)]
            pub end_offset: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionSnapshot {
            ///The partition index
            #[kafka(added = 0i16)]
            pub partition: krost::primitive::Int32,
            ///The current leader epoch of the partition, -1 for unknown leader epoch
            #[kafka(added = 0i16)]
            pub current_leader_epoch: krost::primitive::Int32,
            ///The snapshot endOffset and epoch to fetch
            #[kafka(added = 0i16)]
            pub snapshot_id: SnapshotId,
            ///The byte position within the snapshot to start fetching from
            #[kafka(added = 0i16)]
            pub position: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicSnapshot {
            ///The name of the topic to fetch
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partitions to fetch
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionSnapshot>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_cluster {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 60i16, added = 0i16)]
        pub struct DescribeClusterRequest {
            ///Whether to include cluster authorized operations.
            #[kafka(added = 0i16)]
            pub include_cluster_authorized_operations: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_producers {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 61i16, added = 0i16)]
        pub struct DescribeProducersRequest {
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicRequest>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicRequest {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The indexes of the partitions to list producers for.
            #[kafka(added = 0i16)]
            pub partition_indexes: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod broker_registration {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 62i16, added = 0i16)]
        pub struct BrokerRegistrationRequest {
            ///The broker ID.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The cluster id of the broker process.
            #[kafka(added = 0i16)]
            pub cluster_id: krost::primitive::String,
            ///The incarnation id of the broker process.
            #[kafka(added = 0i16)]
            pub incarnation_id: krost::primitive::Uuid,
            ///The listeners of this broker
            #[kafka(added = 0i16)]
            pub listeners: Vec<Listener>,
            ///The features on this broker
            #[kafka(added = 0i16)]
            pub features: Vec<Feature>,
            ///The rack which this broker is in.
            #[kafka(added = 0i16)]
            pub rack: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct Listener {
            ///The name of the endpoint.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The hostname.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The port.
            #[kafka(added = 0i16)]
            pub port: uint16,
            ///The security protocol.
            #[kafka(added = 0i16)]
            pub security_protocol: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct Feature {
            ///The feature name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The minimum supported feature level.
            #[kafka(added = 0i16)]
            pub min_supported_version: krost::primitive::Int16,
            ///The maximum supported feature level.
            #[kafka(added = 0i16)]
            pub max_supported_version: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod broker_heartbeat {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 63i16, added = 0i16)]
        pub struct BrokerHeartbeatRequest {
            ///The broker ID.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The broker epoch.
            #[kafka(added = 0i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///The highest metadata offset which the broker has reached.
            #[kafka(added = 0i16)]
            pub current_metadata_offset: krost::primitive::Int64,
            ///True if the broker wants to be fenced, false otherwise.
            #[kafka(added = 0i16)]
            pub want_fence: krost::primitive::Bool,
            ///True if the broker wants to be shut down, false otherwise.
            #[kafka(added = 0i16)]
            pub want_shut_down: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod unregister_broker {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 64i16, added = 0i16)]
        pub struct UnregisterBrokerRequest {
            ///The broker ID to unregister.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_transactions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 65i16, added = 0i16)]
        pub struct DescribeTransactionsRequest {
            ///Array of transactionalIds to include in describe results. If empty, then no results will be returned.
            #[kafka(added = 0i16)]
            pub transactional_ids: Vec<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_transactions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 66i16, added = 0i16)]
        pub struct ListTransactionsRequest {
            ///The transaction states to filter by: if empty, all transactions are returned; if non-empty, then only transactions matching one of the filtered states will be returned
            #[kafka(added = 0i16)]
            pub state_filters: Vec<krost::primitive::String>,
            ///The producerIds to filter by: if empty, all transactions will be returned; if non-empty, only transactions which match one of the filtered producerIds will be returned
            #[kafka(added = 0i16)]
            pub producer_id_filters: Vec<krost::primitive::Int64>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod allocate_producer_ids {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 67i16, added = 0i16)]
        pub struct AllocateProducerIdsRequest {
            ///The ID of the requesting broker
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The epoch of the requesting broker
            #[kafka(added = 0i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
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
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 0i16, added = 0i16, removed = 9i16)]
        pub struct ProduceResponse {
            ///Each produce response
            #[kafka(added = 0i16)]
            pub responses: Vec<TopicProduceResponse>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16, default = "0")]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct BatchIndexAndErrorMessage {
            ///The batch index of the record that cause the batch to be dropped
            #[kafka(added = 8i16)]
            pub batch_index: krost::primitive::Int32,
            ///The error message of the record that caused the batch to be dropped
            #[kafka(added = 8i16, default = "null")]
            pub batch_index_error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionProduceResponse {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The base offset.
            #[kafka(added = 0i16)]
            pub base_offset: krost::primitive::Int64,
            ///The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
            #[kafka(added = 2i16, default = "-1")]
            pub log_append_time_ms: krost::primitive::Int64,
            ///The log start offset.
            #[kafka(added = 5i16, default = "-1")]
            pub log_start_offset: krost::primitive::Int64,
            ///The batch indices of records that caused the batch to be dropped
            #[kafka(added = 8i16)]
            pub record_errors: Vec<BatchIndexAndErrorMessage>,
            ///The global error message summarizing the common root cause of the records that caused the batch to be dropped
            #[kafka(added = 8i16, default = "null")]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicProduceResponse {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition that we produced to within the topic.
            #[kafka(added = 0i16)]
            pub partition_responses: Vec<PartitionProduceResponse>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod fetch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 1i16, added = 0i16, removed = 13i16)]
        pub struct FetchResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top level response error code.
            #[kafka(added = 7i16)]
            pub error_code: krost::primitive::Int16,
            ///The fetch session ID, or 0 if this is not part of a fetch session.
            #[kafka(added = 7i16, default = "0")]
            pub session_id: krost::primitive::Int32,
            ///The response topics.
            #[kafka(added = 0i16)]
            pub responses: Vec<FetchableTopicResponse>,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EpochEndOffset {
            #[kafka(added = 12i16, default = "-1")]
            pub epoch: krost::primitive::Int32,
            #[kafka(added = 12i16, default = "-1")]
            pub end_offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct LeaderIdAndEpoch {
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 12i16, default = "-1")]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 12i16, default = "-1")]
            pub leader_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct SnapshotId {
            #[kafka(added = 0i16, default = "-1")]
            pub end_offset: krost::primitive::Int64,
            #[kafka(added = 0i16, default = "-1")]
            pub epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AbortedTransaction {
            ///The producer id associated with the aborted transaction.
            #[kafka(added = 4i16)]
            pub producer_id: krost::primitive::Int64,
            ///The first offset in the aborted transaction.
            #[kafka(added = 4i16)]
            pub first_offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no fetch error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The current high water mark.
            #[kafka(added = 0i16)]
            pub high_watermark: krost::primitive::Int64,
            ///The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
            #[kafka(added = 4i16, default = "-1")]
            pub last_stable_offset: krost::primitive::Int64,
            ///The current log start offset.
            #[kafka(added = 5i16, default = "-1")]
            pub log_start_offset: krost::primitive::Int64,
            ///In case divergence is detected based on the `LastFetchedEpoch` and `FetchOffset` in the request, this field indicates the largest epoch and its end offset such that subsequent records are known to diverge
            #[kafka(added = 12i16)]
            pub diverging_epoch: EpochEndOffset,
            #[kafka(added = 12i16)]
            pub current_leader: LeaderIdAndEpoch,
            ///In the case of fetching an offset less than the LogStartOffset, this is the end offset and epoch that should be used in the FetchSnapshot request.
            #[kafka(added = 12i16)]
            pub snapshot_id: SnapshotId,
            ///The aborted transactions.
            #[kafka(added = 4i16)]
            pub aborted_transactions: Option<Vec<AbortedTransaction>>,
            ///The preferred read replica for the consumer to use on its next fetch request
            #[kafka(added = 11i16, default = "-1")]
            pub preferred_read_replica: krost::primitive::Int32,
            ///The record data.
            #[kafka(added = 0i16)]
            pub records: Option<krost::record::RecordBatch>,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct FetchableTopicResponse {
            ///The topic name.
            #[kafka(added = 0i16, removed = 12i16)]
            pub topic: krost::primitive::String,
            ///The unique topic ID
            #[kafka(added = 13i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The topic partitions.
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 12i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_offsets {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 2i16, added = 0i16, removed = 7i16)]
        pub struct ListOffsetsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 2i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each topic in the response.
            #[kafka(added = 0i16)]
            pub topics: Vec<ListOffsetsTopicResponse>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListOffsetsPartitionResponse {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The partition error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The result offsets.
            #[kafka(added = 0i16)]
            pub old_style_offsets: Vec<krost::primitive::Int64>,
            ///The timestamp associated with the returned offset.
            #[kafka(added = 1i16, default = "-1")]
            pub timestamp: krost::primitive::Int64,
            ///The returned offset.
            #[kafka(added = 1i16, default = "-1")]
            pub offset: krost::primitive::Int64,
            #[kafka(added = 4i16, default = "-1")]
            pub leader_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListOffsetsTopicResponse {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition in the response.
            #[kafka(added = 0i16)]
            pub partitions: Vec<ListOffsetsPartitionResponse>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod metadata {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 3i16, added = 0i16, removed = 12i16)]
        pub struct MetadataResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 3i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each broker in the response.
            #[kafka(added = 0i16)]
            pub brokers: Vec<MetadataResponseBroker>,
            ///The cluster ID that responding broker belongs to.
            #[kafka(added = 2i16, default = "null")]
            pub cluster_id: Option<krost::primitive::String>,
            ///The ID of the controller broker.
            #[kafka(added = 1i16, default = "-1")]
            pub controller_id: krost::primitive::Int32,
            ///Each topic in the response.
            #[kafka(added = 0i16)]
            pub topics: Vec<MetadataResponseTopic>,
            ///32-bit bitfield to represent authorized operations for this cluster.
            #[kafka(added = 8i16, removed = 10i16, default = "-2147483648")]
            pub cluster_authorized_operations: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MetadataResponseBroker {
            ///The broker ID.
            #[kafka(added = 0i16)]
            pub node_id: krost::primitive::Int32,
            ///The broker hostname.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The broker port.
            #[kafka(added = 0i16)]
            pub port: krost::primitive::Int32,
            ///The rack of the broker, or null if it has not been assigned to a rack.
            #[kafka(added = 1i16, default = "null")]
            pub rack: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MetadataResponsePartition {
            ///The partition error, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The ID of the leader broker.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The leader epoch of this partition.
            #[kafka(added = 7i16, default = "-1")]
            pub leader_epoch: krost::primitive::Int32,
            ///The set of all nodes that host this partition.
            #[kafka(added = 0i16)]
            pub replica_nodes: Vec<krost::primitive::Int32>,
            ///The set of nodes that are in sync with the leader for this partition.
            #[kafka(added = 0i16)]
            pub isr_nodes: Vec<krost::primitive::Int32>,
            ///The set of offline replicas of this partition.
            #[kafka(added = 5i16)]
            pub offline_replicas: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MetadataResponseTopic {
            ///The topic error, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: Option<krost::primitive::String>,
            ///The topic id.
            #[kafka(added = 10i16)]
            pub topic_id: krost::primitive::Uuid,
            ///True if the topic is internal.
            #[kafka(added = 1i16, default = "false")]
            pub is_internal: krost::primitive::Bool,
            ///Each partition in the topic.
            #[kafka(added = 0i16)]
            pub partitions: Vec<MetadataResponsePartition>,
            ///32-bit bitfield to represent authorized operations for this topic.
            #[kafka(added = 8i16, default = "-2147483648")]
            pub topic_authorized_operations: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 9i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod leader_and_isr {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 4i16, added = 0i16, removed = 6i16)]
        pub struct LeaderAndIsrResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///Each partition in v0 to v4 message.
            #[kafka(added = 0i16, removed = 4i16)]
            pub partition_errors: Vec<LeaderAndIsrPartitionError>,
            ///Each topic
            #[kafka(added = 5i16)]
            pub topics: Vec<LeaderAndIsrTopicError>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct LeaderAndIsrTopicError {
            ///The unique topic ID
            #[kafka(added = 5i16)]
            pub topic_id: krost::primitive::Uuid,
            ///Each partition.
            #[kafka(added = 5i16)]
            pub partition_errors: Vec<LeaderAndIsrPartitionError>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod stop_replica {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 5i16, added = 0i16, removed = 3i16)]
        pub struct StopReplicaResponse {
            ///The top-level error code, or 0 if there was no top-level error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The responses for each partition.
            #[kafka(added = 0i16)]
            pub partition_errors: Vec<StopReplicaPartitionError>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct StopReplicaPartitionError {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The partition error code, or 0 if there was no partition error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod update_metadata {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 6i16, added = 0i16, removed = 7i16)]
        pub struct UpdateMetadataResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod controlled_shutdown {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 7i16, added = 0i16, removed = 3i16)]
        pub struct ControlledShutdownResponse {
            ///The top-level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The partitions that the broker still leads.
            #[kafka(added = 0i16)]
            pub remaining_partitions: Vec<RemainingPartition>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct RemainingPartition {
            ///The name of the topic.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            ///The index of the partition.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_commit {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 8i16, added = 0i16, removed = 8i16)]
        pub struct OffsetCommitResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 3i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses for each topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetCommitResponseTopic>,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetCommitResponsePartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetCommitResponseTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The responses for each partition in the topic.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OffsetCommitResponsePartition>,
            ///The tagged fields.
            #[kafka(added = 8i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_fetch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 9i16, added = 0i16, removed = 8i16)]
        pub struct OffsetFetchResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 3i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses per topic.
            #[kafka(added = 0i16, removed = 7i16)]
            pub topics: Vec<OffsetFetchResponseTopic>,
            ///The top-level error code, or 0 if there was no error.
            #[kafka(added = 2i16, removed = 7i16, default = "0")]
            pub error_code: krost::primitive::Int16,
            ///The responses per group id.
            #[kafka(added = 8i16)]
            pub groups: Vec<OffsetFetchResponseGroup>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchResponsePartition {
            ///The partition index.
            #[kafka(added = 0i16, removed = 7i16)]
            pub partition_index: krost::primitive::Int32,
            ///The committed message offset.
            #[kafka(added = 0i16, removed = 7i16)]
            pub committed_offset: krost::primitive::Int64,
            ///The leader epoch.
            #[kafka(added = 5i16, removed = 7i16, default = "-1")]
            pub committed_leader_epoch: krost::primitive::Int32,
            ///The partition metadata.
            #[kafka(added = 0i16, removed = 7i16)]
            pub metadata: Option<krost::primitive::String>,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16, removed = 7i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchResponseTopic {
            ///The topic name.
            #[kafka(added = 0i16, removed = 7i16)]
            pub name: krost::primitive::String,
            ///The responses per partition
            #[kafka(added = 0i16, removed = 7i16)]
            pub partitions: Vec<OffsetFetchResponsePartition>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchResponsePartitions {
            ///The partition index.
            #[kafka(added = 8i16)]
            pub partition_index: krost::primitive::Int32,
            ///The committed message offset.
            #[kafka(added = 8i16)]
            pub committed_offset: krost::primitive::Int64,
            ///The leader epoch.
            #[kafka(added = 8i16, default = "-1")]
            pub committed_leader_epoch: krost::primitive::Int32,
            ///The partition metadata.
            #[kafka(added = 8i16)]
            pub metadata: Option<krost::primitive::String>,
            ///The partition-level error code, or 0 if there was no error.
            #[kafka(added = 8i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchResponseTopics {
            ///The topic name.
            #[kafka(added = 8i16)]
            pub name: krost::primitive::String,
            ///The responses per partition
            #[kafka(added = 8i16)]
            pub partitions: Vec<OffsetFetchResponsePartitions>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetFetchResponseGroup {
            ///The group ID.
            #[kafka(added = 8i16)]
            pub group_id: krost::primitive::String,
            ///The responses per topic.
            #[kafka(added = 8i16)]
            pub topics: Vec<OffsetFetchResponseTopics>,
            ///The group-level error code, or 0 if there was no error.
            #[kafka(added = 8i16, default = "0")]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod find_coordinator {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 10i16, added = 0i16, removed = 4i16)]
        pub struct FindCoordinatorResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16, removed = 3i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 1i16, removed = 3i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The node id.
            #[kafka(added = 0i16, removed = 3i16)]
            pub node_id: krost::primitive::Int32,
            ///The host name.
            #[kafka(added = 0i16, removed = 3i16)]
            pub host: krost::primitive::String,
            ///The port.
            #[kafka(added = 0i16, removed = 3i16)]
            pub port: krost::primitive::Int32,
            ///Each coordinator result in the response
            #[kafka(added = 4i16)]
            pub coordinators: Vec<Coordinator>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct Coordinator {
            ///The coordinator key.
            #[kafka(added = 4i16)]
            pub key: krost::primitive::String,
            ///The node id.
            #[kafka(added = 4i16)]
            pub node_id: krost::primitive::Int32,
            ///The host name.
            #[kafka(added = 4i16)]
            pub host: krost::primitive::String,
            ///The port.
            #[kafka(added = 4i16)]
            pub port: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 4i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 4i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod join_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 11i16, added = 0i16, removed = 9i16)]
        pub struct JoinGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 2i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The generation ID of the group.
            #[kafka(added = 0i16, default = "-1")]
            pub generation_id: krost::primitive::Int32,
            ///The group protocol name.
            #[kafka(added = 7i16, default = "null")]
            pub protocol_type: Option<krost::primitive::String>,
            ///The group protocol selected by the coordinator.
            #[kafka(added = 0i16)]
            pub protocol_name: Option<krost::primitive::String>,
            ///The leader of the group.
            #[kafka(added = 0i16)]
            pub leader: krost::primitive::String,
            ///True if the leader must skip running the assignment.
            #[kafka(added = 9i16, default = "false")]
            pub skip_assignment: krost::primitive::Bool,
            ///The member ID assigned by the group coordinator.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub members: Vec<JoinGroupResponseMember>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct JoinGroupResponseMember {
            ///The group member ID.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 5i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The group member metadata.
            #[kafka(added = 0i16)]
            pub metadata: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 6i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod heartbeat {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 12i16, added = 0i16, removed = 4i16)]
        pub struct HeartbeatResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod leave_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 13i16, added = 0i16, removed = 5i16)]
        pub struct LeaveGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///List of leaving member responses.
            #[kafka(added = 3i16)]
            pub members: Vec<MemberResponse>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct MemberResponse {
            ///The member ID to remove from the group.
            #[kafka(added = 3i16)]
            pub member_id: krost::primitive::String,
            ///The group instance ID to remove from the group.
            #[kafka(added = 3i16)]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 3i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sync_group {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 14i16, added = 0i16, removed = 5i16)]
        pub struct SyncGroupResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The group protocol type.
            #[kafka(added = 5i16, default = "null")]
            pub protocol_type: Option<krost::primitive::String>,
            ///The group protocol name.
            #[kafka(added = 5i16, default = "null")]
            pub protocol_name: Option<krost::primitive::String>,
            ///The member assignment.
            #[kafka(added = 0i16)]
            pub assignment: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 15i16, added = 0i16, removed = 5i16)]
        pub struct DescribeGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each described group.
            #[kafka(added = 0i16)]
            pub groups: Vec<DescribedGroup>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribedGroupMember {
            ///The member ID assigned by the group coordinator.
            #[kafka(added = 0i16)]
            pub member_id: krost::primitive::String,
            ///The unique identifier of the consumer instance provided by end user.
            #[kafka(added = 4i16, default = "null")]
            pub group_instance_id: Option<krost::primitive::String>,
            ///The client ID used in the member's latest join group request.
            #[kafka(added = 0i16)]
            pub client_id: krost::primitive::String,
            ///The client host.
            #[kafka(added = 0i16)]
            pub client_host: krost::primitive::String,
            ///The metadata corresponding to the current group protocol in use.
            #[kafka(added = 0i16)]
            pub member_metadata: Vec<u8>,
            ///The current assignment provided by the group leader.
            #[kafka(added = 0i16)]
            pub member_assignment: Vec<u8>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribedGroup {
            ///The describe error, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The group ID string.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The group state string, or the empty string.
            #[kafka(added = 0i16)]
            pub group_state: krost::primitive::String,
            ///The group protocol type, or the empty string.
            #[kafka(added = 0i16)]
            pub protocol_type: krost::primitive::String,
            ///The group protocol data, or the empty string.
            #[kafka(added = 0i16)]
            pub protocol_data: krost::primitive::String,
            ///The group members.
            #[kafka(added = 0i16)]
            pub members: Vec<DescribedGroupMember>,
            ///32-bit bitfield to represent authorized operations for this group.
            #[kafka(added = 3i16, default = "-2147483648")]
            pub authorized_operations: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 16i16, added = 0i16, removed = 4i16)]
        pub struct ListGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///Each group in the response.
            #[kafka(added = 0i16)]
            pub groups: Vec<ListedGroup>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ListedGroup {
            ///The group ID.
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The group protocol type.
            #[kafka(added = 0i16)]
            pub protocol_type: krost::primitive::String,
            ///The group state name.
            #[kafka(added = 4i16)]
            pub group_state: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sasl_handshake {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 17i16, added = 0i16, removed = 1i16)]
        pub struct SaslHandshakeResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The mechanisms enabled in the server.
            #[kafka(added = 0i16)]
            pub mechanisms: Vec<krost::primitive::String>,
        }
    }
    pub mod api_versions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 18i16, added = 0i16, removed = 3i16)]
        pub struct ApiVersionsResponse {
            ///The top-level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The APIs supported by the broker.
            #[kafka(added = 0i16)]
            pub api_keys: Vec<ApiVersion>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Features supported by the broker.
            #[kafka(added = 3i16)]
            pub supported_features: Vec<SupportedFeatureKey>,
            ///The monotonically increasing epoch for the finalized features information. Valid values are >= 0. A value of -1 is special and represents unknown epoch.
            #[kafka(added = 3i16, default = "-1")]
            pub finalized_features_epoch: krost::primitive::Int64,
            ///List of cluster-wide finalized features. The information is valid only if FinalizedFeaturesEpoch >= 0.
            #[kafka(added = 3i16)]
            pub finalized_features: Vec<FinalizedFeatureKey>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ApiVersion {
            ///The API index.
            #[kafka(added = 0i16)]
            pub api_key: krost::primitive::Int16,
            ///The minimum supported version, inclusive.
            #[kafka(added = 0i16)]
            pub min_version: krost::primitive::Int16,
            ///The maximum supported version, inclusive.
            #[kafka(added = 0i16)]
            pub max_version: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct SupportedFeatureKey {
            ///The name of the feature.
            #[kafka(added = 3i16)]
            pub name: krost::primitive::String,
            ///The minimum supported version for the feature.
            #[kafka(added = 3i16)]
            pub min_version: krost::primitive::Int16,
            ///The maximum supported version for the feature.
            #[kafka(added = 3i16)]
            pub max_version: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct FinalizedFeatureKey {
            ///The name of the feature.
            #[kafka(added = 3i16)]
            pub name: krost::primitive::String,
            ///The cluster-wide finalized max version level for the feature.
            #[kafka(added = 3i16)]
            pub max_version_level: krost::primitive::Int16,
            ///The cluster-wide finalized min version level for the feature.
            #[kafka(added = 3i16)]
            pub min_version_level: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_topics {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 19i16, added = 0i16, removed = 7i16)]
        pub struct CreateTopicsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 2i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Results for each topic we tried to create.
            #[kafka(added = 0i16)]
            pub topics: Vec<CreatableTopicResult>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatableTopicConfigs {
            ///The configuration name.
            #[kafka(added = 5i16)]
            pub name: krost::primitive::String,
            ///The configuration value.
            #[kafka(added = 5i16)]
            pub value: Option<krost::primitive::String>,
            ///True if the configuration is read-only.
            #[kafka(added = 5i16)]
            pub read_only: krost::primitive::Bool,
            ///The configuration source.
            #[kafka(added = 5i16, default = "-1")]
            pub config_source: krost::primitive::Int8,
            ///True if this configuration is sensitive.
            #[kafka(added = 5i16)]
            pub is_sensitive: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatableTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The unique topic ID
            #[kafka(added = 7i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 1i16)]
            pub error_message: Option<krost::primitive::String>,
            ///Optional topic config error returned if configs are not returned in the response.
            #[kafka(added = 5i16)]
            pub topic_config_error_code: krost::primitive::Int16,
            ///Number of partitions of the topic.
            #[kafka(added = 5i16, default = "-1")]
            pub num_partitions: krost::primitive::Int32,
            ///Replication factor of the topic.
            #[kafka(added = 5i16, default = "-1")]
            pub replication_factor: krost::primitive::Int16,
            ///Configuration of the topic.
            #[kafka(added = 5i16)]
            pub configs: Option<Vec<CreatableTopicConfigs>>,
            ///The tagged fields.
            #[kafka(added = 5i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_topics {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 20i16, added = 0i16, removed = 6i16)]
        pub struct DeleteTopicsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 1i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each topic we tried to delete.
            #[kafka(added = 0i16)]
            pub responses: Vec<DeletableTopicResult>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeletableTopicResult {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: Option<krost::primitive::String>,
            ///the unique topic ID
            #[kafka(added = 6i16)]
            pub topic_id: krost::primitive::Uuid,
            ///The deletion error, or 0 if the deletion succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 5i16, default = "null")]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_records {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 21i16, added = 0i16, removed = 2i16)]
        pub struct DeleteRecordsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each topic that we wanted to delete records from.
            #[kafka(added = 0i16)]
            pub topics: Vec<DeleteRecordsTopicResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteRecordsPartitionResult {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The partition low water mark.
            #[kafka(added = 0i16)]
            pub low_watermark: krost::primitive::Int64,
            ///The deletion error code, or 0 if the deletion succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteRecordsTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition that we wanted to delete records from.
            #[kafka(added = 0i16)]
            pub partitions: Vec<DeleteRecordsPartitionResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod init_producer_id {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 22i16, added = 0i16, removed = 4i16)]
        pub struct InitProducerIdResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The current producer id.
            #[kafka(added = 0i16, default = -1f64)]
            pub producer_id: krost::primitive::Int64,
            ///The current epoch associated with the producer id.
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_for_leader_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 23i16, added = 0i16, removed = 4i16)]
        pub struct OffsetForLeaderEpochResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 2i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each topic we fetched offsets for.
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetForLeaderTopicResult>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EpochEndOffset {
            ///The error code 0, or if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition: krost::primitive::Int32,
            ///The leader epoch of the partition.
            #[kafka(added = 1i16, default = "-1")]
            pub leader_epoch: krost::primitive::Int32,
            ///The end offset of the epoch.
            #[kafka(added = 0i16, default = "-1")]
            pub end_offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetForLeaderTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            ///Each partition in the topic we fetched offsets for.
            #[kafka(added = 0i16)]
            pub partitions: Vec<EpochEndOffset>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod add_partitions_to_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 24i16, added = 0i16, removed = 3i16)]
        pub struct AddPartitionsToTxnResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each topic.
            #[kafka(added = 0i16)]
            pub results: Vec<AddPartitionsToTxnTopicResult>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AddPartitionsToTxnPartitionResult {
            ///The partition indexes.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The response error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AddPartitionsToTxnTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The results for each partition
            #[kafka(added = 0i16)]
            pub results: Vec<AddPartitionsToTxnPartitionResult>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod add_offsets_to_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 25i16, added = 0i16, removed = 3i16)]
        pub struct AddOffsetsToTxnResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The response error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod end_txn {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 26i16, added = 0i16, removed = 3i16)]
        pub struct EndTxnResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod write_txn_markers {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 27i16, added = 0i16, removed = 1i16)]
        pub struct WriteTxnMarkersResponse {
            ///The results for writing makers.
            #[kafka(added = 0i16)]
            pub markers: Vec<WritableTxnMarkerResult>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct WritableTxnMarkerPartitionResult {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct WritableTxnMarkerTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The results by partition.
            #[kafka(added = 0i16)]
            pub partitions: Vec<WritableTxnMarkerPartitionResult>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct WritableTxnMarkerResult {
            ///The current producer ID in use by the transactional ID.
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///The results by topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<WritableTxnMarkerTopicResult>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod txn_offset_commit {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 28i16, added = 0i16, removed = 3i16)]
        pub struct TxnOffsetCommitResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses for each topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<TxnOffsetCommitResponseTopic>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TxnOffsetCommitResponsePartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TxnOffsetCommitResponseTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The responses for each partition in the topic.
            #[kafka(added = 0i16)]
            pub partitions: Vec<TxnOffsetCommitResponsePartition>,
            ///The tagged fields.
            #[kafka(added = 3i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 29i16, added = 0i16, removed = 2i16)]
        pub struct DescribeAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///Each Resource that is referenced in an ACL.
            #[kafka(added = 0i16)]
            pub resources: Vec<DescribeAclsResource>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AclDescription {
            ///The ACL principal.
            #[kafka(added = 0i16)]
            pub principal: krost::primitive::String,
            ///The ACL host.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The ACL operation.
            #[kafka(added = 0i16)]
            pub operation: krost::primitive::Int8,
            ///The ACL permission type.
            #[kafka(added = 0i16)]
            pub permission_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeAclsResource {
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The resource pattern type.
            #[kafka(added = 1i16, default = "3")]
            pub pattern_type: krost::primitive::Int8,
            ///The ACLs.
            #[kafka(added = 0i16)]
            pub acls: Vec<AclDescription>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 30i16, added = 0i16, removed = 2i16)]
        pub struct CreateAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each ACL creation.
            #[kafka(added = 0i16)]
            pub results: Vec<AclCreationResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AclCreationResult {
            ///The result error, or zero if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The result message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_acls {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 31i16, added = 0i16, removed = 2i16)]
        pub struct DeleteAclsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each filter.
            #[kafka(added = 0i16)]
            pub filter_results: Vec<DeleteAclsFilterResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteAclsMatchingAcl {
            ///The deletion error code, or 0 if the deletion succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The deletion error message, or null if the deletion succeeded.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The ACL resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The ACL resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The ACL resource pattern type.
            #[kafka(added = 1i16, default = "3")]
            pub pattern_type: krost::primitive::Int8,
            ///The ACL principal.
            #[kafka(added = 0i16)]
            pub principal: krost::primitive::String,
            ///The ACL host.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The ACL operation.
            #[kafka(added = 0i16)]
            pub operation: krost::primitive::Int8,
            ///The ACL permission type.
            #[kafka(added = 0i16)]
            pub permission_type: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeleteAclsFilterResult {
            ///The error code, or 0 if the filter succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if the filter succeeded.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The ACLs which matched this filter.
            #[kafka(added = 0i16)]
            pub matching_acls: Vec<DeleteAclsMatchingAcl>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 32i16, added = 0i16, removed = 4i16)]
        pub struct DescribeConfigsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each resource.
            #[kafka(added = 0i16)]
            pub results: Vec<DescribeConfigsResult>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeConfigsSynonym {
            ///The synonym name.
            #[kafka(added = 1i16)]
            pub name: krost::primitive::String,
            ///The synonym value.
            #[kafka(added = 1i16)]
            pub value: Option<krost::primitive::String>,
            ///The synonym source.
            #[kafka(added = 1i16)]
            pub source: krost::primitive::Int8,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeConfigsResourceResult {
            ///The configuration name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The configuration value.
            #[kafka(added = 0i16)]
            pub value: Option<krost::primitive::String>,
            ///True if the configuration is read-only.
            #[kafka(added = 0i16)]
            pub read_only: krost::primitive::Bool,
            ///True if the configuration is not set.
            #[kafka(added = 0i16)]
            pub is_default: krost::primitive::Bool,
            ///The configuration source.
            #[kafka(added = 1i16, default = "-1")]
            pub config_source: krost::primitive::Int8,
            ///True if this configuration is sensitive.
            #[kafka(added = 0i16)]
            pub is_sensitive: krost::primitive::Bool,
            ///The synonyms for this configuration key.
            #[kafka(added = 1i16)]
            pub synonyms: Vec<DescribeConfigsSynonym>,
            ///The configuration data type. Type can be one of the following values - BOOLEAN, STRING, INT, SHORT, LONG, DOUBLE, LIST, CLASS, PASSWORD
            #[kafka(added = 3i16, default = "0")]
            pub config_type: krost::primitive::Int8,
            ///The configuration documentation.
            #[kafka(added = 3i16)]
            pub documentation: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeConfigsResult {
            ///The error code, or 0 if we were able to successfully describe the configurations.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if we were able to successfully describe the configurations.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///Each listed configuration.
            #[kafka(added = 0i16)]
            pub configs: Vec<DescribeConfigsResourceResult>,
            ///The tagged fields.
            #[kafka(added = 4i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 33i16, added = 0i16, removed = 2i16)]
        pub struct AlterConfigsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses for each resource.
            #[kafka(added = 0i16)]
            pub responses: Vec<AlterConfigsResourceResponse>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterConfigsResourceResponse {
            ///The resource error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The resource error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_replica_log_dirs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 34i16, added = 0i16, removed = 2i16)]
        pub struct AlterReplicaLogDirsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for each topic.
            #[kafka(added = 0i16)]
            pub results: Vec<AlterReplicaLogDirTopicResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterReplicaLogDirPartitionResult {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterReplicaLogDirTopicResult {
            ///The name of the topic.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            ///The results for each partition.
            #[kafka(added = 0i16)]
            pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_log_dirs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 35i16, added = 0i16, removed = 3i16)]
        pub struct DescribeLogDirsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 3i16)]
            pub error_code: krost::primitive::Int16,
            ///The log directories.
            #[kafka(added = 0i16)]
            pub results: Vec<DescribeLogDirsResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeLogDirsPartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The size of the log segments in this partition in bytes.
            #[kafka(added = 0i16)]
            pub partition_size: krost::primitive::Int64,
            ///The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)
            #[kafka(added = 0i16)]
            pub offset_lag: krost::primitive::Int64,
            ///True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
            #[kafka(added = 0i16)]
            pub is_future_key: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeLogDirsTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<DescribeLogDirsPartition>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeLogDirsResult {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The absolute log directory path.
            #[kafka(added = 0i16)]
            pub log_dir: krost::primitive::String,
            ///Each topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<DescribeLogDirsTopic>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod sasl_authenticate {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 36i16, added = 0i16, removed = 2i16)]
        pub struct SaslAuthenticateResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The SASL authentication bytes from the server, as defined by the SASL mechanism.
            #[kafka(added = 0i16)]
            pub auth_bytes: Vec<u8>,
            ///The SASL authentication bytes from the server, as defined by the SASL mechanism.
            #[kafka(added = 1i16, default = "0")]
            pub session_lifetime_ms: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_partitions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 37i16, added = 0i16, removed = 3i16)]
        pub struct CreatePartitionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The partition creation results for each topic.
            #[kafka(added = 0i16)]
            pub results: Vec<CreatePartitionsTopicResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CreatePartitionsTopicResult {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The result error, or zero if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The result message, or null if there was no error.
            #[kafka(added = 0i16, default = "null")]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod create_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 38i16, added = 0i16, removed = 2i16)]
        pub struct CreateDelegationTokenResponse {
            ///The top-level error, or zero if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The principal type of the token owner.
            #[kafka(added = 0i16)]
            pub principal_type: krost::primitive::String,
            ///The name of the token owner.
            #[kafka(added = 0i16)]
            pub principal_name: krost::primitive::String,
            ///When this token was generated.
            #[kafka(added = 0i16)]
            pub issue_timestamp_ms: krost::primitive::Int64,
            ///When this token expires.
            #[kafka(added = 0i16)]
            pub expiry_timestamp_ms: krost::primitive::Int64,
            ///The maximum lifetime of this token.
            #[kafka(added = 0i16)]
            pub max_timestamp_ms: krost::primitive::Int64,
            ///The token UUID.
            #[kafka(added = 0i16)]
            pub token_id: krost::primitive::String,
            ///HMAC of the delegation token.
            #[kafka(added = 0i16)]
            pub hmac: Vec<u8>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod renew_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 39i16, added = 0i16, removed = 2i16)]
        pub struct RenewDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The timestamp in milliseconds at which this token expires.
            #[kafka(added = 0i16)]
            pub expiry_timestamp_ms: krost::primitive::Int64,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod expire_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 40i16, added = 0i16, removed = 2i16)]
        pub struct ExpireDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The timestamp in milliseconds at which this token expires.
            #[kafka(added = 0i16)]
            pub expiry_timestamp_ms: krost::primitive::Int64,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_delegation_token {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 41i16, added = 0i16, removed = 2i16)]
        pub struct DescribeDelegationTokenResponse {
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tokens.
            #[kafka(added = 0i16)]
            pub tokens: Vec<DescribedDelegationToken>,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribedDelegationTokenRenewer {
            ///The renewer principal type
            #[kafka(added = 0i16)]
            pub principal_type: krost::primitive::String,
            ///The renewer principal name
            #[kafka(added = 0i16)]
            pub principal_name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribedDelegationToken {
            ///The token principal type.
            #[kafka(added = 0i16)]
            pub principal_type: krost::primitive::String,
            ///The token principal name.
            #[kafka(added = 0i16)]
            pub principal_name: krost::primitive::String,
            ///The token issue timestamp in milliseconds.
            #[kafka(added = 0i16)]
            pub issue_timestamp: krost::primitive::Int64,
            ///The token expiry timestamp in milliseconds.
            #[kafka(added = 0i16)]
            pub expiry_timestamp: krost::primitive::Int64,
            ///The token maximum timestamp length in milliseconds.
            #[kafka(added = 0i16)]
            pub max_timestamp: krost::primitive::Int64,
            ///The token ID.
            #[kafka(added = 0i16)]
            pub token_id: krost::primitive::String,
            ///The token HMAC.
            #[kafka(added = 0i16)]
            pub hmac: Vec<u8>,
            ///Those who are able to renew this token before it expires.
            #[kafka(added = 0i16)]
            pub renewers: Vec<DescribedDelegationTokenRenewer>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod delete_groups {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 42i16, added = 0i16, removed = 2i16)]
        pub struct DeleteGroupsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The deletion results
            #[kafka(added = 0i16)]
            pub results: Vec<DeletableGroupResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DeletableGroupResult {
            ///The group id
            #[kafka(added = 0i16)]
            pub group_id: krost::primitive::String,
            ///The deletion error, or 0 if the deletion succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod elect_leaders {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 43i16, added = 0i16, removed = 2i16)]
        pub struct ElectLeadersResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top level response error code.
            #[kafka(added = 1i16)]
            pub error_code: krost::primitive::Int16,
            ///The election results, or an empty array if the requester did not have permission and the request asks for all partitions.
            #[kafka(added = 0i16)]
            pub replica_election_results: Vec<ReplicaElectionResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionResult {
            ///The partition id
            #[kafka(added = 0i16)]
            pub partition_id: krost::primitive::Int32,
            ///The result error, or zero if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The result message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ReplicaElectionResult {
            ///The topic name
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            ///The results for each partition
            #[kafka(added = 0i16)]
            pub partition_result: Vec<PartitionResult>,
            ///The tagged fields.
            #[kafka(added = 2i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod incremental_alter_configs {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 44i16, added = 0i16, removed = 1i16)]
        pub struct IncrementalAlterConfigsResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses for each resource.
            #[kafka(added = 0i16)]
            pub responses: Vec<AlterConfigsResourceResponse>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterConfigsResourceResponse {
            ///The resource error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The resource error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The resource type.
            #[kafka(added = 0i16)]
            pub resource_type: krost::primitive::Int8,
            ///The resource name.
            #[kafka(added = 0i16)]
            pub resource_name: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_partition_reassignments {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 45i16, added = 0i16)]
        pub struct AlterPartitionReassignmentsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top-level error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The responses to topics to reassign.
            #[kafka(added = 0i16)]
            pub responses: Vec<ReassignableTopicResponse>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ReassignablePartitionResponse {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code for this partition, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message for this partition, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ReassignableTopicResponse {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The responses to partitions to reassign
            #[kafka(added = 0i16)]
            pub partitions: Vec<ReassignablePartitionResponse>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_partition_reassignments {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 46i16, added = 0i16)]
        pub struct ListPartitionReassignmentsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top-level error code, or 0 if there was no error
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The ongoing reassignments for each topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<OngoingTopicReassignment>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OngoingPartitionReassignment {
            ///The index of the partition.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The current replica set.
            #[kafka(added = 0i16)]
            pub replicas: Vec<krost::primitive::Int32>,
            ///The set of replicas we are currently adding.
            #[kafka(added = 0i16)]
            pub adding_replicas: Vec<krost::primitive::Int32>,
            ///The set of replicas we are currently removing.
            #[kafka(added = 0i16)]
            pub removing_replicas: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OngoingTopicReassignment {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The ongoing reassignments for each partition.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OngoingPartitionReassignment>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod offset_delete {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 47i16, added = 0i16)]
        pub struct OffsetDeleteResponse {
            ///The top-level error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The responses for each topic.
            #[kafka(added = 0i16)]
            pub topics: Vec<OffsetDeleteResponseTopic>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetDeleteResponsePartition {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct OffsetDeleteResponseTopic {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The responses for each partition in the topic.
            #[kafka(added = 0i16)]
            pub partitions: Vec<OffsetDeleteResponsePartition>,
        }
    }
    pub mod describe_client_quotas {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 48i16, added = 0i16, removed = 1i16)]
        pub struct DescribeClientQuotasResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or `0` if the quota description succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or `null` if the quota description succeeded.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///A result entry.
            #[kafka(added = 0i16)]
            pub entries: Option<Vec<EntryData>>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(added = 0i16)]
            pub entity_type: krost::primitive::String,
            ///The entity name, or null if the default.
            #[kafka(added = 0i16)]
            pub entity_name: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ValueData {
            ///The quota configuration key.
            #[kafka(added = 0i16)]
            pub key: krost::primitive::String,
            ///The quota configuration value.
            #[kafka(added = 0i16)]
            pub value: float64,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntryData {
            ///The quota entity description.
            #[kafka(added = 0i16)]
            pub entity: Vec<EntityData>,
            ///The quota values for the entity.
            #[kafka(added = 0i16)]
            pub values: Vec<ValueData>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_client_quotas {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 49i16, added = 0i16, removed = 1i16)]
        pub struct AlterClientQuotasResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The quota configuration entries to alter.
            #[kafka(added = 0i16)]
            pub entries: Vec<EntryData>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntityData {
            ///The entity type.
            #[kafka(added = 0i16)]
            pub entity_type: krost::primitive::String,
            ///The name of the entity, or null if the default.
            #[kafka(added = 0i16)]
            pub entity_name: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct EntryData {
            ///The error code, or `0` if the quota alteration succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, or `null` if the quota alteration succeeded.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The quota entity to alter.
            #[kafka(added = 0i16)]
            pub entity: Vec<EntityData>,
            ///The tagged fields.
            #[kafka(added = 1i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_user_scram_credentials {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 50i16, added = 0i16)]
        pub struct DescribeUserScramCredentialsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The message-level error code, 0 except for user authorization or infrastructure issues.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The message-level error message, if any.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The results for descriptions, one per user.
            #[kafka(added = 0i16)]
            pub results: Vec<DescribeUserScramCredentialsResult>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct CredentialInfo {
            ///The SCRAM mechanism.
            #[kafka(added = 0i16)]
            pub mechanism: krost::primitive::Int8,
            ///The number of iterations used in the SCRAM credential.
            #[kafka(added = 0i16)]
            pub iterations: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeUserScramCredentialsResult {
            ///The user name.
            #[kafka(added = 0i16)]
            pub user: krost::primitive::String,
            ///The user-level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The user-level error message, if any.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The mechanism and related information associated with the user's SCRAM credentials.
            #[kafka(added = 0i16)]
            pub credential_infos: Vec<CredentialInfo>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_user_scram_credentials {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 51i16, added = 0i16)]
        pub struct AlterUserScramCredentialsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The results for deletions and alterations, one per affected user.
            #[kafka(added = 0i16)]
            pub results: Vec<AlterUserScramCredentialsResult>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct AlterUserScramCredentialsResult {
            ///The user name.
            #[kafka(added = 0i16)]
            pub user: krost::primitive::String,
            ///The error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The error message, if any.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod vote {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 52i16, added = 0i16)]
        pub struct VoteResponse {
            ///The top level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///True if the vote was granted and false otherwise
            #[kafka(added = 0i16)]
            pub vote_granted: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod begin_quorum_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 53i16, added = 0i16)]
        pub struct BeginQuorumEpochResponse {
            ///The top level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
        }
    }
    pub mod end_quorum_epoch {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 54i16, added = 0i16)]
        pub struct EndQuorumEpochResponse {
            ///The top level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
        }
    }
    pub mod describe_quorum {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 55i16, added = 0i16)]
        pub struct DescribeQuorumResponse {
            ///The top level error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub high_watermark: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub current_voters: Vec<ReplicaState>,
            #[kafka(added = 0i16)]
            pub observers: Vec<ReplicaState>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The topic name.
            #[kafka(added = 0i16)]
            pub topic_name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod alter_partition {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 56i16, added = 0i16, removed = 1i16)]
        pub struct AlterPartitionResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top level response error code
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionData {
            ///The partition index
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The partition level error code
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The broker ID of the leader.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The leader epoch.
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///The in-sync replica IDs.
            #[kafka(added = 0i16)]
            pub isr: Vec<krost::primitive::Int32>,
            ///1 if the partition is recovering from an unclean leader election; 0 otherwise.
            #[kafka(added = 1i16, default = "0")]
            pub leader_recovery_state: krost::primitive::Int8,
            ///The current epoch for the partition for KRaft controllers. The current ZK version for the legacy controllers.
            #[kafka(added = 0i16)]
            pub partition_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            ///The name of the topic
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod update_features {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 57i16, added = 0i16, removed = 1i16)]
        pub struct UpdateFeaturesResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top-level error code, or `0` if there was no top-level error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The top-level error message, or `null` if there was no top-level error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///Results for each feature update.
            #[kafka(added = 0i16)]
            pub results: Vec<UpdatableFeatureResult>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct UpdatableFeatureResult {
            ///The name of the finalized feature.
            #[kafka(added = 0i16)]
            pub feature: krost::primitive::String,
            ///The feature update error code or `0` if the feature update succeeded.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The feature update error, or `null` if the feature update succeeded.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod envelope {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 58i16, added = 0i16)]
        pub struct EnvelopeResponse {
            ///The embedded response header and data.
            #[kafka(added = 0i16, default = "null")]
            pub response_data: Option<Vec<u8>>,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod fetch_snapshot {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 59i16, added = 0i16)]
        pub struct FetchSnapshotResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top level response error code.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The topics to fetch.
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicSnapshot>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct SnapshotId {
            #[kafka(added = 0i16)]
            pub end_offset: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct LeaderIdAndEpoch {
            ///The ID of the current leader or -1 if the leader is unknown.
            #[kafka(added = 0i16)]
            pub leader_id: krost::primitive::Int32,
            ///The latest known leader epoch
            #[kafka(added = 0i16)]
            pub leader_epoch: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionSnapshot {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub index: krost::primitive::Int32,
            ///The error code, or 0 if there was no fetch error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The snapshot endOffset and epoch fetched
            #[kafka(added = 0i16)]
            pub snapshot_id: SnapshotId,
            #[kafka(added = 0i16)]
            pub current_leader: LeaderIdAndEpoch,
            ///The total size of the snapshot.
            #[kafka(added = 0i16)]
            pub size: krost::primitive::Int64,
            ///The starting byte position within the snapshot included in the Bytes field.
            #[kafka(added = 0i16)]
            pub position: krost::primitive::Int64,
            ///Snapshot data in records format which may not be aligned on an offset boundary
            #[kafka(added = 0i16)]
            pub unaligned_records: krost::record::RecordBatch,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicSnapshot {
            ///The name of the topic to fetch.
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///The partitions to fetch.
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionSnapshot>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_cluster {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 60i16, added = 0i16)]
        pub struct DescribeClusterResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top-level error code, or 0 if there was no error
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The top-level error message, or null if there was no error.
            #[kafka(added = 0i16, default = "null")]
            pub error_message: Option<krost::primitive::String>,
            ///The cluster ID that responding broker belongs to.
            #[kafka(added = 0i16)]
            pub cluster_id: krost::primitive::String,
            ///The ID of the controller broker.
            #[kafka(added = 0i16, default = "-1")]
            pub controller_id: krost::primitive::Int32,
            ///Each broker in the response.
            #[kafka(added = 0i16)]
            pub brokers: Vec<DescribeClusterBroker>,
            ///32-bit bitfield to represent authorized operations for this cluster.
            #[kafka(added = 0i16, default = "-2147483648")]
            pub cluster_authorized_operations: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct DescribeClusterBroker {
            ///The broker ID.
            #[kafka(added = 0i16)]
            pub broker_id: krost::primitive::Int32,
            ///The broker hostname.
            #[kafka(added = 0i16)]
            pub host: krost::primitive::String,
            ///The broker port.
            #[kafka(added = 0i16)]
            pub port: krost::primitive::Int32,
            ///The rack of the broker, or null if it has not been assigned to a rack.
            #[kafka(added = 0i16, default = "null")]
            pub rack: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_producers {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 61i16, added = 0i16)]
        pub struct DescribeProducersResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///Each topic in the response.
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicResponse>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct ProducerState {
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int32,
            #[kafka(added = 0i16, default = "-1")]
            pub last_sequence: krost::primitive::Int32,
            #[kafka(added = 0i16, default = "-1")]
            pub last_timestamp: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub coordinator_epoch: krost::primitive::Int32,
            #[kafka(added = 0i16, default = "-1")]
            pub current_txn_start_offset: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct PartitionResponse {
            ///The partition index.
            #[kafka(added = 0i16)]
            pub partition_index: krost::primitive::Int32,
            ///The partition error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The partition error message, which may be null if no additional details are available
            #[kafka(added = 0i16, default = "null")]
            pub error_message: Option<krost::primitive::String>,
            #[kafka(added = 0i16)]
            pub active_producers: Vec<ProducerState>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicResponse {
            ///The topic name
            #[kafka(added = 0i16)]
            pub name: krost::primitive::String,
            ///Each partition in the response.
            #[kafka(added = 0i16)]
            pub partitions: Vec<PartitionResponse>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod broker_registration {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 62i16, added = 0i16)]
        pub struct BrokerRegistrationResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The broker's assigned epoch, or -1 if none was assigned.
            #[kafka(added = 0i16, default = "-1")]
            pub broker_epoch: krost::primitive::Int64,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod broker_heartbeat {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 63i16, added = 0i16)]
        pub struct BrokerHeartbeatResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///True if the broker has approximately caught up with the latest metadata.
            #[kafka(added = 0i16, default = "false")]
            pub is_caught_up: krost::primitive::Bool,
            ///True if the broker is fenced.
            #[kafka(added = 0i16, default = "true")]
            pub is_fenced: krost::primitive::Bool,
            ///True if the broker should proceed with its shutdown.
            #[kafka(added = 0i16)]
            pub should_shut_down: krost::primitive::Bool,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod unregister_broker {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 64i16, added = 0i16)]
        pub struct UnregisterBrokerResponse {
            ///Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The error code, or 0 if there was no error.
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The top-level error message, or `null` if there was no top-level error.
            #[kafka(added = 0i16)]
            pub error_message: Option<krost::primitive::String>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod describe_transactions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 65i16, added = 0i16)]
        pub struct DescribeTransactionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub transaction_states: Vec<TransactionState>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TopicData {
            #[kafka(added = 0i16)]
            pub topic: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub partitions: Vec<krost::primitive::Int32>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TransactionState {
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub transaction_state: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub transaction_timeout_ms: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub transaction_start_time_ms: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            #[kafka(added = 0i16)]
            pub producer_epoch: krost::primitive::Int16,
            ///The set of partitions included in the current transaction (if active). When a transaction is preparing to commit or abort, this will include only partitions which do not have markers.
            #[kafka(added = 0i16)]
            pub topics: Vec<TopicData>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod list_transactions {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 66i16, added = 0i16)]
        pub struct ListTransactionsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///Set of state filters provided in the request which were unknown to the transaction coordinator
            #[kafka(added = 0i16)]
            pub unknown_state_filters: Vec<krost::primitive::String>,
            #[kafka(added = 0i16)]
            pub transaction_states: Vec<TransactionState>,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
        #[derive(Debug, PartialEq, Krost, Clone)]
        pub struct TransactionState {
            #[kafka(added = 0i16)]
            pub transactional_id: krost::primitive::String,
            #[kafka(added = 0i16)]
            pub producer_id: krost::primitive::Int64,
            ///The current transaction state of the producer
            #[kafka(added = 0i16)]
            pub transaction_state: krost::primitive::String,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
        }
    }
    pub mod allocate_producer_ids {
        #[derive(Debug, PartialEq, Krost, Clone)]
        #[kafka(apikey = 67i16, added = 0i16)]
        pub struct AllocateProducerIdsResponse {
            ///The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
            #[kafka(added = 0i16)]
            pub throttle_time_ms: krost::primitive::Int32,
            ///The top level response error code
            #[kafka(added = 0i16)]
            pub error_code: krost::primitive::Int16,
            ///The first producer ID in this range, inclusive
            #[kafka(added = 0i16)]
            pub producer_id_start: krost::primitive::Int64,
            ///The number of producer IDs in this range
            #[kafka(added = 0i16)]
            pub producer_id_len: krost::primitive::Int32,
            ///The tagged fields.
            #[kafka(added = 0i16)]
            pub _tagged_fields: krost::primitive::TaggedFields,
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
