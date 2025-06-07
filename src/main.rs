#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use eyre::WrapErr;
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use std::path::PathBuf;

use clap::Parser as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_grpc_proto::geyser::{SubscribeUpdateTransaction, SubscribeUpdateTransactionInfo};
use yellowstone_vixen::{self as vixen, proto::parser, vixen_core::proto::Proto, Pipeline};
use yellowstone_vixen_jupiter_swap_parser::{
    accounts_parser::AccountParser as JupiterSwapAccParser,
    instructions_parser::InstructionParser as JupiterSwapIxParser,
    proto_def::DESCRIPTOR_SET as JUPITER_SWAP_DESCRIPTOR_SET,
};
use yellowstone_vixen_kamino_limit_orders_parser::{
    accounts_parser::AccountParser as KaminoLimitOrdersAccParser,
    instructions_parser::InstructionParser as KaminoLimitOrdersIxParser,
    proto_def::DESCRIPTOR_SET as KAMINO_LIMIT_ORDERS_DESCRIPTOR_SET,
};
use yellowstone_vixen_meteora_amm_parser::{
    accounts_parser::AccountParser as MeteoraAmmAccParser,
    instructions_parser::InstructionParser as MeteoraAmmIxParser,
    proto_def::DESCRIPTOR_SET as METEORA_AMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_meteora_parser::{
    accounts_parser::AccountParser as MeteoraAccParser,
    instructions_parser::InstructionParser as MeteoraIxParser,
    proto_def::DESCRIPTOR_SET as METEORA_DESCRIPTOR_SET,
};
use yellowstone_vixen_moonshot_parser::{
    accounts_parser::AccountParser as MoonshotAccParser,
    instructions_parser::InstructionParser as MoonshotIxParser,
    proto_def::DESCRIPTOR_SET as MOONSHOT_DESCRIPTOR_SET,
};
use yellowstone_vixen_orca_whirlpool_parser::{
    accounts_parser::AccountParser as OrcaWhirlpoolAccParser,
    instructions_parser::InstructionParser as OrcaWhirlpoolIxParser,
    proto_def::DESCRIPTOR_SET as ORCA_WHIRLPOOL_DESCRIPTOR_SET,
};
use yellowstone_vixen_parser::{
    token_extension_program::{
        AccountParser as TokenExtensionProgramAccParser,
        InstructionParser as TokenExtensionProgramIxParser,
    },
    token_program::{
        AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    },
};
use yellowstone_vixen_pump_swaps_parser::{
    accounts_parser::AccountParser as PumpAmmAccParser,
    instructions_parser::InstructionParser as PumpAmmIxParser,
    proto_def::DESCRIPTOR_SET as PUMP_AMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_pumpfun_parser::{
    accounts_parser::AccountParser as PumpfunAccParser,
    instructions_parser::InstructionParser as PumpfunIxParser,
    proto_def::DESCRIPTOR_SET as PUMP_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_amm_v4_parser::{
    accounts_parser::AccountParser as RaydiumAmmV4AccParser,
    instructions_parser::InstructionParser as RaydiumAmmV4IxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_AMM_V4_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_clmm_parser::{
    accounts_parser::AccountParser as RaydiumClmmAccParser,
    instructions_parser::InstructionParser as RaydiumClmmIxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_CLMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_cpmm_parser::{
    accounts_parser::AccountParser as RaydiumCpmmAccParser,
    instructions_parser::InstructionParser as RaydiumCpmmIxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_CPMM_DESCRIPTOR_SET,
};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

async fn _unused() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::stream::Server::builder()
        // .descriptor_set(parser::token::DESCRIPTOR_SET)
        // .descriptor_set(parser::token_extensions::DESCRIPTOR_SET)
        .descriptor_set(METEORA_DESCRIPTOR_SET)
        // .descriptor_set(PUMP_DESCRIPTOR_SET)
        // .descriptor_set(JUPITER_SWAP_DESCRIPTOR_SET)
        // .descriptor_set(PUMP_AMM_DESCRIPTOR_SET)
        // .descriptor_set(RAYDIUM_CPMM_DESCRIPTOR_SET)
        // .descriptor_set(ORCA_WHIRLPOOL_DESCRIPTOR_SET)
        // .descriptor_set(MOONSHOT_DESCRIPTOR_SET)
        // .descriptor_set(METEORA_AMM_DESCRIPTOR_SET)
        // .descriptor_set(RAYDIUM_AMM_V4_DESCRIPTOR_SET)
        // .descriptor_set(RAYDIUM_CLMM_DESCRIPTOR_SET)
        // .descriptor_set(KAMINO_LIMIT_ORDERS_DESCRIPTOR_SET)
        // .account(Proto::new(MeteoraAccParser))
        // .account(Proto::new(PumpfunAccParser))
        // .account(Proto::new(TokenExtensionProgramAccParser))
        // .account(Proto::new(TokenProgramAccParser))
        // .account(Proto::new(JupiterSwapAccParser))
        //.account(Proto::new(PumpAmmAccParser))
        //.account(Proto::new(RaydiumCpmmAccParser))
        //.account(Proto::new(OrcaWhirlpoolAccParser))
        //.account(Proto::new(MoonshotAccParser))
        // .account(Proto::new(MeteoraAmmAccParser))
        //.account(Proto::new(RaydiumAmmV4AccParser))
        //.account(Proto::new(RaydiumClmmAccParser))
        // .account(Proto::new(KaminoLimitOrdersAccParser))
        .instruction(Proto::new(MeteoraIxParser))
        // .instruction(Proto::new(PumpfunIxParser))
        // .instruction(Proto::new(TokenProgramIxParser))
        // .instruction(Proto::new(TokenExtensionProgramIxParser))
        // .instruction(Proto::new(JupiterSwapIxParser))
        // .instruction(Proto::new(PumpAmmIxParser))
        // .instruction(Proto::new(RaydiumCpmmIxParser))
        // .instruction(Proto::new(OrcaWhirlpoolIxParser))
        // .instruction(Proto::new(MoonshotIxParser))
        // .instruction(Proto::new(MeteoraAmmIxParser))
        // .instruction(Proto::new(RaydiumAmmV4IxParser))
        // .instruction(Proto::new(RaydiumClmmIxParser))
        // .instruction(Proto::new(KaminoLimitOrdersIxParser))
        .build(config)
        .try_run_async()
        .await
        .wrap_err("Error running server")?;

    Ok(())
}

#[derive(Debug)]
struct SolParser;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V> for SolParser {
    async fn handle(&self, value: &V) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::from_default_env())
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // let Opts { config } = Opts::parse();
    // let config =
    //     toml::from_str(&std::fs::read_to_string(config).expect("Error reading config file"))
    //         .expect("Error parsing config");

    // let runtime = vixen::Runtime::builder()
    //     .instruction(Pipeline::new(Proto::new(MeteoraIxParser), [SolParser]))
    //     .build(config);

    // let rpc_client =
    //     RpcClient::new(std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL is set"));

    // let transaction = rpc_client
    //     .get_transaction(
    //         &transaction_signature.into(),
    //         UiTransactionEncoding::JsonParsed,
    //     )
    //     .await?;

    // let update: SubscribeUpdateTransaction = SubscribeUpdateTransaction {
    //     slot: 123,
    //     transaction,
    // };

    // runtime
    //     .process_transaction(
    //         yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction::default(),
    //         vec![],
    //     )
    //     .await
    //     .wrap_err("Error processing transaction")?;

    _unused().await?;

    Ok(())
}

fn transaction_to_subscribe_update_transaction(
    signature: Vec<u8>,
    transaction: EncodedConfirmedTransactionWithStatusMeta,
) -> SubscribeUpdateTransaction {
    tracing::info!("{}", serde_json::to_string_pretty(&transaction).unwrap());
    SubscribeUpdateTransaction {
        slot: transaction.slot,
        transaction: Some(SubscribeUpdateTransactionInfo {
            is_vote: false,
            signature,
            ..Default::default()
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_convert_transaction_into_subscribe_update_transaction() {
        let rpc_client =
            RpcClient::new(std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL is set"));

        let transaction_signature =
         "4X6qJrEKNvWYNNG3zSmCFZAVXoLCz8tsEA86uH2oAeNtMvhK5CoPiQTwFzCmUQRMRQdETLBsT781TWvzNU7AVFmg";

        let transaction = rpc_client
            .get_transaction(
                &transaction_signature.into(),
                UiTransactionEncoding::JsonParsed,
            )
            .await;

        let update = SubscribeUpdateTransaction::try_from(transaction).unwrap();
    }
}
