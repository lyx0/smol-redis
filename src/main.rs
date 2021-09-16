use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
  /*
   * client::connect asynchronously establishes a TCP connection
   * for the specified address.
   * Afterwards a client handle is returned.
   * The only indication that it is asynchronous is the
   * .await operator.
   */
  let mut client = client::connect("127.0.0.1:6379").await?;

  // Set a new key "hello" with value "world"
  client.set("hello", "world".into()).await?;

  // Get key "hello"
  let result = client.get("hello").await?;

  println!("got value from the server; result={:?}", result);

  Ok(())
}
