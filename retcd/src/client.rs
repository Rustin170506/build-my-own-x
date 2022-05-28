use super::proto::etcdserverpb::kv_client::KvClient;
use super::proto::etcdserverpb::PutRequest;
use tonic::transport::Channel;

pub struct Client {
    kv_client: KvClient<Channel>,
}

impl Client {
    
}