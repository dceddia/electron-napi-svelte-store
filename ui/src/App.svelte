<script>
	import Ticker from "./Ticker.svelte";

	let showTicker = true;

	let ticker = new Napi.Ticker(42);
</script>

<main>
	<h1>Svelte store, built in Rust!</h1>
	<button on:click={() => (showTicker = !showTicker)}>Toggle Counter</button>
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
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		font-size: 2em;
		font-weight: 400;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
