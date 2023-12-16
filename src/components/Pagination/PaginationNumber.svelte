<script lang="ts">
  import { DOTS } from "@/constant/constant";

  export let setPage: (page: number) => void;
  export let pageNumber: number | string;
  export let currentPage: number;

  const handleSelectPage = () => {
    if (typeof pageNumber == "string") {
      setPage(parseInt(pageNumber));
    } else {
      setPage(pageNumber);
    }
  };
</script>

{#if pageNumber == DOTS && typeof pageNumber == "string"}
  <div class="">
    {pageNumber}
  </div>
{:else}
  <button
    class={`${
      pageNumber == currentPage ? "bg-brand-red-main text-white" : "bg-white"
    } rounded-full p-3 w-12 h-12 shadow-md`}
    on:click={handleSelectPage}
  >
    {pageNumber}
  </button>
{/if}
