<script lang="ts">
  export let text: string = "";
  export let icon: any;
  export let extraClass: string = "";
  export let type: "submit" | "reset" | "button" = "submit";
  export let onClick: () => void = () => {};
  export let currentColor: string = "#ffffff";
  export let design : "solid" | "outline" = "solid";
</script>

{#if design === "solid"}
  <button
    {type}
    on:click={onClick}
    class="bg-brand-red-main flex items-center gap-3 py-2.5 px-4 rounded-md hover:bg-brand-red-main-shade"
  >
    <span class="font-semibold text-white text-xs">{text}</span>
    {#if icon}
      <svelte:component this={icon} currentColor={currentColor} {extraClass} />
    {/if}
  </button>
{:else if design === "outline"}
  <button
    {type}
    on:click={onClick}
    class="border-2 border-brand-red-main hover:bg-brand-red-ground flex items-center gap-3 py-2.5 px-4 rounded-md"
  >
    <span class="font-semibold text-brand-red-main text-xs">{text}</span>
    {#if icon}
      <svelte:component this={icon} currentColor={currentColor} {extraClass} />
    {/if}
  </button>
{/if}
