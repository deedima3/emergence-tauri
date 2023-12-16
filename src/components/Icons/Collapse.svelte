<script lang="ts">
    export let currentColor = "#ffffff";
    export let extraClass = "";
    export let strokeWidth = 1.5;
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    stroke-width={strokeWidth}
    stroke={currentColor}
    class={extraClass ? extraClass : "w-5 h-5"}
    viewBox="0 0 20 20"
    fill="none"
    aria-hidden="true"
>
    <path
        fill-rule="evenodd"
        d="M4 10a.75.75 0 01.75-.75h10.5a.75.75 0 010 1.5H4.75A.75.75 0 014 10z"
        clip-rule="evenodd"
    />
</svg>
