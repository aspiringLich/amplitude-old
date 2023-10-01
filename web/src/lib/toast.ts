import type { ToastStore, ToastSettings } from "@skeletonlabs/skeleton";
import { getToastStore } from "@skeletonlabs/skeleton";

export class Toaster {
    toastStore: ToastStore;

    constructor() {
        this.toastStore = getToastStore();
    }
    
    error(message: string) {
        this.toastStore.trigger({
            message,
            background: "bg-error-300-600-token",
            classes: "error",
        });
    }
    
    success(message: string) {
        this.toastStore.trigger({
            message,
            background: "bg-success-300-600-token",
            classes: "success",
        });
    }
}

export function getToaster() {
    return new Toaster();
}