# Solana Transfer Project

## Overview

This project implements a Solana program using the **Anchor framework** to transfer SOL tokens between accounts. The program provides two methods for transferring SOL:

1. **Cross Program Invocation (CPI) Transfer**: Uses Solana's built-in system program to safely transfer SOL between accounts.
2. **Direct Transfer via Program Ownership**: Directly modifies the lamport balance of accounts, which is only allowed when the program owns the accounts.

## Functions

### 1. Transfer SOL with CPI
Transfers SOL using Solana's system program. This function ensures that the transfer follows Solana’s security rules.

```rs
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

### 2. Direct Transfer via Program Ownership
Directly transfers SOL between accounts by modifying the lamports. This method is only valid if the program is the owner of the involved accounts.
```rs
pub fn transfer_sol_with_program(ctx: Context<TransferSolWithProgram>, amount: u64) -> Result<()> {
    **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;
    Ok(())
}
```

## Structure

- **Anchor framework**: Used for Solana program development.
- **System Program**: Utilized for secure SOL transfers.
- **Unchecked Accounts**: Handled with appropriate checks to ensure correct ownership and balance modification.