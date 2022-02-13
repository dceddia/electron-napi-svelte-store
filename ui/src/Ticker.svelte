<script>
  export let ticker;
</script>

<p>value is {$ticker ?? ""}</p>
<button on:click={() => ticker.increment()}>Increment</button>
