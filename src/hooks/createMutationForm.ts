import { createMutation } from "@tanstack/svelte-query";
import toast from "svelte-french-toast";
import { validator } from '@felte/validator-zod';
import type { ZodSchema } from "zod";
import { translate } from "../utils/utils";
import { get } from "svelte/store";
import { push } from "svelte-spa-router";
import { createForm } from "felte";
import reporter from "@felte/reporter-tippy";
import type { ApiResponse } from "@/types/api.types";

interface MutationFormParams<T> {
    mutationApi: (data: T) => any,
    actionName: string,
    formSchema: ZodSchema,
    successFn?: (data: any) => void,
    errorFn?: (err: any) => void,
    refetch?: () => void,
    callbackRoute?: string,
    submitTransform? : (value : T) => any
}

export function createMutationForm<T>({ mutationApi, actionName, successFn, errorFn, refetch, callbackRoute, formSchema, submitTransform }: MutationFormParams<T>) {
    const { form, data, setData, setInitialValues, reset, setFields } = createForm({
        extend: [
            reporter(),
            validator({ schema: formSchema, level: 'error' }),
        ],
        onSubmit(values) {
            if(submitTransform){
                pushData(submitTransform(formSchema.parse(values)))
            } else {
                pushData(formSchema.parse(values))
            }
        },
    });

    const mutation = createMutation({
        mutationFn: (data : T) => {
            return mutationApi(data)
        },
        onSuccess: (data) => {
            toast.success(translate(`success.success`, {
                data: actionName
            }), {
                position: "bottom-center"
            })
            if (successFn) {
                console.log(data)
                successFn(data)
            }
            if (callbackRoute) {
                push(callbackRoute)
            }
            if (refetch) {
                refetch()
            }
        },
        onError: (err: Error | ApiResponse<unknown>) => {
            if (err.response) {
                const response = err.response.data as ApiResponse<unknown>;
                toast.error(translate(`api_error.${response.error.code}`, {
                    data: actionName
                }), {
                    position: "bottom-center"
                });
            }
            if (errorFn) {
                errorFn(err);
            }
        },
    })
    const mutate = get(mutation).mutate

    const pushData = (data: any) => {
        mutate(data)
    }

    return {
        form: {
            form,
            data,
            setData,
            setInitialValues,
            reset,
            setFields
        },
        mutation: {
            mutation
        }
    }
}