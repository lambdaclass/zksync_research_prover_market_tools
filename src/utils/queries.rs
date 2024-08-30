use sqlx::{pool::PoolConnection, Executor, Postgres, Row};
use zksync_ethers_rs::{
    abi::Hash,
    types::{
        zksync::{
            protocol_version::{ProtocolSemanticVersion, VersionPatch},
            L1BatchNumber, ProtocolVersionId,
        },
        TryFromPrimitive,
    },
};

pub async fn insert_witness_inputs(
    l1_batch_number: L1BatchNumber,
    witness_inputs_blob_url: &str,
    protocol_version: ProtocolVersionId,
    protocol_version_patch: VersionPatch,
    prover_db: &mut PoolConnection<Postgres>,
) -> eyre::Result<()> {
    let query = format!(
        "
        INSERT INTO
            witness_inputs_fri (
                l1_batch_number,
                witness_inputs_blob_url,
                protocol_version,
                status,
                created_at,
                updated_at,
                protocol_version_patch
            )
        VALUES
            ({}, '{}', {}, 'queued', NOW(), NOW(), {})
        ON CONFLICT (l1_batch_number) DO NOTHING
        ",
        l1_batch_number.0, witness_inputs_blob_url, protocol_version as u16, protocol_version_patch
    );
    prover_db.execute(query.as_str()).await?;
    Ok(())
}

pub async fn insert_prover_protocol_version(
    protocol_version: ProtocolVersionId,
    recursion_scheduler_level_vk_hash: Hash,
    recursion_node_level_vk_hash: Hash,
    recursion_leaf_level_vk_hash: Hash,
    recursion_circuits_set_vks_hash: Hash,
    protocol_version_patch: VersionPatch,
    prover_db: &mut PoolConnection<Postgres>,
) -> eyre::Result<()> {
    let query = format!(
        "
        INSERT INTO
            prover_fri_protocol_versions (
                id,
                recursion_scheduler_level_vk_hash,
                recursion_node_level_vk_hash,
                recursion_leaf_level_vk_hash,
                recursion_circuits_set_vks_hash,
                protocol_version_patch,
                created_at
            )
        VALUES
            ({}, '\\x{:x}', '\\x{:x}', '\\x{:x}', '\\x{:x}', {}, NOW())
        ON CONFLICT (id, protocol_version_patch) DO UPDATE SET
            recursion_scheduler_level_vk_hash = EXCLUDED.recursion_scheduler_level_vk_hash,
            recursion_node_level_vk_hash = EXCLUDED.recursion_node_level_vk_hash,
            recursion_leaf_level_vk_hash = EXCLUDED.recursion_leaf_level_vk_hash,
            recursion_circuits_set_vks_hash = EXCLUDED.recursion_circuits_set_vks_hash
        ",
        protocol_version as u16,
        recursion_scheduler_level_vk_hash,
        recursion_node_level_vk_hash,
        recursion_leaf_level_vk_hash,
        recursion_circuits_set_vks_hash,
        protocol_version_patch
    );
    prover_db.execute(query.as_str()).await?;
    Ok(())
}

pub async fn get_prover_protocol_version(
    prover_db: &mut PoolConnection<Postgres>,
) -> eyre::Result<Option<ProtocolSemanticVersion>> {
    let query = "
        SELECT
            id,
            protocol_version_patch
        FROM prover_fri_protocol_versions
        ORDER BY created_at DESC
        LIMIT 1
        ";
    let row = prover_db.fetch_optional(query).await?;
    Ok(row.map(|r| {
        let protocol_version: i32 = r.get("id");
        let protocol_version_patch: i32 = r.get("protocol_version_patch");
        ProtocolSemanticVersion::new(
            ProtocolVersionId::try_from_primitive(
                protocol_version
                    .try_into()
                    .expect("Invalid protocol version"),
            )
            .expect("Invalid protocol version"),
            VersionPatch(
                protocol_version_patch
                    .try_into()
                    .expect("Invalid protocol version patch"),
            ),
        )
    }))
}
