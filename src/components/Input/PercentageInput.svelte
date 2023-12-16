<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import { IMask, imask } from "@imask/svelte";
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
  export let maxValue = 100;

  const { field, onChange } = createField(name);

  const options = {
    mask: "num %",
    lazy: true,
    blocks: {
      num: {
        mask: Number,
        expose: true,
        thousandsSeparator: ".",
        radix: ", ",
        max: 100,
      },
    },
  };

  const pipe = IMask.createPipe(options);

  let showedValue = "";
  let hiddenValue = "";
  let showedTemp = "";

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    hiddenValue = e.currentTarget.value.replace("%", "");
    showedValue = `${pipe(hiddenValue.toString())}%`;
    showedTemp = `${pipe(hiddenValue.toString())}%`;
  };

  const onBlurShow = (e: FocusEvent) => {
    let parsedNumber = parseFloat(showedValue.replace("%", ""));
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
      showedValue = `${pipe((getValue(value, name) as string).toString())} %`;
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
