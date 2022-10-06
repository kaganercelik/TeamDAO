import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MethodsBuilder } from "@project-serum/anchor/dist/cjs/program/namespace/methods";
import { assert } from "chai";
import { TeamDao } from "../target/types/team_dao";

xdescribe("Voting and Distributing tests", () => {
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

	// the team addresses array
	let team = [alice.publicKey, bob.publicKey, carol.publicKey, dan.publicKey];

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

		// adding members to the team
		for (let i = 0; i < team.length; i++) {
			const ix = await program.methods.addMember(teamName, uid, team[i]);
			const tx = await ix.rpc();
		}
	});

	it("should create a team successfully", async () => {
		const teamAccount = await program.account.teamAccount.fetch(
			teamAccountAddr
		);

		const team = await program.account.teamAccount.fetch(teamAccountAddr);

		console.log(team);
	});
});
