import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);

  // Deploy is handled by `anchor deploy`.
  // This file can be used for post-deploy initialization.
  console.log("Tribe Protocol deployed successfully.");
};
