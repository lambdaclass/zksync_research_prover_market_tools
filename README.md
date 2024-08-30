# research_prover_market_tools

Tools for ZKsync's Data Collection for Prover Market Design.

## How to prove batch data coming from the provided endpoint

The data that we get from the `/get_batch` endpoint is essentially `WitnessInputData`. This is the data necessary by the prover subsystems to start generating the proof for some batch, specifically this is the data the the basic witness generator needs to start processing some batch proof. In order for us to start this process, we need to insert this data to the database, into the `witness_inputs_fri` table (`basic_witness_inputs_fri` in the future). This is done by running the following command:

```
cargo +nightly run --release -- insert-witness
```

which will ask you to enter certain necessary data, these being your `participant_id` (provided to you by the Matter Labs team), and your prover database URL (with this format `postgres://postgres:notsecurepassword@localhost/prover_local`).

> NOTE: If you have a fresh new prover database, you will be asked to enter the prover's protocol version you're running, defaulting `0.24.2` if you don't enter anything. This data is needed to insert the witness data because there's some foreign key constraints that need to be satisfied.

After you've inserted the data, you need to move the downloaded batch data to your prover's object store, if you have a local object store then you need to move the file to the corresponding local directory (set in the prover's config) or if you are using the prover with Buckets then you need to upload the file to the corresponding bucket. After you've done this, you can either start the prover subsystems or just restart the basic witness generator and wait for it to start working (could take a little bit for it to take the job).

Once the final proof is ready, we can start the process of submitting it.

## How to submit a final proof

TODO.
