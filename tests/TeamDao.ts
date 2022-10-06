import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

describe("TeamDao", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();

	const program = anchor.workspace.TeamDao as Program<TeamDao>;

	let teamName = "Test Team";
	let uid = new anchor.BN(1234567890);
	let teamAccountAddr;

	let teamPda, teamBump;

	before(async () => {
		[teamPda, teamBump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(teamName), Buffer.from(`${uid}`)],
			program.programId
		);

		// creating account here because i will use it in other tests
		const ix = await program.methods.createTeam(teamName, uid);
		teamAccountAddr = (await ix.pubkeys()).teamAccount;

		const tx = await ix.rpc();
	});

	it("should create a team successfully", async () => {
		const teamAccount = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(teamAccount.name, teamName);
	});

	it("should add a member to the team", async () => {
		let teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		let teamLength = teamAccount.members.length;

		const ix = await program.methods.addMember(teamName, uid, bob.publicKey);
		const tx = await ix.rpc();

		teamAccount = await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(teamAccount.members.length, teamLength + 1);
		assert.equal(
			teamAccount.members[teamAccount.members.length - 1].toBase58(),
			bob.publicKey.toBase58()
		);
	});
});
