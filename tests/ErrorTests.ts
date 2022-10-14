import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { decodeIdlAccount } from "@project-serum/anchor/dist/cjs/idl";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

describe("Error tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();
	const dan = anchor.web3.Keypair.generate();

	let tournament = anchor.web3.Keypair.generate();

	const program = anchor.workspace.TeamDao as Program<TeamDao>;

	let teamName = "Test Team 3";
	let uid = new anchor.BN(1234567);
	let teamAccountAddr;

	// the team addresses array
	let team = [alice, bob, carol, dan];

	let teamPda, teamBump;

	before(async () => {
		// creating account here because i will use it in other tests
		const ix = await program.methods.createTeam(teamName, uid);
		teamAccountAddr = (await ix.pubkeys()).teamAccount;
		const tx = await ix.rpc();

		[teamPda, teamBump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(teamName), Buffer.from(`${uid}`)],
			program.programId
		);

		// adding team members
		// only 4 member added to the team because i ll test  the last one for captain error
		for (let i = 0; i < team.length - 1; i++) {
			await program.methods.addMember(teamName, uid, team[i].publicKey).rpc();
		}

		// initing tournament
		await program.methods
			.initTournament(teamName, uid, tournament.publicKey, new anchor.BN(100))
			.rpc();
	});

	it("should not let a player vote twice", async () => {
		try {
			await program.methods.voteForTournament(teamName, uid, { yes: {} }).rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"Member is already voted for the tournament"
			);
			assert.equal(err.error.errorCode.code, "AlreadyVotedError");
		}
	});

	it("should not let anybody that is not in the team to vote", async () => {
		let anotherUser = anchor.web3.Keypair.generate();
		try {
			await program.methods
				.voteForTournament(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: anotherUser.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([anotherUser])
				.rpc();
		} catch (err) {
			assert.equal(err.error.errorMessage, "Member is not in the team");
			assert.equal(err.error.errorCode.code, "MemberNotInTeamError");
		}
	});

	it("should not let anyone else other than captain add member", async () => {
		try {
			await program.methods
				.addMember(teamName, uid, dan.publicKey)
				.accounts({
					teamAccount: teamAccountAddr,
					signer: alice.publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([alice])
				.rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"Only captain can call this function"
			);
			assert.equal(err.error.errorCode.code, "NotCaptainError");
		}
	});
});
