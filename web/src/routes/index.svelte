<script lang="ts">
	import Button from '@smui/button';
	import Textfield, {} from '@smui/textfield';

	import Dice from '$lib/Dice.svelte'
	import { goto } from '$app/navigation';
	
	let join_code: string = "";

	const newGame = async () => {
		let res = await fetch('http://localhost:4000/new_game', {
			method: 'POST'
		});
		let code: string = (await res.json() as any).data.code;
		goto(`/game/${code}`)
	}

	function joinGame() {
		goto(`/game/${join_code}`)
	}
</script>

<h1>Liar</h1>
<div class="vertical">
	<Button
		on:click={newGame}
		variant="outlined">
			Start a new game
	</Button>
	<Textfield
		bind:value={join_code}
		variant="outlined"
		label="code"/>
	<Button
		on:click={joinGame}
		variant="outlined">
		Join an existing game
	</Button>
	<a href="/about">About</a>
</div>

<style>
	.vertical {
		margin: auto;
		padding: 60px;
		width: 20%;
		gap: 20px;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
	h1 {
		text-align: center;
		font-family: 'Noto Serif', serif;
	}
</style>