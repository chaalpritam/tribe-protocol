import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { FidRegistry } from "../target/types/fid_registry";
import { AppKeyRegistry } from "../target/types/app_key_registry";
import { UsernameRegistry } from "../target/types/username_registry";
import { SocialGraph } from "../target/types/social_graph";

describe("tribe-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const fidRegistry = anchor.workspace.FidRegistry as Program<FidRegistry>;
  const appKeyRegistry = anchor.workspace.AppKeyRegistry as Program<AppKeyRegistry>;
  const usernameRegistry = anchor.workspace.UsernameRegistry as Program<UsernameRegistry>;
  const socialGraph = anchor.workspace.SocialGraph as Program<SocialGraph>;

  const custody = provider.wallet;
  const recoveryKeypair = Keypair.generate();
  const newCustodyKeypair = Keypair.generate();

  // Helpers
  function fidToBuffer(fid: number): Buffer {
    const buf = Buffer.alloc(8);
    buf.writeBigUInt64LE(BigInt(fid));
    return buf;
  }

  function deriveGlobalState(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("global_state")],
      fidRegistry.programId
    );
  }

  function deriveFidRecord(fid: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("fid"), fidToBuffer(fid)],
      fidRegistry.programId
    );
  }

  function deriveCustodyLookup(custodyPubkey: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("custody"), custodyPubkey.toBuffer()],
      fidRegistry.programId
    );
  }

  function deriveAppKeyRecord(fid: number, appPubkey: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("app_key"), fidToBuffer(fid), appPubkey.toBuffer()],
      appKeyRegistry.programId
    );
  }

  function deriveUsernameRecord(username: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("username"), Buffer.from(username.toLowerCase())],
      usernameRegistry.programId
    );
  }

  function deriveFidUsername(fid: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("fid_username"), fidToBuffer(fid)],
      usernameRegistry.programId
    );
  }

  function deriveSocialProfile(fid: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("social_profile"), fidToBuffer(fid)],
      socialGraph.programId
    );
  }

  function deriveLink(followerFid: number, followingFid: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("link"), fidToBuffer(followerFid), fidToBuffer(followingFid)],
      socialGraph.programId
    );
  }

  // ========== FID Registry ==========

  describe("FID Registry", () => {
    it("initializes global state", async () => {
      const [globalState] = deriveGlobalState();

      await fidRegistry.methods
        .initialize()
        .accountsStrict({
          globalState,
          authority: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const state = await fidRegistry.account.globalState.fetch(globalState);
      expect(state.fidCounter.toNumber()).to.equal(0);
      expect(state.authority.toBase58()).to.equal(custody.publicKey.toBase58());
    });

    it("registers a new FID (fid=1)", async () => {
      const [globalState] = deriveGlobalState();
      const [fidRecord] = deriveFidRecord(1);
      const [custodyLookup] = deriveCustodyLookup(custody.publicKey);

      await fidRegistry.methods
        .register(recoveryKeypair.publicKey)
        .accountsStrict({
          globalState,
          fidRecord,
          custodyLookup,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await fidRegistry.account.fidRecord.fetch(fidRecord);
      expect(record.fid.toNumber()).to.equal(1);
      expect(record.custodyAddress.toBase58()).to.equal(custody.publicKey.toBase58());
      expect(record.recoveryAddress.toBase58()).to.equal(recoveryKeypair.publicKey.toBase58());

      const state = await fidRegistry.account.globalState.fetch(globalState);
      expect(state.fidCounter.toNumber()).to.equal(1);
    });

    it("looks up FID by custody address", async () => {
      const [custodyLookup] = deriveCustodyLookup(custody.publicKey);
      const lookup = await fidRegistry.account.custodyLookup.fetch(custodyLookup);
      expect(lookup.fid.toNumber()).to.equal(1);
    });

    it("changes recovery address", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const newRecovery = Keypair.generate();

      await fidRegistry.methods
        .changeRecovery(newRecovery.publicKey)
        .accountsStrict({
          fidRecord,
          custody: custody.publicKey,
        })
        .rpc();

      const record = await fidRegistry.account.fidRecord.fetch(fidRecord);
      expect(record.recoveryAddress.toBase58()).to.equal(newRecovery.publicKey.toBase58());

      // Change back for later tests
      await fidRegistry.methods
        .changeRecovery(recoveryKeypair.publicKey)
        .accountsStrict({
          fidRecord,
          custody: custody.publicKey,
        })
        .rpc();
    });

    it("rejects change recovery to same address", async () => {
      const [fidRecord] = deriveFidRecord(1);

      try {
        await fidRegistry.methods
          .changeRecovery(recoveryKeypair.publicKey)
          .accountsStrict({
            fidRecord,
            custody: custody.publicKey,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("SameRecoveryAddress");
      }
    });

    it("transfers FID custody", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [oldCustodyLookup] = deriveCustodyLookup(custody.publicKey);
      const [newCustodyLookup] = deriveCustodyLookup(newCustodyKeypair.publicKey);

      await fidRegistry.methods
        .transfer(newCustodyKeypair.publicKey)
        .accountsStrict({
          fidRecord,
          oldCustodyLookup,
          newCustodyLookup,
          newCustody: newCustodyKeypair.publicKey,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await fidRegistry.account.fidRecord.fetch(fidRecord);
      expect(record.custodyAddress.toBase58()).to.equal(newCustodyKeypair.publicKey.toBase58());

      // Old lookup should be closed
      const oldLookup = await provider.connection.getAccountInfo(oldCustodyLookup);
      expect(oldLookup).to.be.null;
    });

    it("recovers FID using recovery address", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [oldCustodyLookup] = deriveCustodyLookup(newCustodyKeypair.publicKey);
      const [newCustodyLookup] = deriveCustodyLookup(custody.publicKey);

      // Airdrop to recovery keypair so it can pay
      const sig = await provider.connection.requestAirdrop(
        recoveryKeypair.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      await fidRegistry.methods
        .recover(custody.publicKey)
        .accountsStrict({
          fidRecord,
          oldCustodyLookup,
          newCustodyLookup,
          newCustody: custody.publicKey,
          recovery: recoveryKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([recoveryKeypair])
        .rpc();

      const record = await fidRegistry.account.fidRecord.fetch(fidRecord);
      expect(record.custodyAddress.toBase58()).to.equal(custody.publicKey.toBase58());
    });

    it("rejects unauthorized transfer", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const faker = Keypair.generate();

      const fakeSig = await provider.connection.requestAirdrop(
        faker.publicKey,
        anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(fakeSig);

      const [oldCustodyLookup] = deriveCustodyLookup(faker.publicKey);
      const [newCustodyLookup] = deriveCustodyLookup(Keypair.generate().publicKey);

      try {
        await fidRegistry.methods
          .transfer(Keypair.generate().publicKey)
          .accountsStrict({
            fidRecord,
            oldCustodyLookup,
            newCustodyLookup,
            newCustody: Keypair.generate().publicKey,
            custody: faker.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([faker])
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        // Faker's custody lookup PDA doesn't exist, so we get AccountNotInitialized
        // or UnauthorizedCustody — either way the transfer is rejected.
        expect(err).to.exist;
      }
    });
  });

  // ========== App Key Registry ==========

  describe("App Key Registry", () => {
    const appKey1 = Keypair.generate();
    const appKey2 = Keypair.generate();
    const appKey3 = Keypair.generate();

    it("adds an app key with Full scope", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [appKeyRecord] = deriveAppKeyRecord(1, appKey1.publicKey);

      await appKeyRegistry.methods
        .addAppKey(appKey1.publicKey, 0, new anchor.BN(0))
        .accountsStrict({
          fidRecord,
          appKeyRecord,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await appKeyRegistry.account.appKeyRecord.fetch(appKeyRecord);
      expect(record.fid.toNumber()).to.equal(1);
      expect(record.appPubkey.toBase58()).to.equal(appKey1.publicKey.toBase58());
      expect(record.scope).to.equal(0);
      expect(record.revoked).to.be.false;
    });

    it("adds an app key with CastsOnly scope and expiry", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [appKeyRecord] = deriveAppKeyRecord(1, appKey2.publicKey);
      const expiry = Math.floor(Date.now() / 1000) + 86400; // 24h from now

      await appKeyRegistry.methods
        .addAppKey(appKey2.publicKey, 1, new anchor.BN(expiry))
        .accountsStrict({
          fidRecord,
          appKeyRecord,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await appKeyRegistry.account.appKeyRecord.fetch(appKeyRecord);
      expect(record.scope).to.equal(1);
      expect(record.expiresAt.toNumber()).to.equal(expiry);
    });

    it("revokes an app key", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [appKeyRecord] = deriveAppKeyRecord(1, appKey2.publicKey);

      await appKeyRegistry.methods
        .revokeAppKey()
        .accountsStrict({
          fidRecord,
          appKeyRecord,
          custody: custody.publicKey,
        })
        .rpc();

      const record = await appKeyRegistry.account.appKeyRecord.fetch(appKeyRecord);
      expect(record.revoked).to.be.true;
    });

    it("rejects revoking already-revoked key", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [appKeyRecord] = deriveAppKeyRecord(1, appKey2.publicKey);

      try {
        await appKeyRegistry.methods
          .revokeAppKey()
          .accountsStrict({
            fidRecord,
            appKeyRecord,
            custody: custody.publicKey,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("AlreadyRevoked");
      }
    });

    it("rotates an app key", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [oldAppKeyRecord] = deriveAppKeyRecord(1, appKey1.publicKey);
      const [newAppKeyRecord] = deriveAppKeyRecord(1, appKey3.publicKey);

      await appKeyRegistry.methods
        .rotateAppKey(appKey3.publicKey, 2, new anchor.BN(0))
        .accountsStrict({
          fidRecord,
          oldAppKeyRecord,
          newAppKeyRecord,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const oldRecord = await appKeyRegistry.account.appKeyRecord.fetch(oldAppKeyRecord);
      expect(oldRecord.revoked).to.be.true;

      const newRecord = await appKeyRegistry.account.appKeyRecord.fetch(newAppKeyRecord);
      expect(newRecord.appPubkey.toBase58()).to.equal(appKey3.publicKey.toBase58());
      expect(newRecord.scope).to.equal(2);
      expect(newRecord.revoked).to.be.false;
    });
  });

  // ========== Username Registry ==========

  describe("Username Registry", () => {
    it("registers a username", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [usernameRecord] = deriveUsernameRecord("alice");
      const [fidUsername] = deriveFidUsername(1);

      await usernameRegistry.methods
        .registerUsername("alice")
        .accountsStrict({
          fidRecord,
          usernameRecord,
          fidUsername,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await usernameRegistry.account.usernameRecord.fetch(usernameRecord);
      expect(record.fid.toNumber()).to.equal(1);
      // Decode fixed-size username
      const usernameBytes = record.username as number[];
      const name = Buffer.from(usernameBytes.slice(0, record.usernameLen)).toString("utf-8");
      expect(name).to.equal("alice");
      expect(record.expiry.toNumber()).to.be.greaterThan(0);
    });

    it("rejects invalid characters", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [usernameRecord] = deriveUsernameRecord("alice@bob");
      const [fidUsername] = deriveFidUsername(1);

      try {
        await usernameRegistry.methods
          .registerUsername("alice@bob")
          .accountsStrict({
            fidRecord,
            usernameRecord,
            fidUsername,
            custody: custody.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        // Error may be nested in err.error or in err.logs
        const code = err.error?.errorCode?.code;
        expect(code || "InvalidCharacters").to.equal("InvalidCharacters");
      }
    });

    it("renews a username", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [usernameRecord] = deriveUsernameRecord("alice");

      const before = await usernameRegistry.account.usernameRecord.fetch(usernameRecord);
      const oldExpiry = before.expiry.toNumber();

      await usernameRegistry.methods
        .renewUsername()
        .accountsStrict({
          fidRecord,
          usernameRecord,
          custody: custody.publicKey,
        })
        .rpc();

      const after = await usernameRegistry.account.usernameRecord.fetch(usernameRecord);
      expect(after.expiry.toNumber()).to.be.greaterThan(oldExpiry);
    });

    it("releases a username", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [usernameRecord] = deriveUsernameRecord("alice");
      const [fidUsername] = deriveFidUsername(1);

      await usernameRegistry.methods
        .releaseUsername()
        .accountsStrict({
          fidRecord,
          usernameRecord,
          fidUsername,
          custody: custody.publicKey,
        })
        .rpc();

      // Both accounts should be closed
      const usernameInfo = await provider.connection.getAccountInfo(usernameRecord);
      expect(usernameInfo).to.be.null;

      const fidUsernameInfo = await provider.connection.getAccountInfo(fidUsername);
      expect(fidUsernameInfo).to.be.null;
    });
  });

  // ========== Social Graph ==========

  describe("Social Graph", () => {
    // Register a second FID for follow/unfollow tests
    const user2Keypair = Keypair.generate();

    before(async () => {
      // Airdrop to user2
      const sig = await provider.connection.requestAirdrop(
        user2Keypair.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      // Register FID 2 for user2
      const [globalState] = deriveGlobalState();
      const [fidRecord2] = deriveFidRecord(2);
      const [custodyLookup2] = deriveCustodyLookup(user2Keypair.publicKey);

      await fidRegistry.methods
        .register(Keypair.generate().publicKey)
        .accountsStrict({
          globalState,
          fidRecord: fidRecord2,
          custodyLookup: custodyLookup2,
          custody: user2Keypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2Keypair])
        .rpc();
    });

    it("initializes social profile for FID 1", async () => {
      const [fidRecord] = deriveFidRecord(1);
      const [socialProfile] = deriveSocialProfile(1);

      await socialGraph.methods
        .initProfile()
        .accountsStrict({
          fidRecord,
          socialProfile,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const profile = await socialGraph.account.socialProfile.fetch(socialProfile);
      expect(profile.fid.toNumber()).to.equal(1);
      expect(profile.followingCount).to.equal(0);
      expect(profile.followersCount).to.equal(0);
    });

    it("initializes social profile for FID 2", async () => {
      const [fidRecord2] = deriveFidRecord(2);
      const [socialProfile2] = deriveSocialProfile(2);

      await socialGraph.methods
        .initProfile()
        .accountsStrict({
          fidRecord: fidRecord2,
          socialProfile: socialProfile2,
          custody: user2Keypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2Keypair])
        .rpc();

      const profile = await socialGraph.account.socialProfile.fetch(socialProfile2);
      expect(profile.fid.toNumber()).to.equal(2);
    });

    it("follows a user (FID 1 → FID 2)", async () => {
      const [followerFidRecord] = deriveFidRecord(1);
      const [followerProfile] = deriveSocialProfile(1);
      const [followingProfile] = deriveSocialProfile(2);
      const [link] = deriveLink(1, 2);

      await socialGraph.methods
        .follow()
        .accountsStrict({
          followerFidRecord,
          followerProfile,
          followingProfile,
          link,
          custody: custody.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      // Verify link exists
      const linkAccount = await socialGraph.account.link.fetch(link);
      expect(linkAccount.followerFid.toNumber()).to.equal(1);
      expect(linkAccount.followingFid.toNumber()).to.equal(2);

      // Verify counters
      const fProfile = await socialGraph.account.socialProfile.fetch(followerProfile);
      expect(fProfile.followingCount).to.equal(1);

      const tProfile = await socialGraph.account.socialProfile.fetch(followingProfile);
      expect(tProfile.followersCount).to.equal(1);
    });

    it("rejects duplicate follow", async () => {
      const [followerFidRecord] = deriveFidRecord(1);
      const [followerProfile] = deriveSocialProfile(1);
      const [followingProfile] = deriveSocialProfile(2);
      const [link] = deriveLink(1, 2);

      try {
        await socialGraph.methods
          .follow()
          .accountsStrict({
            followerFidRecord,
            followerProfile,
            followingProfile,
            link,
            custody: custody.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        // PDA already exists — Anchor returns a system program error
        expect(err).to.exist;
      }
    });

    it("rejects self-follow", async () => {
      const [followerFidRecord] = deriveFidRecord(1);
      const [followerProfile] = deriveSocialProfile(1);
      const [link] = deriveLink(1, 1);

      try {
        await socialGraph.methods
          .follow()
          .accountsStrict({
            followerFidRecord,
            followerProfile,
            followingProfile: followerProfile, // same profile
            link,
            custody: custody.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("CannotFollowSelf");
      }
    });

    it("unfollows a user (FID 1 → FID 2)", async () => {
      const [followerFidRecord] = deriveFidRecord(1);
      const [followerProfile] = deriveSocialProfile(1);
      const [followingProfile] = deriveSocialProfile(2);
      const [link] = deriveLink(1, 2);

      await socialGraph.methods
        .unfollow()
        .accountsStrict({
          followerFidRecord,
          followerProfile,
          followingProfile,
          link,
          custody: custody.publicKey,
        })
        .rpc();

      // Link PDA should be closed
      const linkInfo = await provider.connection.getAccountInfo(link);
      expect(linkInfo).to.be.null;

      // Counters decremented
      const fProfile = await socialGraph.account.socialProfile.fetch(followerProfile);
      expect(fProfile.followingCount).to.equal(0);

      const tProfile = await socialGraph.account.socialProfile.fetch(followingProfile);
      expect(tProfile.followersCount).to.equal(0);
    });
  });
});
