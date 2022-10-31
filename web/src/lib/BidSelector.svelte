<script type="ts">
	import Button from "@smui/button";
	import type { Bid, Variant } from "src/types";

	function is_bid_valid(curr: Bid, next: Bid, variant: Variant): boolean {
		switch(variant) {
			case "orthogonal":
				return (curr.face     == next.face     && curr.quantity < next.quantity) ||
				(curr.quantity == next.quantity && curr.face < next.face);
			case "raise-face":
				return (curr.face == next.face && curr.quantity < next.quantity) ||
					(curr.face <  next.face);
			case "raise-quantity":
				return (curr.quantity == next.quantity && curr.face < next.face) ||
					(curr.quantity <  next.quantity);
		}
	}

	export let variant: Variant = "raise-face";
	export let raise_bid: Bid;
	export let bid: Bid;
	export let total_dice: number = 15;

	$: valid = is_bid_valid(bid, raise_bid, variant);
	$: quantity_values = []
</script>

<div>
	<select bind:value={raise_bid.quantity} class:invalid="{!valid}">
	{#each {length: total_dice} as _, i}
			<option
			class:invalid="{!is_bid_valid(bid, {face: raise_bid.face, quantity: i+1}, variant)}"
			class:valid="{is_bid_valid(bid, {face: raise_bid.face, quantity: i+1}, variant)}"
			>{i+1}</option>
	{/each}
	</select>
	<select bind:value={raise_bid.face} class:invalid="{!valid}">
	{#each {length: 6} as _, i}
			<option
			class:invalid="{!is_bid_valid(bid, {quantity: raise_bid.quantity, face: i+1}, variant)}"
			class:valid="{is_bid_valid(bid, {quantity: raise_bid.quantity, face: i+1}, variant)}"
			>{i+1}</option>
	{/each}
	</select>

	<Button variant="outlined" disabled={!valid}
	on:click={() =>{ console.log(valid); }}>Raise bid</Button>
</div>

<style type="scss">
	.invalid {
		color: red;
	}
	.valid {
		color: black;
	}
</style>