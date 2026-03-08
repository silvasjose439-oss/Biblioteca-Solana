use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tienda_laptops {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let laptops: Vec<Laptop> = Vec::new();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            laptops,
        });

        Ok(())
    }

    pub fn agregar_laptop(context: Context<NuevaLaptop>, nombre: String, precio: u16) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let laptop = Laptop {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.tienda.laptops.push(laptop);

        Ok(())
    }

    pub fn eliminar_laptop(context: Context<NuevaLaptop>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let laptops = &mut context.accounts.tienda.laptops;

        for i in 0..laptops.len() {
            if laptops[i].nombre == nombre {
                laptops.remove(i);
                msg!("Laptop {} eliminada!", nombre);
                return Ok(());
            }
        }

        Err(Errores::LaptopNoExiste.into())
    }

    pub fn ver_laptops(context: Context<NuevaLaptop>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "La lista de laptops actualmente es: {:#?}",
            context.accounts.tienda.laptops
        );

        Ok(())
    }

    pub fn alternar_disponibilidad(context: Context<NuevaLaptop>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let laptops = &mut context.accounts.tienda.laptops;

        for i in 0..laptops.len() {
            let estado = laptops[i].disponible;

            if laptops[i].nombre == nombre {
                let nuevo_estado = !estado;
                laptops[i].disponible = nuevo_estado;

                msg!(
                    "La laptop: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::LaptopNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,
    #[msg("Error, la laptop no existe")]
    LaptopNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    laptops: Vec<Laptop>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Laptop {
    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaLaptop<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
