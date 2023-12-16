<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import type { Writable } from "svelte/store";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;
  export let queryObj: Writable<Record<string, any>>;

  let search = "";
  let timeout: string | number | NodeJS.Timeout | undefined;

  const handleSearch = () => {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(assignSearch, 300);
  };

  const assignSearch = () => {
    queryObj.set({
      keyword: search,
    });
  };
</script>

<div class="flex flex-col w-full h-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class="border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark"
    {name}
    {placeholder}
    {disabled}
    bind:value={search}
    on:input={handleSearch}
  />
  <Error {name} />
</div>
