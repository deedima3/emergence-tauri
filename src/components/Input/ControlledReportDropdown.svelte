<script lang="ts">
  import type { CustomOption } from "@/interfaces/ui.interfaces";

  export let selectedValue: number | string;
  export let options: CustomOption[];
  export let placeholder: string = "";
</script>

<select
  class="border-2 flex-grow border-brand-red-main bg-white text-brand-red-main outline-none focus:outline-none rounded-md px-3 py-2 focus:border-brand-red-main"
  bind:value={selectedValue}
>
  <option value="" class="font-bold" disabled>
    {placeholder}
  </option>
  {#each options as option}
    <option class="" value={option.value}>
      {option.label}
    </option>
  {/each}
</select>
