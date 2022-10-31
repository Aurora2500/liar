<script type="ts">
	import Dice from '$lib/Dice.svelte';
import type { DiceSet } from 'src/types';
	
	export let name: string = "";
	export let dice: DiceSet = {type: 'unknown', length: 5};
</script>

<div class="card">
	<span class="name">{name}</span>
	<div class="dice-container">
		{#if dice.type === 'unknown'}
			{#each {length: dice.length} as _, i}
				<Dice/>
			{/each}
		{:else}
			{#each dice.dice as die}
				<Dice face={die}/>
			{/each}
		{/if}
	</div>
</div>

<style>
	.name {
		font-size: xx-large;
	}
	.card {
		border: 2px solid paleturquoise;
		border-radius: 1em;
		padding: 1em;
		width: 20em;
		height: 18em;
		text-align: center;
	}
	.dice-container {
		background-color: #21333E;
		border-radius: 16px;
		padding: 24px;
		margin-top: 3em;
		gap: 4px;
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
	}
</style>