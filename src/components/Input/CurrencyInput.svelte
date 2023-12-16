<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import { IMask } from "@imask/svelte";
  import { get, writable, type Writable } from "svelte/store";
  import type { ChangeEventHandler } from "svelte/elements";
  import { tick } from "svelte";
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

  const hiddenValue = writable("");
  const showedValue = writable("");
  const initialInput = writable(true);

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    if (e.currentTarget.value && e.currentTarget.value != get(showedValue) && get(initialInput) == false) {
      console.log("CurrentTarget", e.currentTarget.value);
      showedValue.set(`Rp. ${pipe(e.currentTarget.value.toString())}`);
      onChange(e.currentTarget.value.split(".").join(""));
    } else if (get(hiddenValue) == '') {
      console.log("Else Triggered")
      showedValue.set("");
    }
    initialInput.set(false);
  };

  // hiddenValue.subscribe(async (value) => {
  //   await tick();
  //   if(value && initialInput){
  //     showedValue.set(`Rp. ${pipe(value)}`)
  //     initialInput.set(false);
  //   }
  // })
  

  data.subscribe(async (data: any) => {
    await tick();
    if (getValue(data, name) && get(initialInput)) {
      console.log(`Rp. ${pipe(getValue(data, name).toString())}`);
      console.log("GetValue", getValue(data, name));
      hiddenValue.set(getValue(data, name).toString() || "");
      showedValue.set(`Rp. ${pipe(getValue(data, name).toString())}`);
    }
  });

  showedValue.subscribe((value) => {
    console.log("ShowedValue", value);
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
    name={`display_${name}`}
    bind:value={$showedValue}
    on:change={showedOnChange}
    {placeholder}
    {disabled}
  />
  <input type="hidden" bind:value={$hiddenValue} use:field {name} />
  <Error {name} />
</div>
