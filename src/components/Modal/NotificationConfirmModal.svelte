<script lang="ts">
  import DarkBackground from "@/components/Modal/DarkBackground.svelte";
  import OutlineButton from "../Button/OutlineButton.svelte";
  import Button from "../Button/Button.svelte";

  export let onContinue: () => void;
  export let onCancel: () => void;
  export let open: boolean;
</script>

{#if open}
  <DarkBackground onClose={onCancel}>
    <div
      class="px-16 py-8 bg-white flex flex-col gap-8 rounded-md items-center"
    >
      <p class="text-2xl font-bold max-w-sm text-center w-full">
        Apakah anda yakin ingin ?
      </p>
      <div class="w-full justify-around gap-6 flex">
        <OutlineButton onClick={onContinue}>Ya</OutlineButton>
        <Button onClick={onCancel}>Tidak</Button>
      </div>
    </div>
  </DarkBackground>
{/if}
