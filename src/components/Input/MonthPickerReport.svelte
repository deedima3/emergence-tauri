<script lang="ts">
  import { writable, type Writable } from "svelte/store";
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField } from "felte";
  import dayjs from "dayjs";
  import "dayjs/locale/id";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;

  const { field, onInput, onBlur } = createField(name);

  export let filterMonth: Writable<string> = writable(
    dayjs().format("MMMM YYYY")
  );

  const handleDateChange = (event: any) => {
    const selectedDate = event.target.value;
    filterMonth.set(selectedDate);
  };
</script>

<div class="flex flex-col">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class={`border-2 border-brand-red-main w-full h-[30px] text-brand-red-main outline-none focus:outline-none rounded-md px-3 py-5 focus:border-brand-red-main ${
      disabled ? "bg-gray-300" : ""
    }`}
    type="month"
    {disabled}
    on:blur={onBlur}
    bind:value={$filterMonth}
    on:change={handleDateChange}
    {placeholder}
    {...field}
    contenteditable="true"
  />
  <Error {name} />
</div>

<style>
  ::-webkit-calendar-picker-indicator {
    background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="16" height="15" viewBox="0 0 24 24"><path fill="%23EE0979" d="M20 3h-1V1h-2v2H7V1H5v2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 18H4V8h16v13z"/></svg>');
  }
</style>
