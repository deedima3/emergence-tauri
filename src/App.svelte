<script lang="ts">
  import "./app.css";
  import "@fontsource/dm-sans/400.css";
  import "@fontsource/dm-sans/500.css";
  import "@fontsource/dm-sans/700.css";
  import "@fontsource/dm-sans/900.css";
  import "@fontsource/poppins/400.css";
  import "@fontsource/poppins/500.css";
  import "@fontsource/poppins/700.css";
  import "@fontsource/poppins/900.css";
  import "tippy.js/dist/tippy.css";
  import { invoke } from "@tauri-apps/api/tauri";

  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { Toaster } from "svelte-french-toast";
  import Router from "svelte-spa-router";
  import { routes } from "./constant/routes";
  import { onMount } from "svelte";

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: 3,
      },
    },
  });

  onMount(() => {
    invoke("close_splashscreen");
  });
</script>

<QueryClientProvider client={queryClient}>
  <Router {routes} restoreScrollState={true} />
</QueryClientProvider>
<Toaster />
