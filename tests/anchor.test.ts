// No imports needed: web3, anchor, pg and more are globally available

describe("Tienda de Laptops", () => {
  it("crear tienda", async () => {

    const tiendaPda = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tienda"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    )[0];

    const nombre = "Laptop Store";

    const txHash = await pg.program.methods
      .crearTienda(nombre)
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    await pg.connection.confirmTransaction(txHash);

    const tienda = await pg.program.account.tienda.fetch(tiendaPda);

    console.log("Datos on-chain de la tienda:", tienda);
  });
});
