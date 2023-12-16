<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import type { CustomOption } from "@/interfaces/ui.interfaces";
  import { select } from "radash";

  export let searchName: string;
  export let selectName: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled: boolean = false;
  export let search: string;
  export let options: CustomOption[] = [];
  export let setOptionValue: (value: number) => void;

  let focus: boolean = false;
  let mouseHover: boolean = false;

  $: popupOpen = focus || mouseHover;

  $: filteredOptions =
    search && search === "" && options
      ? options
      : options.filter(({ label }) => {
          if (label && search) {
            return label.toLowerCase().includes(search.toLowerCase());
          } else {
            return "";
          }
        });

  const getOptionName = (value: string | number) =>
    select(
      options,
      (f) => f.label,
      (f) => f.value == value
    );

  const onChangeData = (value: string) => {
    setOptionValue(parseInt(value));
    if (
      getOptionName(value) &&
      getOptionName(value).length != 0 &&
      getOptionName(value)[0]
    ) {
      console.log("OnChange Setted", getOptionName(value)[0]);
      search = getOptionName(value)[0] as string;
      popupOpen = false;
    }
  };

  function handleInput(e) {
    search = e.target.value;
  }

  function handleFocus() {
    focus = true;
  }

  function handleBlur() {
    focus = false;
  }
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label name={selectName} {label} />
  {/if}
  <div class="relative w-full">
    <input
      class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
        disabled ? "bg-gray-300" : ""
      }`}
      name={searchName}
      {placeholder}
      bind:value={search}
      on:change={handleInput}
      on:focus={handleFocus}
      on:blur={handleBlur}
      autocomplete="off"
      {disabled}
    />
    <input
      name={selectName}
      type="hidden"
      {disabled}
      on:change={(e) => {
        setOptionValue(e.target.value);
      }}
    />
    {#if popupOpen}
      <div
        class="top-10 absolute bg-white shadow-md rounded-md px-5 py-3 w-full z-[30]"
        on:mouseleave={() => {
          mouseHover = false;
        }}
        on:mouseenter={() => {
          mouseHover = true;
        }}
        role="dialog"
      >
        <div class="w-full flex flex-col gap-2 justify-start text-start">
          {#each filteredOptions as option}
            <button
              class="text-black font-base w-full text-start"
              on:click={() => {
                console.log("Value", option.value);
                onChangeData(option.value);
              }}
            >
              {option.label}
            </button>
          {/each}
        </div>
        <p class="text-sm mt-5">Ketik minimal 2 karakter untuk mencari data</p>
      </div>
    {/if}
  </div>
  <Error name={selectName} />
</div>
