import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { decodeIdlAccount } from "@project-serum/anchor/dist/cjs/idl";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

describe("Voting tests", () => {
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

	let teamName = "Test Team 2";
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
		for (let i = 0; i < team.length; i++) {
			await program.methods.addMember(teamName, uid, team[i].publicKey).rpc();
		}

		// initing tournament
		await program.methods
			.initTournament(teamName, uid, tournament.publicKey, new anchor.BN(100))
			.rpc();
	});

	it("should vote yes successfully", async () => {
		await program.methods.voteForTournament(teamName, uid, { yes: {} }).rpc();

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(
			teamDetails.votedPlayers[0].toString(),
			user.publicKey.toString()
		);
		assert.equal(teamDetails.yesVotes, 1);
	});

	it("should set tournament address successfully", async () => {
		// remember it only sets the tournament address if the yes votes are more than 50%,
		// in this case teams will only have 5 members so 3 yes votes will be enough
		// since we voted for captain already, we need to vote for 2 more members
		for (let i = 0; i < 2; i++) {
			await program.methods
				.voteForTournament(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc();
		}

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(
			teamDetails.activeTournament.toBase58(),
			tournament.publicKey.toBase58()
		);
		assert.equal(teamDetails.votingResult, true);
	});

	it("should not increase yes votes", async () => {
		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(teamDetails.yesVotes, 0);
	});

	it("should not init another tournament if there is still an active one", async () => {
		let anotherTournament = anchor.web3.Keypair.generate();
		try {
			// trying to vote for another tournament
			await program.methods
				.initTournament(
					teamName,
					uid,
					anotherTournament.publicKey,
					new anchor.BN(100)
				)
				.rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"The team has an active tournament and cannot vote for another tournament, leave the current one first"
			);
			assert.equal(err.error.errorCode.code, "AlreadyActiveTournamentError");
		}
	});

	it("should let a team leave the tournament", async () => {
		// leaving the tournament
		// still, 3 votes for leaving the tournament is enough because of majority reasons and more than 3 votes will send an error
		for (let i = 0; i < 3; i++) {
			await program.methods
				.leaveTournament(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc();
		}

		let teamDetails = await program.account.teamAccount.fetch(teamAccountAddr);
		assert.equal(
			teamDetails.activeTournament.toBase58(),
			"11111111111111111111111111111111" // Pubkey::default()
		);
	}); // testing for errors seemed unnecessary because almost the same errors are tested in the previous tests
});
