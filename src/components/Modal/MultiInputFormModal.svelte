<script lang="ts">
  import { BRAND_RED } from "@/constant/constant";
  import ArrowLeft from "../Icons/ArrowLeft.svelte";
  import DarkBackground from "./DarkBackground.svelte";

  export let open: boolean;
  export let onCancel: () => void;
  export let title: string;
  export let form: any;
</script>

{#if open}
  <DarkBackground onClose={onCancel}>
    <div
      class="flex flex-col bg-white px-10 py-9 rounded-md relative min-w-[800px] max-h-[80vh] overflow-y-scroll"
    >
      <div class="w-full flex flex-col gap-1 text-start">
        <button class="flex gap-2" on:click={onCancel}>
          <ArrowLeft currentColor={BRAND_RED} />
          <p class="text-brand-red-main">Kembali</p>
        </button>
        <p class="text-4xl font-bold">{title}</p>
      </div>
      <div class=" px-5 py-5 mt-5 w-full flex flex-col items-center">
        <form
          class="flex flex-col items-center justify-center gap-5 w-full"
          use:form
        >
          <slot />
        </form>
      </div>
    </div>
  </DarkBackground>
{/if}
