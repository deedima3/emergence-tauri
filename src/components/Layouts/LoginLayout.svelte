<script lang="ts">
  import Setting from "../Icons/Setting.svelte";
  import BackgroundLayout from "./BackgroundLayout.svelte";

</script>

<BackgroundLayout>
  <div class="w-full flex flex-col items-center justify-center">
    <slot />
  </div>
  <div class="absolute bottom-5 right-5">
    <Setting currentColor="#FFFFFF" extraClass="w-8 h-8" />
  </div>
</BackgroundLayout>
