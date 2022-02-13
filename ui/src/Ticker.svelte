<script>
  let ticker = new Napi.Ticker(42);
</script>

<p>value is {$ticker}</p>
<button on:click={() => ticker.increment()}>Increment</button>
