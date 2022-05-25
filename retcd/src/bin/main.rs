use retcd::proto::etcdserverpb::kv_client::KvClient;
use retcd::proto::etcdserverpb::PutRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvClient::connect("http://127.0.0.1:2379").await?;

    let request = tonic::Request::new(PutRequest {
        key: "retcd".as_bytes().to_vec(),
        value: "retcd".as_bytes().to_vec(),
        lease: 0,
        prev_kv: false,
        ignore_value: false,
        ignore_lease: false,
    });

    let response = client.put(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
