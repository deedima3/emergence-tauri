<script lang="ts">
  import DarkBackground from "./DarkBackground.svelte";

  export let dragOver: boolean;
</script>

{#if dragOver}
  <DarkBackground onClose={() => {}}>
    <div class="w-full max-w-md border-main">
      <p class="text-5xl font-bold text-main">Drop File</p>
    </div>
  </DarkBackground>
{/if}
