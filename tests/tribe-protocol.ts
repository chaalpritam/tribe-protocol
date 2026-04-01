import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

// Program types will be generated after `anchor build`
// import { FidRegistry } from "../target/types/fid_registry";
// import { AppKeyRegistry } from "../target/types/app_key_registry";
// import { UsernameRegistry } from "../target/types/username_registry";
// import { SocialGraph } from "../target/types/social_graph";

describe("tribe-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Uncomment after `anchor build` generates IDL types:
  // const fidRegistry = anchor.workspace.FidRegistry as Program<FidRegistry>;
  // const appKeyRegistry = anchor.workspace.AppKeyRegistry as Program<AppKeyRegistry>;
  // const usernameRegistry = anchor.workspace.UsernameRegistry as Program<UsernameRegistry>;
  // const socialGraph = anchor.workspace.SocialGraph as Program<SocialGraph>;

  describe("FID Registry", () => {
    it("initializes global state", async () => {
      // TODO: Call initialize, verify fid_counter = 0
    });

    it("registers a new FID", async () => {
      // TODO: Call register, verify FID = 1, custody = signer
    });

    it("transfers FID custody", async () => {
      // TODO: Call transfer, verify new custody address
    });

    it("recovers FID using recovery address", async () => {
      // TODO: Call recover with recovery signer
    });

    it("changes recovery address", async () => {
      // TODO: Call change_recovery
    });

    it("rejects unauthorized transfer", async () => {
      // TODO: Expect UnauthorizedCustody error
    });
  });

  describe("App Key Registry", () => {
    it("adds an app key", async () => {
      // TODO: Add app key with Full scope
    });

    it("revokes an app key", async () => {
      // TODO: Revoke and verify revoked = true
    });

    it("rotates an app key", async () => {
      // TODO: Rotate old → new, verify old revoked + new active
    });

    it("rejects invalid scope", async () => {
      // TODO: Expect InvalidScope error for scope > 3
    });
  });

  describe("Username Registry", () => {
    it("registers a username", async () => {
      // TODO: Register "alice", verify binding to FID
    });

    it("renews a username", async () => {
      // TODO: Renew, verify expiry extended
    });

    it("rejects invalid characters", async () => {
      // TODO: Expect InvalidCharacters for "alice@bob"
    });

    it("releases a username", async () => {
      // TODO: Release, verify account closed
    });
  });

  describe("Social Graph", () => {
    it("initializes a social profile", async () => {
      // TODO: Init profile, verify counters = 0
    });

    it("follows a user (creates Link PDA)", async () => {
      // TODO: Follow, verify Link exists + counters incremented
    });

    it("unfollows a user (closes Link PDA)", async () => {
      // TODO: Unfollow, verify Link closed + counters decremented
    });

    it("rejects self-follow", async () => {
      // TODO: Expect CannotFollowSelf error
    });

    it("rejects duplicate follow", async () => {
      // TODO: Follow twice, expect PDA already exists error
    });
  });
});
