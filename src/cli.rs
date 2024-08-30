use crate::utils::{
    console::{prompt, prompt_with_default},
    default_values::{
        DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH, DEFAULT_RECURSION_LEAF_VK_HASH,
        DEFAULT_RECURSION_NODE_VK_HASH, DEFAULT_RECURSION_SCHEDULER_VK_HASH, DEFAULT_VERSION_PATCH,
    },
    queries::{get_prover_protocol_version, insert_prover_protocol_version, insert_witness_inputs},
    types::GetBatchResponse,
};
use clap::Parser;
use spinoff::{spinners::Dots, Color, Spinner};
use sqlx::{pool::PoolConnection, Postgres};
use std::str::FromStr;
use zksync_ethers_rs::types::{
    zksync::{
        inputs::WitnessInputData, protocol_version::ProtocolSemanticVersion, L1BatchNumber,
        ProtocolVersionId,
    },
    Bytes, TryFromPrimitive,
};

pub const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name="prover", author, version=VERSION_STRING, about, long_about = None)]
pub enum Command {
    #[clap(about = "Inserts witness inputs into the prover's database.", visible_aliases = ["insert-witness", "insert-witness-inputs"])]
    InsertNextBatchWitnessInputs,
}

impl Command {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Command::InsertNextBatchWitnessInputs => {
                let participant_id: String = prompt("Insert your participant ID")?;
                let server_url: String = prompt("Insert the server URL")?;
                let batch_data_response =
                    get_batch_data_from_server(&participant_id, &server_url).await?;
                let batch_witness_input_data_bytes =
                    download_batch_witness_input_data(batch_data_response.batch_file, &server_url)
                        .await?;
                let batch_witness_input_data =
                    deserialize_batch_witness_input_data(&batch_witness_input_data_bytes).await?;
                let witness_inputs_blob_url = format!(
                    "witness_inputs_{}.bin",
                    batch_witness_input_data.vm_run_data.l1_batch_number
                );
                write_batch_witness_input_data(
                    &batch_witness_input_data_bytes,
                    &witness_inputs_blob_url,
                )?;
                let prover_db_url = prompt_with_default(
                    "Insert your prover database URL",
                    String::from_str(
                        "postgres://postgres:notsecurepassword@localhost/prover_local",
                    )?,
                )?;
                let mut prover_db = connect_to_prover_database(&prover_db_url).await?;
                let protocol_version =
                    check_prover_database_protocol_version(&mut prover_db).await?;
                insert_batch_witness_input_to_prover_database(
                    batch_witness_input_data.vm_run_data.l1_batch_number,
                    &witness_inputs_blob_url,
                    protocol_version,
                    &mut prover_db,
                )
                .await?
            }
        };
        Ok(())
    }
}

async fn get_batch_data_from_server(
    participant_id: &str,
    server_url: &str,
) -> eyre::Result<GetBatchResponse> {
    let mut spinner = Spinner::new(Dots, "Getting batch data from the server", Color::Blue);
    let batch_data_url = format!("{server_url}/get_batch/?participant_id={participant_id}");
    let batch_response = reqwest::get(&batch_data_url)
        .await?
        .json::<GetBatchResponse>()
        .await?;
    spinner.success(&format!("Batch data received: {batch_response:?}"));
    Ok(batch_response)
}

async fn download_batch_witness_input_data(
    batch_file: String,
    server_url: &str,
) -> eyre::Result<Bytes> {
    let mut spinner = Spinner::new(Dots, "Downloading batch witness input data", Color::Blue);
    let batch_witness_input_data_url = format!("{server_url}/{batch_file}");
    let batch_witness_input_data_bytes = reqwest::get(&batch_witness_input_data_url)
        .await?
        .bytes()
        .await?;
    spinner.success("Batch witness input data downloaded");
    Ok(batch_witness_input_data_bytes.into())
}

async fn deserialize_batch_witness_input_data(
    batch_witness_input_data_bytes: &[u8],
) -> eyre::Result<WitnessInputData> {
    let mut spinner = Spinner::new(Dots, "Deserializing batch witness input data", Color::Blue);
    let batch_witness_input_data: WitnessInputData =
        bincode::deserialize(batch_witness_input_data_bytes)?;
    spinner.success("Batch witness input data deserialized");
    Ok(batch_witness_input_data)
}

fn write_batch_witness_input_data(
    batch_witness_input_data_bytes: &[u8],
    witness_inputs_blob_url: &str,
) -> eyre::Result<()> {
    let mut spinner = Spinner::new(Dots, "Writing batch witness input data", Color::Blue);
    match std::fs::write(witness_inputs_blob_url, batch_witness_input_data_bytes) {
        Ok(_) => {
            spinner.success("Batch witness input data written.");
            Ok(())
        }
        Err(e) => {
            spinner.fail("Batch witness input data writing failed.");
            Err(e.into())
        }
    }
}

async fn connect_to_prover_database(prover_db_url: &str) -> eyre::Result<PoolConnection<Postgres>> {
    let mut spinner = Spinner::new(Dots, "Connecting to the prover's database", Color::Blue);
    match sqlx::PgPool::connect_lazy(prover_db_url)?.acquire().await {
        Ok(connection) => {
            spinner.success("Prover database connection established");
            Ok(connection)
        }
        Err(e) => {
            spinner.fail("Prover database connection failed");
            Err(e.into())
        }
    }
}

async fn check_prover_database_protocol_version(
    prover_db: &mut PoolConnection<Postgres>,
) -> eyre::Result<ProtocolSemanticVersion> {
    let mut spinner = Spinner::new(
        Dots,
        "Checking prover database protocol version",
        Color::Blue,
    );
    if let Some(prover_protocol_version) = get_prover_protocol_version(prover_db).await? {
        spinner.success(&format!(
            "Prover protocol version found in database: {prover_protocol_version}"
        ));
        Ok(prover_protocol_version)
    } else {
        spinner.warn("No protocol version found in prover's database.");
        let protocol_version = ProtocolVersionId::try_from_primitive(prompt_with_default(
            "Prover protocol version",
            24,
        )?)?;
        let recursion_scheduler_vk_hash = prompt_with_default(
            "Recursion scheduler verification key hash",
            DEFAULT_RECURSION_SCHEDULER_VK_HASH,
        )?;
        let recursion_node_vk_hash = prompt_with_default(
            "Recursion node verification key hash",
            DEFAULT_RECURSION_NODE_VK_HASH,
        )?;
        let recursion_leaf_vk_hash = prompt_with_default(
            "Recursion leaf verification key hash",
            DEFAULT_RECURSION_LEAF_VK_HASH,
        )?;
        let recursion_circuits_set_vk_hash = prompt_with_default(
            "Recursion circuits set verification keys hash",
            DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH,
        )?;
        let protocol_version_patch =
            prompt_with_default("Prover protocol version patch", DEFAULT_VERSION_PATCH)?;
        let mut spinner = Spinner::new(
            Dots,
            "Saving prover protocol version in database",
            Color::Blue,
        );
        match insert_prover_protocol_version(
            protocol_version,
            recursion_scheduler_vk_hash,
            recursion_node_vk_hash,
            recursion_leaf_vk_hash,
            recursion_circuits_set_vk_hash,
            protocol_version_patch,
            prover_db,
        )
        .await
        {
            Ok(_) => {
                let protocol_version =
                    ProtocolSemanticVersion::new(protocol_version, protocol_version_patch);
                spinner.success(&format!(
                    "Prover protocol version saved in database: {protocol_version}"
                ));
                Ok(protocol_version)
            }
            Err(e) => {
                spinner.fail("Prover protocol version saving failed.");
                Err(e)
            }
        }
    }
}

async fn insert_batch_witness_input_to_prover_database(
    l1_batch_number: L1BatchNumber,
    witness_inputs_blob_url: &str,
    protocol_version: ProtocolSemanticVersion,
    prover_db: &mut PoolConnection<Postgres>,
) -> eyre::Result<()> {
    let mut spinner = Spinner::new(Dots, "Inserting witness inputs", Color::Blue);
    match insert_witness_inputs(
        l1_batch_number,
        witness_inputs_blob_url,
        protocol_version.minor,
        protocol_version.patch,
        prover_db,
    )
    .await
    {
        Ok(_) => {
            spinner.success("Batch proof inserted.");
            Ok(())
        }
        Err(e) => {
            spinner.fail("Batch proof insertion failed.");
            Err(e)
        }
    }
}
