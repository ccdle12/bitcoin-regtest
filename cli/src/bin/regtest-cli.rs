/// The main file containing all the external cli commands for bootstrapping
/// and interfacing with regtest.

extern crate clap;
use clap::{clap_app, ArgMatches};
use std::env;
use std::fs;
use std::process::Command;
use std::str;

/// An environment variable key for the Bitcoin project path.
const BITCOIN_PROJ_PATH: &str = "BITCOIN_PROJ_PATH";
const ALICE_PATH: &str = "REGTEST_ALICE";
const BOB_PATH: &str = "REGTEST_BOB";

/// The paths for the bitcoin cli and bitcoind binaries. Uses the env variable
/// BITCOIN_PROJ_PATH to prefix the destination folder.
const BITCOIN_CLI: &str = "/src/bitcoin-cli";
const BITCOIN_D: &str = "/src/bitcoind";

/// Enum for each Regtest Node.
enum RegtestNode {
    ALICE,
    BOB,
}

fn main() {
    // Capture the user CLI commands.
    let cmd = clap_app!(regtest_cli => 
        (version: "0.1")
        (author: "ccdle12 <chris.coverdale24@gmail.com")
        (about: "Regtest CLI provides a tool to bootstrap a Regtest Network")
        (@subcommand recompile => 
            (about: "Recompiles the Bitcoin Project")
        )
        (@subcommand setupnodes =>
            (about: "Starts the regtest nodes")
        )
        (@subcommand alice =>
            (about: "Run Bitcoin RPC commands on Alice's node")
            (@arg command: -r --rpc +required +takes_value +multiple
                "A Bitcoin RPC Command")
        )
        (@subcommand bob =>
            (about: "Run Bitcoin RPC commands on Bob's node")
            (@arg command: -r --rpc +required +takes_value +multiple
                "A Bitcoin RPC Command")
        )
        (@subcommand resetnetwork=>
            (about: "Reset the regtest network")
        )
    ).get_matches();


    // Handle and execute the subcommands.
    match &cmd.subcommand_name() {
        Some("recompile") => recompile_bitcoin(),
        Some("setupnodes") => run_regtest_nodes(),
        Some("alice") => run_bitcoin_rpc(&cmd, RegtestNode::ALICE),
        Some("bob") => run_bitcoin_rpc(&cmd, RegtestNode::BOB),
        Some("resetnetwork") => reset_network(),
        _ => ()
    }
}

/// Recompiles the Bitcoin project according to the Path set in the environment.
fn recompile_bitcoin() {
    let bitcoin_path = env::var_os(BITCOIN_PROJ_PATH)
                        .expect("Environment variable BITCOIN_PROJ_PATH is empty")
                        .into_string()
                        .unwrap();

    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && make", bitcoin_path))
        .output()
        .map(|process| {
            if !process.status.success() {
                println!("{}", str::from_utf8(&process.stderr).unwrap())
            } else {
                println!("{}", str::from_utf8(&process.stdout).unwrap())
            };
        })
        .unwrap();
}

/// Runs the nodes Alice and Bob in regtest.
fn run_regtest_nodes() {
    let bitcoin_path = env::var_os(BITCOIN_PROJ_PATH)
                        .expect("Environment variable BITCOIN_PROJ_PATH is empty");

    let bitcoind = bitcoin_path.into_string().unwrap() + BITCOIN_D;

    Command::new("sh")
            .arg("-c")
            .arg(format!("{} -regtest -datadir={} -daemon", 
                    bitcoind, get_alice_path())
            )
            .output()
            .map(|process| {
                if !process.status.success() {
                    println!("{}", str::from_utf8(&process.stderr).unwrap())
                } else {
                    println!("{}", str::from_utf8(&process.stdout).unwrap())
                };
            })
            .unwrap();

    Command::new("sh")
            .arg("-c")
            .arg(format!("{} -regtest -datadir={} -daemon", 
                    bitcoind, get_bob_path())
            )
            .output()
            .map(|process| {
                if !process.status.success() {
                    println!("{}", str::from_utf8(&process.stderr).unwrap())
                } else {
                    println!("{}", str::from_utf8(&process.stdout).unwrap())
                };
            })
         .unwrap()
}

/// Runs Bitcoin RPC commands for the Regtest Nodes.
fn run_bitcoin_rpc(cmd: &ArgMatches, node: RegtestNode) {
    let (node_name, node_path) = match node {
        RegtestNode::ALICE => ("alice", ALICE_PATH),
        RegtestNode::BOB => ("bob", BOB_PATH),
    };

    let rpc_cmd: Vec<&str> = cmd.subcommand_matches(node_name)
                                .unwrap()
                                .values_of("command")
                                .unwrap()
                                .collect();

    exec_rpc_command(rpc_cmd.join(" "), node_path)
}

fn exec_rpc_command(cmd: String, node_path: &str) {
    let bitcoin_cli = env::var_os(BITCOIN_PROJ_PATH)
                        .expect("Environment variable BITCOIN_PROJ_PATH is empty")
                        .into_string()
                        .unwrap() + BITCOIN_CLI;

    let path = env::var_os(node_path)
                        .expect("Environment variable for node path is empty")
                        .into_string()
                        .expect("Failed to convert node path to String");

    Command::new("sh")
            .arg("-c")
            .arg(format!("{} -regtest -datadir={} {}", bitcoin_cli, path, cmd))
            .output()
            .map(|process| {
                if !process.status.success() {
                    println!("{}", str::from_utf8(&process.stderr).unwrap())
                } else {
                    println!("{}", str::from_utf8(&process.stdout).unwrap())
                }
            })
            .unwrap();
}

/// Resets the Regtest network.
fn reset_network() {
    exec_rpc_command("stop".to_string(), ALICE_PATH);
    exec_rpc_command("stop".to_string(), BOB_PATH);

    match fs::remove_dir_all(format!("{}/regtest/", get_alice_path())) {
        Err(e) => {
            println!("ALICE_PATH: {}", e);
        },
        _ => (),
    }

    match fs::remove_dir_all(format!("{}/regtest/", get_bob_path())) {
        Err(e) => {
            println!("BOB_PATH: {}", e);
        },
        _ => (),
    }
}

fn get_alice_path() -> String {
    env::var_os(ALICE_PATH)
        .expect("Environment variable ALICE_PATH is empty")
        .into_string()
        .expect("Failed to convert ALICE_PATH to String")
}

fn get_bob_path() -> String {
    env::var_os(BOB_PATH)
        .expect("Environment variable BOB_PATH is empty")
        .into_string()
        .expect("Failed to convert BOB_PATH to String")
}
