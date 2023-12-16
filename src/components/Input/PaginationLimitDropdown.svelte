<script lang="ts">
  import { PAGINATION_LIMIT_OPTION } from "@/constant/constant";
  import type { ChangeEventHandler } from "svelte/elements";

  export let changePerPage: (limit: number) => void;

  const handleEvent: ChangeEventHandler<HTMLSelectElement> = (e) => {
    changePerPage(parseInt(e.currentTarget.value));
  };
</script>

<div class="flex gap-2 items-baseline">
  <p class="text-black text-base font-semibold">Show</p>
  <select
    class="w-[70px] h-[50px] rounded-md px-3 py-2 focus:border-brand-dark text-gray-600 bg-transparent text-sm font-semibold"
    on:change={handleEvent}
  >
    {#each PAGINATION_LIMIT_OPTION as option}
      <option value={option.value}>
        {option.label}
      </option>
    {/each}
  </select>
</div>
