<script lang="ts">
  import { downloadFile, uploadFile } from "@/api/filesystem/file";
  import { getExtension } from "@/utils/utils";
  import { createField, getValue } from "felte";
  import { createEventDispatcher, onMount } from "svelte";
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";

  import toast from "svelte-french-toast";
  import { get, writable, type Writable } from "svelte/store";
  import Button from "../Button/Button.svelte";
  import type { DragEventHandler } from "svelte/elements";
  import DocumentUpload from "../Icons/DocumentUpload.svelte";
  import { BRAND_RED } from "@/constant/constant";
  import saveFile from "@/utils/download";

  export let fileTypes: string[];
  export let name: string;
  export let disabled: boolean = false;
  export let label: string;
  export let fileSize: number;
  export let data: Omit<Writable<any>, "subscribe"> & {
    subscribe(
      subscriber: (values: { [x: string]: unknown }) => any
    ): () => void;
  } & Record<string, any>;

  const dispatch = createEventDispatcher();
  const { field, onChange } = createField(name);

  const filename = writable("");
  const file = writable<FileList>();

  const onTypeError = () => {
    toast.error(
      `Format file tidak didukung, pilih file dengan format ${fileTypes.toString()}`
    );
  };

  const onFileSizeLarger = (fileSize: number) => {
    toast.error(`File tidak boleh lebih besar dari ${fileSize / 1000} MB`);
  };

  const onFileMoreThanOne = () => {
    toast.error(`File tidak boleh lebih dari satu!`);
  };

  const handleDropFile: DragEventHandler<HTMLDivElement> = async (e) => {
    e.preventDefault();
    e.stopPropagation();

    if (!e.dataTransfer) {
      return;
    }

    const { files } = e.dataTransfer;

    console.log(files);

    if (files && files.length) {
      if (files.length > 1) {
        onFileMoreThanOne();
        return;
      }
      if (
        !fileTypes.includes(getExtension(files[0].name as string) as string)
      ) {
        onTypeError();
        return;
      }
      if (files[0].size / 1000 > fileSize) {
        onFileSizeLarger(fileSize);
        return;
      }
    //   onChange(files);
      filename.set(files[0].name);
      file.set(files);
    }
  };

  const handleDragOver = (e: {
    preventDefault: () => void;
    stopPropagation: () => void;
  }) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleOnClick = () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = fileTypes
      .map((value) => {
        return `.${value}`;
      })
      .join(",");
    input.onchange = async (_) => {
      // you can use this method to get file and perform respective operations
      if (input.files) {
        const files: File[] = Array.from(input.files);
        console.log("Files", files);
        if (files && files.length) {
          if (files.length > 1) {
            onFileMoreThanOne();
            return;
          }
          if (files[0] == null || undefined) {
            return;
          }
          if (!fileTypes.includes(getExtension(files[0].name) as string)) {
            onTypeError();
            return;
          }
          if (files[0].size / 1000 > fileSize) {
            onFileSizeLarger(fileSize);
            return;
          }
          onChange(input.value);
          filename.set(files[0].name);
          file.set(input.files);
        }
      }
    };
    input.click();
  };

  data.subscribe((data: any) => {
    filename.set(getValue(data, name));
  });
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="w-full flex flex-col gap-5 items-center">
    <div
      class="flex w-full flex-col items-center justify-center border-2 border-dashed border-gray-500 bg-gray-200 px-5 py-5"
      on:drop={handleDropFile}
      on:dragover={handleDragOver}
      on:click={handleOnClick}
    >
      {#if $filename}
        <div class="flex w-full gap-5">
          <div class="w-full flex flex-col items-center">
            <DocumentUpload currentColor={BRAND_RED} />
            <p class="text-center">File Uploaded</p>
            <p class="text-center text-sm">{$filename}</p>
          </div>
        </div>
      {:else}
        <div class="w-full flex flex-col items-center">
          <DocumentUpload currentColor={BRAND_RED} />
          <p class="text-xl">
            Seret file kesini atau,
            <span class="font-bold text-brand-red-main">Cari!</span>
          </p>
          <p class="text-sm">
            File yang disupport : {fileTypes.join(" ")}
          </p>
        </div>
      {/if}
      <input
        type="file"
        bind:files={$file}
        bind:value={$filename}
        use:field
        {name}
      />
    </div>
    {#if $filename}
      <Button onClick={() => saveFile(get(file)[0], $filename)}
        >Download File</Button
      >
    {/if}
  </div>
  <Error {name} />
</div>
