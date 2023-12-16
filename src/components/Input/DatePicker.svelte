<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField } from "felte";
  import dayjs from "dayjs";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;

  const { field, onInput, onBlur } = createField(name);

  const parseDate = (parsableDate: Date) => {
    return dayjs(parsableDate).format("YYYY-MM-DD");
  };

  function handleInput(e) {
    onInput(parseDate(e.currentTarget.innerText));
  }
</script>

<div class="flex flex-col">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : ""
    }`}
    {disabled}
    on:input={handleInput}
    on:blur={onBlur}
    {name}
    {placeholder}
    {...field}
    type="date"
    lang="id-ID"
  />
  <Error {name} />
</div>
