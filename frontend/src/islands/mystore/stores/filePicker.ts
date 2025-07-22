import { writable } from 'svelte/store';

interface File {

};


function createFilePickerStore() {
  const { subscribe, update } = writable<string[]>([]);

  async function getOne(mime: string | undefined): Promise<File | undefined> {
    return undefined;
  }

  return {
    subscribe,
  };
}

export const filePicker = createFilePickerStore();
