<script lang="ts">
  export let onClick: () => void = () => {};
  export let icon: string = "";
  export let type: "submit" | "reset" | "button" = "submit";
</script>

<button
  class="border-brand-red-main border-[1px] text-xl px-5 py-2 flex gap-2 w-[200px] text-white font-bold rounded-md"
  {type}
  on:click={onClick}
>
  <div class="w-full text-xl text-brand-red-main font-roboto">
    <slot />
  </div>
  {#if icon}
    <div class="w-5">
      <img src={icon} alt="Button Icon" />
    </div>
  {/if}
</button>
