import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { decodeIdlAccount } from "@project-serum/anchor/dist/cjs/idl";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

describe("Distribution tests", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const user = provider.wallet;

	const alice = anchor.web3.Keypair.generate();
	const bob = anchor.web3.Keypair.generate();
	const carol = anchor.web3.Keypair.generate();
	const dan = anchor.web3.Keypair.generate();
	let team = [alice, bob, carol, dan];

	let tournament = anchor.web3.Keypair.generate();

	const program = anchor.workspace.TeamDao as Program<TeamDao>;

	let teamName = "Test Team 4";
	let uid = new anchor.BN(1234567);
	let teamAccountAddr;

	// the team addresses array

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

		// creating tournament
		// voting for 3 members
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

		await program.provider.connection.confirmTransaction(
			await program.provider.connection.requestAirdrop(
				alice.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 101
			)
		);
	});

	it("should init percentage proposal successfully", async () => {
		let proposalPercentages = [30, 10, 20, 15, 25];
		await program.methods
			.initPercentageProposal(teamName, uid, Buffer.from(proposalPercentages)) // buffering the data
			.rpc();

		let { distributionPercentages: distPerc } =
			await program.account.teamAccount.fetch(teamAccountAddr);

		// checking if the send array and the fetched array has the same values
		let isArrayEqual = proposalPercentages.every(
			(element, index) => element === distPerc[index]
		);

		assert.equal(isArrayEqual, true);
	});

	it("should let players vote for percentages successfully", async () => {
		// voting for 3 members
		for (let i = 0; i < 3; i++) {
			await program.methods
				.distributionProposalHandler(teamName, uid, { yes: {} })
				.accounts({
					teamAccount: teamAccountAddr,
					signer: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc();
		}

		let { distributionVotingResult: distResult } =
			await program.account.teamAccount.fetch(teamAccountAddr);

		assert.equal(distResult, true);
	});

	it("should be able to set canJoinTournament successfully", async () => {
		await program.methods.canJoinTournament(teamName, uid).rpc();

		let { canJoinTournament } = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		assert.equal(canJoinTournament, true);
	});

	it("should distribute prizes successfully", async () => {
		let tournamentPrize = anchor.web3.LAMPORTS_PER_SOL * 100;
		let reward;

		// airdrop for teamAccount
		await program.provider.connection.confirmTransaction(
			await program.provider.connection.requestAirdrop(
				teamAccountAddr,
				anchor.web3.LAMPORTS_PER_SOL * 101
			)
		);

		// get the balance of teamAccount
		let teamAccountBalance = await program.provider.connection.getBalance(
			teamAccountAddr
		);

		// air drop for user
		await program.provider.connection.confirmTransaction(
			await program.provider.connection.requestAirdrop(
				user.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 10
			)
		);

		// getting the distribution percentage of team[i] from program
		let { distributionPercentages: distPerc } =
			await program.account.teamAccount.fetch(teamAccountAddr);

		// get the balance of user
		let userBalance =
			(await program.provider.connection.getBalance(user.publicKey)) /
			anchor.web3.LAMPORTS_PER_SOL;

		// calculating the reward for user
		reward = (tournamentPrize * distPerc[0]) / 100;

		await program.methods
			.claimReward(teamName, uid, new anchor.BN(reward))
			.accounts({
				teamAccount: teamAccountAddr,
				from: teamAccountAddr,
				to: user.publicKey,
				user: user.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc()
			.catch((err) => console.log(err));

		// get the balance of user after transaction
		let userBalanceAfter =
			(await program.provider.connection.getBalance(user.publicKey)) /
			anchor.web3.LAMPORTS_PER_SOL;

		let result = Math.ceil(userBalanceAfter - userBalance);

		// assert equal user balances
		assert.equal(result, reward / anchor.web3.LAMPORTS_PER_SOL);

		// airdrop for team
		for (let i = 0; i < team.length; i++) {
			await program.provider.connection.confirmTransaction(
				await program.provider.connection.requestAirdrop(
					team[i].publicKey,
					anchor.web3.LAMPORTS_PER_SOL * 10
				)
			);

			// get the balance of team[i]
			let teamMemberBalance = await program.provider.connection.getBalance(
				team[i].publicKey
			);

			reward = (tournamentPrize * distPerc[i + 1]) / 100; // i + 1 because the first element of the array is user account that i used above

			// claim the reward for alice
			await program.methods
				.claimReward(teamName, uid, new anchor.BN(reward))
				.accounts({
					teamAccount: teamAccountAddr,
					from: teamAccountAddr,
					to: team[i].publicKey,
					user: team[i].publicKey,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.signers([team[i]])
				.rpc()
				.catch((err) => console.log(err));

			// get the balance of team[i] after claiming the reward
			let teamMemberBalanceAfterTx =
				await program.provider.connection.getBalance(team[i].publicKey);

			assert.equal(teamMemberBalanceAfterTx, teamMemberBalance + reward);
		}
	});
});
