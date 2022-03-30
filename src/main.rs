mod libp2p_chat;

use libp2p::{identity, Multiaddr, PeerId, Swarm};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::SwarmEvent;
use std::error::Error;
use clap::{Arg, App, SubCommand};
use futures::StreamExt;
use shell_command;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rust-solana-cli")
        .version("0.1.0")
        .author("Soushi888 <sacha.pignot@protonmail.com>")
        .about("Create a Solana Toekn (SPL) on the Solana Testnet and disable the minting.")
        .subcommand(SubCommand::with_name("create-token")
            .about("Create a Solana Toekn (SPL) on the Solana Testnet")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .required(true)
                .takes_value(true)
                .help("Name of the Token")
            )
        )
        .subcommand(SubCommand::with_name("show-account")
            .about("Show all the tokens of the connected account")
        )
        .subcommand(SubCommand::with_name("connect")
            .about("Make a libp2p connection that ping the address in argument")
            .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                .required(true)
                .takes_value(true)
                .help("Address to connect")))
        .subcommand(SubCommand::with_name("chat")
            .about("Launch a libp2p chat client")
            .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                // .required(true)
                .takes_value(true)
                .help("Address to connect")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-token") {
        create_token(matches.value_of("name").unwrap().to_string());
    } else if let Some(_matches) = matches.subcommand_matches("show-account") {
        let show_account_command = shell_command::run_shell_command("spl-token accounts").unwrap();
        println!("Account : {}", show_account_command);
    } else if let Some(matches) = matches.subcommand_matches("connect") {
        create_libp2p_connection(matches.value_of("address").unwrap().to_string()).await?;
    } else if let Some(matches) = matches.subcommand_matches("chat") {
        let address = match matches.value_of("address") {
            Some(str) => { str.to_string() }
            None => { "".to_string() }
        };
        libp2p_chat::libp2p_chat(address).await?;
    } else {
        println!("rust-solana-cli is a program for creating a Solana Toekn (SPL) on the Solana Testnet \
        and register its name. Use the command `help` for more informations.");
    }

    Ok(())
}

async fn create_libp2p_connection(address: String) -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id : {:?}\n", local_peer_id);

    let transport = libp2p::development_transport(local_key.clone()).await?;
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let remote: Multiaddr = address.parse()?;
    swarm.dial(remote)?;
    println!("Dialed {}", address);

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}

fn create_token(token_name: String) {
    println!("Token name : {:?}\n", token_name);

    let create_token_command = shell_command::run_shell_command("spl-token create-token").unwrap();
    let mut create_token_result = create_token_command.split(" ");
    let created_token_hash = create_token_result.nth(2).unwrap().to_string().replace("\n\nSignature:", "");
    println!("Created Token Hash : {}", created_token_hash);

    let create_account_commande = shell_command::run_shell_command(&*format!("spl-token create-account {}", created_token_hash)).unwrap();
    let mut create_account_result = create_account_commande.split(" ");
    let created_account_hash = create_account_result.nth(2).unwrap().to_string().replace("\n\nSignature:", "");
    println!("Created Account Hash :  {}\n", created_account_hash);

    let mint_tokens_command = shell_command::run_shell_command(&*format!("spl-token mint {} 10000000 {}", created_token_hash, created_account_hash)).unwrap();
    println!("{}", mint_tokens_command);

    let disable_mint_tokens_command = shell_command::run_shell_command(&*format!("spl-token authorize {} mint --disable", created_token_hash)).unwrap();
    println!("{}", disable_mint_tokens_command);

    let show_balance_command = shell_command::run_shell_command(&*format!("spl-token balance {}", created_token_hash)).unwrap();
    println!("Balance : {}", show_balance_command);
}
