<script>
  export let counter;
</script>

<p>value is {$counter ?? ""}</p>
<button on:click={() => counter.increment()}>Increment</button>
