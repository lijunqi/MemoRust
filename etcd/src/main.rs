use etcd_client::{Client, Error, GetOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = Client::connect(["localhost:2379"], None).await?;
    //// put kv
    //client.put("foo", "bar", None).await?;
    //// get kv
    //let resp = client.get("greeting", None).await?;
    //if let Some(kv) = resp.kvs().first() {
    //    println!("Get kv: {{{}: {}}}", kv.key_str()?, kv.value_str()?);
    //}

    println!("Get all:");
    let resp = client
        .get("", Some(GetOptions::new().with_all_keys()))
        .await?;
    for kv in resp.kvs() {
        println!("\t{{{}: {}}}", kv.key_str()?, kv.value_str()?);
    }
    println!();

    println!("Done.");
    Ok(())
}
