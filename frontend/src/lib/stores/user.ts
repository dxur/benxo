import type { UserDto } from "@bindings/UserDto";
import { writable } from "svelte/store";

export const userStore = writable(undefined as UserDto | undefined);
