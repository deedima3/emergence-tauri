<script lang="ts">
    export let currentColor = "#ffffff";
    export let extraClass = "";
    export let strokeWidth = 1.5;
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    stroke-width={strokeWidth}
    stroke={currentColor}
    class={extraClass ? extraClass : "w-5 h-5"}
    viewBox="0 0 20 20"
    aria-hidden="true"
>
    <path
        d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z"
    />
</svg>
