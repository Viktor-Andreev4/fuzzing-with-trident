# Solana Project Fuzzing Report

## Overview

This report covers the Solana project developed with Anchor, focusing on the `transfer_sol` module which provides two key functions to transfer SOL tokens between accounts:
1. **Transfer using CPI (Cross Program Invocation)**.
2. **Transfer using direct program-level manipulation of lamports**.

The fuzzing was conducted using Trident, and the findings are discussed in detail below.

## Project Details

### Program ID

```rs
declare_id!("4fQVnLWKKKYxtxgGn7Haw8v2g2Hzbu8K61JvWKvqAi7W");
```

### Core Functions
1. **transfer_sol_with_cpi**
   
This function leverages the system program to transfer SOL using a CPI (Cross Program Invocation). It involves the following steps:

* The transfer is done from a payer account to a recipient account.
* The system program is used to ensure that the transaction is safe and follows the Solana runtime rules.

```rust
pub fn transfer_sol_with_cpi(ctx: Context<TransferSolWithCpi>, amount: u64) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
```
2. **transfer_sol_with_program**
   
This method performs a direct modification of lamports on the accounts:

* **Direct modification** is only possible when the program is the owner of the account.
* The function deducts lamports from the payer account and adds them to the recipient.

```rs
pub fn transfer_sol_with_program(ctx: Context<TransferSolWithProgram>, amount: u64) -> Result<()> {
    **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;
    Ok(())
}
```
### Account Definitions
1. **TransferSolWithCpi**
   
This account struct defines the necessary inputs for the CPI-based SOL transfer.

```rs
#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
```
2. **TransferSolWithProgram**
   
This struct defines the accounts for program-level direct transfers, including unchecked account handling.

```rs
#[derive(Accounts)]
pub struct TransferSolWithProgram<'info> {
    #[account(mut, owner = id())]
    payer: UncheckedAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
}
```
## Fuzzing Report
### Summary
The fuzzing was performed using the Trident fuzzing tool. The following key statistics were recorded:
### Fuzzing Details

#### Run Details

```bash
Iterations : 7 (out of: 1000 [0%])
Mode [1/3] : Feedback Driven Dry Run [7/7]
Target : trident-tests/fuzz_tests/fuzzing.....wn-linux-gnu/release/fuzz_0
Threads : 6, CPUs: 12, CPU%: 138% [11%/CPU]
Speed : 0/sec [avg: 3]
Crashes : 5 [unique: 1, blocklist: 0, verified: 0]
Timeouts : 0 [10 sec]
Corpus Size : 7, max: 1048576 bytes, init: 7 files
Coverage : edge: 5980/998045 [0%], pc: 88, cmp: 376876
```

#### Crashes
5 crashes were observed during the fuzzing session, with only 1 being unique. This indicates that the issue might have a common root cause across multiple runs.

#### Coverage
The current coverage was 0% in terms of edges, with 5980 edges covered out of 998045 potential edges. This indicates that the fuzzer has explored only a small portion of the code base, suggesting further testing and fuzzing iterations are needed to expand coverage.

#### Instruction Sequence
Here is a sample of the instruction sequence that was processed during fuzzing:

```rs
TransferSolWithCpi(TransferSolWithCpi {
    accounts: TransferSolWithCpiAccounts {
        payer: 0,
        recipient: 0,
        system_program: 0,
    },
    data: TransferSolWithCpiData {
        amount: 0,
    },
})
```
#### **In-Progress Details**
```rs
Currently processing: TransferSolWithCpi(TransferSolWithCpi {
    accounts: TransferSolWithCpiAccounts {
        payer: 0,
        recipient: 0,
        system_program: 0,
    },
    data: TransferSolWithCpiData {
        amount: 0,
    },
})
```
This indicates that the fuzzer was analyzing the **TransferSolWithCpi** function at the time of reporting.

### Conclusion
The fuzzing session conducted with **Trident** uncovered a small number of crashes with minimal code coverage. 