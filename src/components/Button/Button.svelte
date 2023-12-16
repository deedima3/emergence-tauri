<script lang="ts">
  export let onClick: () => void = () => {};
  export let icon: string = "";
  export let type: "submit" | "reset" | "button" = "submit";
</script>

<button
  class="bg-brand-red-main w-[200px] px-5 py-2 flex gap-2 text-white font-bold rounded-md"
  {type}
  on:click={onClick}
>
  <div class="w-full text-base font-roboto">
    <slot />
  </div>
  {#if icon}
    <div class="w-5">
      <img src={icon} alt="Button Icon" />
    </div>
  {/if}
</button>
