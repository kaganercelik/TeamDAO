import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { decodeIdlAccount } from "@project-serum/anchor/dist/cjs/idl";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

describe("Voting and Distributing tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();
	const dan = anchor.web3.Keypair.generate();

	const program = anchor.workspace.TeamDao as Program<TeamDao>;

	let teamName = "Test Team 2";
	let uid = new anchor.BN(1234567);
	let teamAccountAddr;
	let voteAccountAddr;

	// the team addresses array
	let team = [alice, bob, carol, dan];

	let teamPda, teamBump;
	let votePda, voteBump;

	before(async () => {
		// creating account here because i will use it in other tests
		const ix = await program.methods.createTeam(teamName, uid);
		teamAccountAddr = (await ix.pubkeys()).teamAccount;
		const tx = await ix.rpc();

		[teamPda, teamBump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(teamName), Buffer.from(`${uid}`)],
			program.programId
		);

		const ix2 = await program.methods.initVote(teamAccountAddr);
		voteAccountAddr = (await ix2.pubkeys()).voteAccount;
		const tx2 = await ix2.rpc();

		[votePda, voteBump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(teamAccountAddr.toBytes())],
			program.programId
		);

		const ix3 = await program.methods.vote(teamAccountAddr, { yes: {} });
		const tx3 = await ix3.rpc();

		let voteAccount = await program.account.voteAccount.fetch(voteAccountAddr);
		console.log(voteAccount);

		// adding members to the team
		for (let i = 0; i < team.length; i++) {
			let ix = await program.methods
				.vote(teamAccountAddr, { yes: {} })
				.accounts({
					voteAccount: voteAccount,
					signer: team[i],
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.rpc();
		}
		voteAccount = await program.account.voteAccount.fetch(voteAccountAddr);

		console.log(voteAccount);
	});

	xit("should initilize the voting", async () => {
		const voteAccount = await program.account.voteAccount.fetch(
			voteAccountAddr
		);

		assert.equal(voteAccount.team.toBase58(), teamAccountAddr);
	});

	xit("should vote for a proposal", async () => {
		const ix = await program.methods.vote(teamAccountAddr, { yes: {} });
		const tx = await ix.rpc();

		const voteAccount = await program.account.voteAccount.fetch(
			voteAccountAddr
		);

		console.log(voteAccount);
	});
});
