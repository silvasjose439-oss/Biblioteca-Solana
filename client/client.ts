// Client

console.log("Dirección del cliente:", pg.wallet.publicKey.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Saldo disponible para comprar laptops: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
