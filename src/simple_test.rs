use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::geyser::SubscribeRequest;

pub async fn simple_geyser_test() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to gRPC endpoint
    let channel = Channel::from_static("http://localhost:10000").connect().await?;
    let mut client = GeyserClient::new(channel);

    // Create a request with 55+ pubkeys
    let pubkeys = vec![
        "11111111111111111111111111111111111111112",
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        // ... add more pubkeys here to exceed 50
        // DEX Programs
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Raydium AMM
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc", // Orca
        "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM", // Serum DEX
        "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB",  // Jupiter
        "CAMMCzo5YL8w4VFF8KVHrK22GGUQpMkFR6nACcp6fDyA", // Raydium CLMM
        "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt", // Lifinity
        "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S", // Lifinity V2
        
        // Major Tokens
        "So11111111111111111111111111111111111111112",  // Wrapped SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB", // USDT
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",  // Marinade SOL
        "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj", // Lido SOL
        "9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E", // Bonk
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // Bonk (old)
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn", // JitoSOL
        "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1",  // BlazeStake SOL
        "HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3", // Pyth
        "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr", // Dogwifhat
        "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk", // Helium
        
        // Pool Tokens & LP Tokens
        "8HGyAAB1yoM1ttS7pXjHMa3dukTFGQggnFFH3hJZgzQh", // COPE
        "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R", // Raydium
        "MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey",  // Marinade
        "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt",  // Serum
        "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE",  // Orca
        "kinXdEcpDQeHPEuQnqmUgtYykqKGVFq6CeVX5iAHJq6",  // Kin
        "MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5",  // Cat in a dogs world
        "WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk",   // WEN
        "ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82",  // Bonfida
        
        // Raydium Pool Addresses
        "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2", // SOL-USDC
        "7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCUWBkwMihLYQo2", // SOL-USDT
        "6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg", // USDC-USDT
        "AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA", // RAY-SOL
        "2RoucD8CjTF6pd8a2cLRB7ZAcXPnGJJwDjvNwQ7yD8WJ", // RAY-USDC
        "F3kYuEwkXJPCKaUFqrxu1J1fxSKqBjNaFyNbP5gXzwYP", // Another pool
        
        // Orca Pool Addresses
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP", // ORCA-SOL
        "2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY", // ORCA-USDC
        "83v8iPyZihDEjDdY8RdZddyZNyUtXngz69Lgo9Kt5d6d", // SOL-USDC (Orca)
        
        // More DEX Programs
        "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",  // Serum DEX V3
        "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o", // Serum DEX V4
        "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1", // Orca V2
        "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8",  // Saber
        "Crt7UoUR6QgrFrN7j8rmSQpUTNWtSomPdLA8gF8fJBHi", // Crema
        "82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ", // Aldrin
        "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6", // Mercurial
        "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky",  // Mercurial V2
        
        // Metaplex and NFT Programs  
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",  // Metaplex
        "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98",  // Metaplex Auction
        "hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk", // Metaplex Auction House
        
        // Oracle Programs
        "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH", // Pyth Oracle
        "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s",  // Switchboard
        
        // Lending Protocols
        "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo",  // Solend
        "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi",   // Port Finance
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD",   // Marginfi
    ];

    // This is a minimal request that should trigger the error
    let request = tonic::Request::new(SubscribeRequest::default());
    
    match client.subscribe(request).await {
        Ok(_) => println!("Subscription worked"),
        Err(e) => {
            println!("Error: {:?}", e);
            if e.message().contains("Max amount of Pubkeys reached, only 50 allowed") {
                println!("🎯 Found the exact error!");
            }
        }
    }

    Ok(())
}
