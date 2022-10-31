<script lang="ts">
	export let face: number = 0;
</script>

{#if face === 0}
<div class="dice"/>
{:else if face === 1}
<div class="dice one">
	<span class="dot"/>
</div>
{:else if face === 2}
<div class="dice two">
	<span class="dot"/>
	<span class="dot"/>
</div>
{:else if face === 3}
<div class="dice three">
	<span class="dot"/>
	<span class="dot"/>
	<span class="dot"/>
</div>
{:else if face === 4}
<div class="dice four">
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
	</div>
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
	</div>
</div>
{:else if face === 5}
<div class="dice five">
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
	</div>
	<div class="column">
		<span class="dot"/>
	</div>
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
	</div>
</div>
{:else if face === 6}
<div class="dice six">
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
		<span class="dot"/>
	</div>
	<div class="column">
		<span class="dot"/>
		<span class="dot"/>
		<span class="dot"/>
	</div>
</div>
{/if}

<style type="scss">
	$white: #EDF2F6; 

	.dice {
		padding: 12px;
		border: 2px solid $white;
		background-color: #38454C;
		border-radius: 12px;
		width: 48px;
		height: 48px;
	}
	.dot {
		display: block;
		border-radius: 50%;
		background-color: $white;
		width: 12px;
		height: 12px;
	}
	.one, .two, .three, .four, .five, .six {
		display: flex;
	}
	.one {
		justify-content: center;
		align-items: center;
	}
	.two {
		justify-content: space-between;

		.dot:nth-of-type(2) {
			align-self: flex-end;
		}
	}
	.three {
		justify-content: space-between;
		.dot:nth-of-type(2) {
			align-self: center;
		}
		.dot:nth-of-type(3) {
			align-self: flex-end;
		}
	}
	.four, .six {
		justify-content: space-between;
		.column {
			display: flex;
			flex-direction: column;
			justify-content: space-between;
		}
	}
	.five {
		justify-content: space-between;
		.column {
			display: flex;
			flex-direction: column;
			justify-content: space-between;
		}
		.column:nth-of-type(2) {
			justify-content: center;
		}
	}
</style>