<script type="ts">
	import {page} from '$app/stores';
	import { onMount } from 'svelte';
	import Button from '@smui/button';
	
	import PlayerCard from '$lib/PlayerCard.svelte';
	import Dice from '$lib/Dice.svelte';
	import type { Bid, Player } from 'src/types';
	import BidSelector from '$lib/BidSelector.svelte';
	import Textfield, {} from '@smui/textfield';

	const code: string = $page.params.code;

	let ws: WebSocket;

	let nickname: string = "Guest";
	
	let started: boolean = false;

	let players: Array<Player> = [{
		name: 'puny',
		dice: {type: 'unknown', length: 3}
	}, {
		name: 'kat',
		dice: {type: 'unknown', length: 5}
	}, {
		name: 'pwa',
		dice: {type: 'unknown', length: 2}
	}];
	let bid: Bid = {
		quantity: 7,
		face: 5,
	};


	let raise_bid: Bid = {
		quantity: 3,
		face: 4,
	};

	let self: Array<number> = [3, 3, 4, 2, 6]

	$: total_dice =
		self.length +
		players.reduce<number>((s, {dice:d}) => s + (d.type=='unknown'?d.length:d.dice.length), 0);

	const set_nickname = () => {
		ws.send(JSON.stringify({action: 'set_name', name: nickname}))
	}

	onMount(() => {
		ws = new WebSocket(`ws://localhost:4000/game/${code}/ws`);
		ws.onopen = (ev) => {
			console.log("Open!");
		}

		ws.onmessage = (ev: MessageEvent<any>) => {
			ev.data
		}
	})
</script>

<!--BidSelector bind:open={raise_bid_open} bid={bid}/-->


{#if started}

<div class="game">
	<div class="player-list">
		{#each players as player}
			<PlayerCard {...player}/>
		{/each}
	</div>
	<div class="game-bar">
		<div class="raise-bid">
			<BidSelector
				variant="orthogonal"
				bind:raise_bid bid={bid}
				total_dice={total_dice}/>
		</div>
		<div class="current-bid">
			<span class="header">Current bid</span>
			<div class="bid">
				<span class="quantity">{bid.quantity}</span>
				X
				<Dice face={bid.face}/>
			</div>
		</div>
		<div>
		<Button variant="outlined">Challenge</Button>
		</div>
	</div>
	<div class="self">
		{#each self as die}
			<Dice face={die} />
		{/each}
	</div>
</div>

{:else}
<div class="lobby">
	<h2>Lobby</h2>
	<div>
		<div>

		</div>
		<div>

		</div>
	</div>
	<div class="footer">
		<Textfield bind:value={nickname}
		variant="outlined"/>
		<Button on:click={set_nickname} variant="outlined">Set nickname</Button>
	</div>
</div>
{/if}

<style type="scss">
	.player-list {
		margin: auto;
		display: flex;
		flex-flow: row wrap;
		justify-content: center;
		padding: 1em;
		gap: 2em;
	}
	.game-bar {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-around;
		margin: 5em 0;
		padding: 0;
		width: 100%;
		height: 12em;
		background-color: #21333E;
		.current-bid {
			height: 90%;
			display: flex;
			flex-direction: column;
			justify-content: space-around;
			align-items: center;
			.header {
				font-size: 200%;
			}
			.bid {
				display: flex;
				align-items: center;
				gap: 5px;
				font-size: 200%;
				.quantity {
					display: block;
					font-size: 200%;
				}
			}
		}
	}
	.self {
		display: flex;
		flex-flow: row nowrap;
		margin: auto;
		width: 60%;
		gap: 6px;
		justify-content: center;
	}
	.lobby {
		display: flex;
		flex-direction: column;
		margin: auto;
		width: 60%;
		justify-content: center;
		outline: red solid 1px;

		h2 {
			text-align: center;
		}

		.footer {
			display: flex;
			justify-content: center;
		}
	}
</style>