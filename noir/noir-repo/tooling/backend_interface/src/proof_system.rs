use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use acvm::acir::native_types::{WitnessMap, WitnessStack};
use acvm::FieldElement;
use tempfile::tempdir;
use tracing::warn;

use crate::cli::{
    CircuitReport, GatesCommand, ProofAsFieldsCommand, ProveCommand, VerifyCommand,
    VkAsFieldsCommand, WriteVkCommand,
};
use crate::{Backend, BackendError};

impl Backend {
    pub fn get_exact_circuit_sizes(
        &self,
        artifact_path: PathBuf,
    ) -> Result<Vec<CircuitReport>, BackendError> {
        let binary_path = self.assert_binary_exists()?;
        self.assert_correct_version()?;

        GatesCommand { crs_path: self.crs_directory(), artifact_path }.run(binary_path)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn prove(
        &self,
        artifact_path: PathBuf,
        witness_stack: WitnessStack,
        num_public_inputs: u32,
    ) -> Result<Vec<u8>, BackendError> {
        let binary_path = self.assert_binary_exists()?;
        self.assert_correct_version()?;

        let temp_directory = tempdir().expect("could not create a temporary directory");
        let temp_directory = temp_directory.path().to_path_buf();

        // Create a temporary file for the witness
        let serialized_witnesses: Vec<u8> =
            witness_stack.try_into().expect("could not serialize witness map");
        let witness_path = temp_directory.join("witness").with_extension("tr");
        write_to_file(&serialized_witnesses, &witness_path);

        // Create proof and store it in the specified path
        let proof_with_public_inputs =
            ProveCommand { crs_path: self.crs_directory(), artifact_path, witness_path }
                .run(binary_path)?;

        let proof = bb_abstraction_leaks::remove_public_inputs(
            // TODO(https://github.com/noir-lang/noir/issues/4428)
            num_public_inputs as usize,
            &proof_with_public_inputs,
        );
        Ok(proof)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn verify(
        &self,
        proof: &[u8],
        public_inputs: WitnessMap,
        artifact_path: PathBuf,
    ) -> Result<bool, BackendError> {
        let binary_path = self.assert_binary_exists()?;
        self.assert_correct_version()?;

        let temp_directory = tempdir().expect("could not create a temporary directory");
        let temp_directory = temp_directory.path().to_path_buf();

        // Create a temporary file for the proof
        let proof_with_public_inputs =
            bb_abstraction_leaks::prepend_public_inputs(proof.to_vec(), public_inputs);
        let proof_path = temp_directory.join("proof").with_extension("proof");
        write_to_file(&proof_with_public_inputs, &proof_path);

        // Create the verification key and write it to the specified path
        let vk_path = temp_directory.join("vk");

        WriteVkCommand {
            crs_path: self.crs_directory(),
            artifact_path,
            vk_path_output: vk_path.clone(),
        }
        .run(binary_path)?;

        // Verify the proof
        VerifyCommand { crs_path: self.crs_directory(), proof_path, vk_path }.run(binary_path)
    }

    pub fn get_intermediate_proof_artifacts(
        &self,
        artifact_path: PathBuf,
        proof: &[u8],
        public_inputs: WitnessMap,
    ) -> Result<(Vec<FieldElement>, FieldElement, Vec<FieldElement>), BackendError> {
        let binary_path = self.assert_binary_exists()?;
        self.assert_correct_version()?;

        let temp_directory = tempdir().expect("could not create a temporary directory");
        let temp_directory = temp_directory.path().to_path_buf();

        // Create the verification key and write it to the specified path
        let vk_path = temp_directory.join("vk");

        WriteVkCommand {
            crs_path: self.crs_directory(),
            artifact_path,
            vk_path_output: vk_path.clone(),
        }
        .run(binary_path)?;

        // Create a temporary file for the proof

        let proof_with_public_inputs =
            bb_abstraction_leaks::prepend_public_inputs(proof.to_vec(), public_inputs);
        let proof_path = temp_directory.join("proof").with_extension("proof");
        write_to_file(&proof_with_public_inputs, &proof_path);

        // Now ready to generate intermediate artifacts.

        let proof_as_fields =
            ProofAsFieldsCommand { proof_path, vk_path: vk_path.clone() }.run(binary_path)?;

        let (vk_hash, vk_as_fields) = VkAsFieldsCommand { vk_path }.run(binary_path)?;

        Ok((proof_as_fields, vk_hash, vk_as_fields))
    }
}

pub(super) fn write_to_file(bytes: &[u8], path: &Path) -> String {
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {display}: {why}"),
        Ok(file) => file,
    };

    match file.write_all(bytes) {
        Err(why) => panic!("couldn't write to {display}: {why}"),
        Ok(_) => display.to_string(),
    }
}
