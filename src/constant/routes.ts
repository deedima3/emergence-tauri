import { wrap } from "svelte-spa-router/wrap";
import Login from "@/routes/Auth/Login.svelte";
import Dashboard from "@/routes/Dashboard/Dashboard.svelte";
import NotFound from "@/routes/NotFound/NotFound.svelte";
import Register from "@/routes/Auth/Register.svelte";

const routes = {
  // Exact path
  "/": Login,
  "/register": Register,
  "/dashboard/": Dashboard,
  // Catch-all
  // This is optional, but if present it must be the last
  "*": NotFound
};  

export { routes };
