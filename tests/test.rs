use whois_rust::*;

#[test]
fn test() {
    let who = WhoIs::from_path("node-whois/servers.json").unwrap();

    let result = who.lookup(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).unwrap();
    println!("{}", String::from_utf8(result).unwrap());

    let result = who.lookup(WhoIsLookupOptions::from_string("172.105.210.153").unwrap()).unwrap();
    println!("{}", String::from_utf8(result).unwrap());
}

#[test]
fn test_srv() {
    let mut who = WhoIs::from_host("whois.arin.net").unwrap();

    assert!(who.can_find_server_for_tld(".lotteryusa.us", "8.8.8.8:53"));

    let result = who.lookup(WhoIsLookupOptions::from_string("lotteryusa.us").unwrap()).unwrap();
    println!("{}", String::from_utf8(result).unwrap());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn test_async() {
    let who = WhoIs::from_path_async("node-whois/servers.json").await.unwrap();

    let result =
        who.lookup_async(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).await.unwrap();
    println!("{}", String::from_utf8(result).unwrap());

    let result = who
        .lookup_async(WhoIsLookupOptions::from_string("172.105.210.153").unwrap())
        .await
        .unwrap();
    println!("{}", String::from_utf8(result).unwrap());

    // Contains non-UTF8 chars.
    let result =
        who.lookup_async(WhoIsLookupOptions::from_string("simeon.com.br").unwrap()).await.unwrap();

    println!("{}", String::from_utf8_lossy(&result));
}
