<script lang="ts">
    import { createField, getValue } from "felte";
    import Label from "./Label.svelte";
    import Error from "./Error.svelte";
    import { get, writable, type Writable } from "svelte/store";

    export let name: string;
    export let label: string;
    export let data : Omit<Writable<any>, "subscribe"> & {
    subscribe(
      subscriber: (values: { [x: string]: unknown }) => any
    ): () => void;
  } & Record<string, any>;

    const switchValue = writable('0')

    const { field, onInput, onBlur } = createField(name);

    data.subscribe((value) => {
        switchValue.set(getValue(value, name) as string)
    })

</script>

<div class="flex flex-col w-full">
    {#if label}
        <Label {name} {label} />
    {/if}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class={`w-[80px] h-[40px] rounded-full border-gray-500 border-[1px] cursor-pointer ${$switchValue == '1' ? "bg-green-200" :  "bg-red-200"}`}
    on:click={() => {
        console.log("Clicked", get(switchValue))
        if(get(switchValue) == '0'){
            switchValue.set('1')
            onInput('1')
        } else {
            switchValue.set('0')
            onInput('0')
        }
    }}>
        <div class={`w-[40px] h-[40px] bg-gray-500 rounded-full transition-all ${$switchValue == '1' ? "translate-x-[40px]" : ""}`}></div>
    </div>
    <input
        on:blur={onBlur}
        {name}
        use:field
        type="hidden"
        bind:value={$switchValue}
    />
    <Error {name} />
</div>
