<script>
	import Counter from "./Counter.svelte";

	let showCounter = true;

	let counter = new Napi.Counter(42);
</script>

<main>
	<h1>Svelte store, built in Rust!</h1>
	<button on:click={() => (showCounter = !showCounter)}>Toggle Counter</button>
	<button on:click={() => ($counter = Math.floor(Math.random() * 1234))}>
		Set to random number
	</button>
	{#if showCounter}
		<Counter {counter} />
	{/if}
	<p>Also: {$counter}</p>
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
