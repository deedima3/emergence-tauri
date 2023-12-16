<script lang="ts">
  import Add from "../Icons/Add.svelte";

  export let onClick: () => void = () => {};
</script>

<button
  class="bg-brand-red-main px-5 py-2 flex gap-2 text-white font-bold rounded-md items-center h-full"
  on:click={onClick}
  type="button"
>
  <Add />
</button>
