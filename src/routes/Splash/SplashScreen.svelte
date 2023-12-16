<script lang="ts">
  import BackgroundLayout from "@/components/Layouts/BackgroundLayout.svelte";
  import "../../app.css";
  import "@fontsource/poppins/400.css";
  import "@fontsource/poppins/500.css";
  import "@fontsource/poppins/700.css";
  import "@fontsource/poppins/900.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  onMount(() => {
    invoke("open_splashscreen");
  });
</script>

<BackgroundLayout>
  <div class="flex flex-col items-center justify-center w-full gap-2 mt-24">
    <img src="/Splash.png" alt="Splash Logo" class="w-96 h-96" />
    <h1 class="text-6xl text-white font-bold font-poppins">Emergence</h1>
    <h2 class="text-xl font-poppins text-white font-light">
      Personal Data Encryption App
    </h2>
  </div>
</BackgroundLayout>
