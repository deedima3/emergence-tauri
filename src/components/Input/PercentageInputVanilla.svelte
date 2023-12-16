<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import type { ChangeEventHandler } from "svelte/elements";
  import type { Writable } from "svelte/store";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;
  export let data: Omit<Writable<any>, "subscribe"> & {
    subscribe(
      subscriber: (values: { [x: string]: unknown }) => any
    ): () => void;
  } & Record<string, any>;

  const { field, onChange } = createField(name);

  let showedValue = "";
  let hiddenValue = "";
  let showedTemp = "";

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    let parsedNumber = parseFloat(showedValue.replace("%", ""));
    if (!isNaN(parsedNumber) && parsedNumber <= 100) {
      hiddenValue = e.currentTarget.value.replace("%", "");
      showedValue = `${hiddenValue.toString()}%`;
      showedTemp = `${hiddenValue.toString()}%`;
    }
  };

  const onBlurShow = (e: FocusEvent) => {
    let parsedNumber = parseFloat(showedValue.replace("%", ""));
    if (!isNaN(parsedNumber) && parsedNumber <= 100) {
      onChange(parsedNumber);
    }
    showedValue = showedTemp;
  };

  const onDataFocus = () => {
    showedValue = hiddenValue;
  };

  data.subscribe((value) => {
    hiddenValue = getValue(value, name) as string;
    if (getValue(value, name)) {
      showedValue = `${(getValue(value, name) as string).toString()}%`;
    }
  });
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : ""
    }`}
    bind:value={showedValue}
    on:change={showedOnChange}
    on:blur={onBlurShow}
    on:focus={onDataFocus}
    {placeholder}
    {disabled}
  />
  <input type="hidden" bind:value={hiddenValue} {name} use:field />
  <Error {name} />
</div>
