use {
    anchor_lang::prelude::*,
    instructions::*,
};

pub mod instructions;
pub mod message;
mod state;

declare_id!("Dg5PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mock_cpi_caller {
    use super::*;

    /// Creates a `PriceAccount` with the given parameters
    pub fn add_price<'info>(
        ctx: Context<'_, '_, '_, 'info, AddPrice<'info>>,
        params: AddPriceParams,
    ) -> Result<()> {
        instructions::add_price(ctx, params)
    }

    /// Updates a `PriceAccount` with the given parameters
    pub fn update_price<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdatePrice<'info>>,
        params: UpdatePriceParams,
    ) -> Result<()> {
        instructions::update_price(ctx, params)
    }
}


#[cfg(test)]
mod test {
    use {
        super::*,
        anchor_lang::InstructionData,
    };

    #[test]
    fn ix_discriminator() {
        let a = &(accumulator_updater::instruction::PutAll {
            base_account_key: anchor_lang::prelude::Pubkey::default(),
            values:           vec![],
        }
        .data()[..8]);

        let sighash = sighash("global", ACCUMULATOR_UPDATER_IX_NAME);
        println!(
            r"
            a: {a:?}
            sighash: {sighash:?}
            ",
        );
        assert_eq!(a, &sighash);
    }
}
