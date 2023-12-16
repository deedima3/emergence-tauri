<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import { IMask } from "@imask/svelte";
  import { type Writable } from "svelte/store";
  import type { ChangeEventHandler } from "svelte/elements";
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

  const options = {
    mask: "Rp. num",
    lazy: true,
    blocks: {
      num: {
        mask: Number,
        expose: true,
        thousandsSeparator: ".",
        radix: ", ",
      },
    },
  };

  const pipe = IMask.createPipe(options);

  let showedValue = "";
  let hiddenValue = "";
  let showedTemp = "";

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    hiddenValue = e.currentTarget.value.split(".").join("");
    showedValue = `Rp. ${pipe(hiddenValue.toString())}`;
    showedTemp = `Rp. ${pipe(hiddenValue.toString())}`;
  };

  const onBlurShow = (e: FocusEvent) => {
    let parsedNumber = parseInt(showedValue.split(".").slice(1).join(""));
    if (!isNaN(parsedNumber)) {
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
      showedValue = `Rp. ${pipe((getValue(value, name) as string).toString())}`;
    }
  });

  $: console.log("ShowedValue", showedValue);
  $: console.log("HiddenValue", hiddenValue);
  $: console.log("TempShowValue", showedTemp);
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
  <input type="hidden" bind:value={hiddenValue} use:field {name} />
  <Error {name} />
</div>
