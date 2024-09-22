use transfer_sol::entry as entry_transfer_sol;
use transfer_sol::ID as PROGRAM_ID_TRANSFER_SOL;
const PROGRAM_NAME_TRANSFER_SOL: &str =  "transfer_sol";
use fuzz_instructions::transfer_sol_fuzz_instructions::FuzzInstruction as FuzzInstruction_transfer_sol;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_transfer_sol;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_TRANSFER_SOL,
                &PROGRAM_ID_TRANSFER_SOL,
                processor!(convert_entry!(entry_transfer_sol))
            );

            let metaplex_program = FuzzingProgram::new(
                "metaplex-token-metadata",
                &anchor_spl::metadata::ID,
                None
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1, metaplex_program])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_TRANSFER_SOL, &mut client);
        });
    }
}
