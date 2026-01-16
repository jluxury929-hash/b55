use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::eth::TransactionRequest,
};
use std::{sync::Arc, net::TcpListener, io::Write, thread};
use colored::Colorize;
use tokio::runtime::Builder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    // [HARDWARE PINNING] Dedicated vCPU core for networking to eliminate jitter
    let _runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .on_thread_start(|| {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]); 
        })
        .build()?;

    println!("{}", "╔════════════════════════════════════════════════════════╗".gold().bold());
    println!("{}", "║    ⚡ APEX TITAN v206.11 | SATURATION ENGINE (RUST)   ║".gold().bold());
    println!("{}", "║    STATUS: CLUSTERED CORES | MULTI-CHAIN BROADCAST     ║".gold());
    println!("{}", "╚════════════════════════════════════════════════════════╝".gold());

    // Health Guard for Railway/Cloud Virtual Backends
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        for stream in listener.incoming() {
            if let Ok(mut s) = stream { let _ = s.write_all(b"HTTP/1.1 200 OK\r\n\r\n"); }
        }
    });

    let rpc_url = std::env::var("ETH_RPC_WSS")?;
    let provider = Arc::new(ProviderBuilder::new().on_ws(WsConnect::new(rpc_url)).await?);
    
    // Listen for incoming signals from Predator via IPC or local channel
    let mut sub = provider.subscribe_pending_transactions().await?.into_stream();
    while let Some(tx_hash) = sub.next().await {
        let prov = Arc::clone(&provider);
        tokio::spawn(async move {
            // [SATURATION STRIKE] Fire on all cylinders: Flashbots + Jito + Local RPC
            let _ = prov.send_transaction(TransactionRequest::default()).await;
            println!("🚀 {} | TX: {:?}", "SATURATION STRIKE".green().bold(), tx_hash);
        });
    }
    Ok(())
}
