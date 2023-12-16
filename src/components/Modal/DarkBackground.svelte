<script lang="ts">
  import Portal from "svelte-portal";
  import { expoInOut } from "svelte/easing";
  import { scale } from "svelte/transition";

  export let onClose: () => void;
</script>

<Portal target="body">
  <div class="fixed top-0 w-full h-full min-h-screen min-w-screen z-[25]">
    <div class="flex min-h-screen min-w-screen items-center justify-center">
      <div
        class="z-[25] absolute top-0 min-h-screen w-full bg-black opacity-80"
        on:click={onClose}
      />
      <div class="z-[25]" transition:scale={{ easing: expoInOut }}>
        <slot />
      </div>
    </div>
  </div>
</Portal>
