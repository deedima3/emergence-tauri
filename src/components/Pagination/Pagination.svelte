<script lang="ts">
  import { DOTS } from "@/constant/constant";
  import type { PaginationData } from "@/interfaces/api.interfaces";
  import type { Writable } from "svelte/store";
  import PaginationNumber from "./PaginationNumber.svelte";

  export let paginationStore: Writable<PaginationData>;
  export let siblingCount = 1;
  export let nextPage: () => void;
  export let previousPage: () => void;
  export let setPage: (page: number) => void;

  const { floor, min, max } = Math;
  const range = (lo, hi) => Array.from({ length: hi - lo }, (_, i) => i + lo);

  const pagination = (
    count: number,
    ellipsis = "…",
    page: number,
    total: number
  ) => {
    const start = max(1, min(page - floor((count - 3) / 2), total - count + 2));
    const end = min(total, max(page + floor((count - 2) / 2), count - 1));
    return [
      ...(start > 2 ? [1, ellipsis] : start > 1 ? [1] : []),
      ...range(start, end + 1),
      ...(end < total - 1 ? [ellipsis, total] : end < total ? [total] : []),
    ];
  };

  $: paginationRange = pagination(
    4,
    "...",
    $paginationStore.page,
    $paginationStore.max as number
  );

  $: console.log("PaginationRange", paginationRange);
</script>

<div class="flex justify-between items-center">
  {#if $paginationStore.page > 1}
    <button class="p-2" on:click={previousPage}>
      {"<"}
    </button>
  {/if}
  <div class="flex gap-5">
    {#each paginationRange as pageNumber}
      <PaginationNumber {pageNumber} {setPage} currentPage={$paginationStore.page} />
    {/each}
  </div>
  {#if !($paginationStore.max == $paginationStore.page) && $paginationStore.max != 0}
    <button class="p-2" on:click={nextPage}> > </button>
  {/if}
</div>
