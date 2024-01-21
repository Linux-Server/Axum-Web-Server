#![allow(unused)]
use anyhow::Result;
use reqwest;
use httpc_test;



#[tokio::test]
async fn http_client()->Result<()>{
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let res = hc.do_get("/").await?; // httpc_test::Response
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;
    // assert_eq!("Hello Sachin", res);
    Ok(())
}