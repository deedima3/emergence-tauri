<script lang="ts">
  import type { CustomOption } from "@/interfaces/ui.interfaces";

  export let selectedValue: number | string;
  export let options: CustomOption[];
</script>

<select
  class="border-[1px] border-gray-400 w-[200px] h-[50px] rounded-md px-3 py-2 focus:border-brand-dark text-black"
  bind:value={selectedValue}
>
  {#each options as option}
    <option value={option.value}>
      {option.label}
    </option>
  {/each}
</select>
