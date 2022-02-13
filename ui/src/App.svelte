<script>
	import Ticker from "./Ticker.svelte";
	export let name;

	let showTicker = true;

	let ticker = new Napi.Ticker(42);
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>2 + 2 = {Napi.sum(2, 2)}</p>
	<button on:click={() => (showTicker = !showTicker)}>Toggle Ticker</button>
	<button on:click={() => ($ticker = Math.floor(Math.random() * 1234))}>
		Set to random number
	</button>
	{#if showTicker}
		<Ticker {ticker} />
	{/if}
	<p>Also: {$ticker}</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
