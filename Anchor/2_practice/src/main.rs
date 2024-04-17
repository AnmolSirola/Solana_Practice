
// Declare the program module
#[program]
mod basic_2 {
    // Bring everything from the parent module into scope
    use super::*;

    // Entry point for creating a new counter
    pub fn create(ctx: Context<Create>, authority: Pubkey) -> Result<()> {

        // Get a mutable reference to the counter account from the context
        let counter = &mut ctx.accounts.counter;
        
        // Set the authority of the counter to the provided public key
        counter.authority = authority;
        
        // Initialize the count to zero
        counter.count = 0;
        
        // Return Ok to indicate successful execution
        Ok(())
    }

    // Entry point for incrementing the counter
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Get a mutable reference to the counter account from the context
        let counter = &mut ctx.accounts.counter;
        
        // Increment the count by 1
        counter.count += 1;
        
        // Return Ok to indicate successful execution
        Ok(())
    }
}