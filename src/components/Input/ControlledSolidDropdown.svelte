<script lang="ts">
  import type { CustomOption } from "@/interfaces/ui.interfaces";

  export let selectedValue: number | string;
  export let options: CustomOption[];
  export let placeholder: string = "";
</script>

<select
  class="h-[50px] rounded-md px-3 py-2 text-white bg-brand-red-main focus:none text-base"
  bind:value={selectedValue}
>
  <option value="" class="font-bold text-white" disabled>
    {placeholder}
  </option>
  {#each options as option}
    <option class="text-white hover:bg-brand-red-main" value={option.value}>
      {option.label}
    </option>
  {/each}
</select>
