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
	});

	xit("should vote yes successfully", async () => {
		await program.methods
			.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
			.rpc();

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(
			teamDetails.votedPlayers[0].toString(),
			user.publicKey.toString()
		);
		assert.equal(teamDetails.yesVotes, 1);
	});

	xit("should set tournament address successfully", async () => {
		// remember it only sets the tournament address if the yes votes are more than 50%,
		// in this case teams will only have 5 members so 3 yes votes will be enough
		for (let i = 0; i < 3; i++) {
			await program.methods
				.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
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

	xit("should not increase yes votes", async () => {
		await program.methods
			.voteForTournament(teamName, uid, tournament.publicKey, { no: {} })
			.rpc();

		const teamDetails = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(teamDetails.yesVotes, 0);
	});

	xit("should not let a player vote twice", async () => {
		await program.methods
			.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
			.rpc();

		try {
			await program.methods
				.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
				.rpc();
		} catch (err) {
			assert.equal(
				err.error.errorMessage,
				"Member is already voted for the tournament"
			);
			assert.equal(err.error.errorCode.code, "AlreadyVotedError");
		}
	});

	xit("should not let vote for another tournament if there is still an active one", async () => {
		// voting for a tournament and starting the voting
		await program.methods
			.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
			.rpc();

		let anotherTournament = anchor.web3.Keypair.generate();
		try {
			// trying to vote for another tournament
			await program.methods
				.voteForTournament(teamName, uid, anotherTournament.publicKey, {
					yes: {},
				})
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
				"The team has an active tournament and cannot vote for another tournament, leave the current one first"
			);
			assert.equal(err.error.errorCode.code, "AlreadyActiveTournamentError");
		}
	});

	xit("should not let anybody that is not in the team to vote", async () => {
		let anotherUser = anchor.web3.Keypair.generate();
		try {
			await program.methods
				.voteForTournament(teamName, uid, tournament.publicKey, { yes: {} })
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
});
