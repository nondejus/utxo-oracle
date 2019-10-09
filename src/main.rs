extern crate bitcoin;
extern crate bitcoincore_rpc;

use std::env;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};

fn main_result() -> Result<(), Error> {
    println!("begin!");
    let url = env::var("RPC_URL").expect("RPC_URL");
    let user = "user".to_string();
    let pass = env::var("RPC_PASS").expect("RPC_PASS");
    let rpc = Client::new(url, Auth::UserPass(user, pass))?;
    let hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", hash);
    let block = rpc.get_block(&hash)?;
    println!("got block");
    let txdata = &block.txdata;
    for tx in txdata {
        for out in &tx.output {
            print!("{} ", out.value);
        }
        println!();
    }

    println!("{} txs", txdata.len());

    return Ok(())
}

fn main() {
    main_result().unwrap();
}
