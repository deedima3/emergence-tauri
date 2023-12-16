<script lang="ts">
  import { register } from "@/api/auth";
  import LoginButton from "@/components/Button/LoginButton.svelte";
  import PasswordInput from "@/components/Input/PasswordInput.svelte";
  import BackgroundLayout from "@/components/Layouts/BackgroundLayout.svelte";
  import LoadingPulse from "@/components/Loading/LoadingPulse.svelte";
  import { registerSchema } from "@/constant/schema";
  import { createMutationForm } from "@/hooks/createMutationForm";

  const {
    form: { form },
    mutation: { mutation },
  } = createMutationForm({
    mutationApi: register,
    formSchema: registerSchema,
    actionName: "Register",
    callbackRoute: "/dashboard",
  });
</script>

<BackgroundLayout>
  <div class="flex flex-col items-center justify-center w-full gap-2 mt-16">
    <img src="/Splash.png" alt="Splash Logo" class="w-60 h-60" />
    <h1 class="text-5xl text-white font-bold font-poppins">Login</h1>
    <h2 class="text-xl text-white font-light font-poppins">
      Masukkan Password Anda!
    </h2>
    <form class="flex flex-col w-full max-w-xs gap-2" use:form>
      <PasswordInput label="Password*" name="password" />
      <PasswordInput label="Encryption Key" name="encryption_key" />
      <LoginButton />
    </form>
  </div>
</BackgroundLayout>

<LoadingPulse isLoading={$mutation.isPending} />
