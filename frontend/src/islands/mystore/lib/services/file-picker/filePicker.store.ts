import type { FileEntrySummary } from "@bindings/FileEntrySummary";
import { writable } from "svelte/store";

interface FilePickerState {
    isOpen: boolean;
    allowedMime?: string[];
    multiSelect: boolean;
    resolve?: (files: FileEntrySummary[] | FileEntrySummary | undefined) => void;
}

function createFilePickerStore() {
    const { subscribe, set, update } = writable<FilePickerState>({
        isOpen: false,
        multiSelect: false
    });

    async function getOne(mime?: string[]): Promise<FileEntrySummary | undefined> {
        return new Promise((resolve) => {
            update(state => ({
                ...state,
                isOpen: true,
                allowedMime: mime,
                multiSelect: false,
                resolve: (files) => {
                    if (Array.isArray(files)) {
                        resolve(files[0]);
                    } else {
                        resolve(files);
                    }
                }
            }));
        });
    }

    async function getMany(mime?: string[]): Promise<FileEntrySummary[] | undefined> {
        return new Promise((resolve) => {
            update(state => ({
                ...state,
                isOpen: true,
                allowedMime: mime,
                multiSelect: true,
                resolve: (files) => {
                    if (Array.isArray(files)) {
                        resolve(files);
                    } else {
                        resolve(files ? [files] : undefined);
                    }
                }
            }));
        });
    }

    function close(selectedFiles?: FileEntrySummary[] | FileEntrySummary) {
        update(state => {
            if (state.resolve) {
                state.resolve(selectedFiles);
            }
            return {
                ...state,
                isOpen: false,
                allowedMime: undefined,
                multiSelect: false,
                resolve: undefined
            };
        });
    }

    function cancel() {
        update(state => {
            if (state.resolve) {
                state.resolve(undefined);
            }
            return {
                ...state,
                isOpen: false,
                allowedMime: undefined,
                multiSelect: false,
                resolve: undefined
            };
        });
    }

    return {
        subscribe,
        getOne,
        getMany,
        close,
        cancel
    };
}

export const filePicker = createFilePickerStore();