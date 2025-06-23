import { writable } from 'svelte/store';

export enum NotifType {
  SUCCESS = "success",
  INFO = "info",
  WARN = "warn",
  ERROR = "error",
}

interface Notif {
  id: number;
  msgType: NotifType;
  title: string;
  msg: string;
}

let counter = 0;

function createNotificationsStore() {
  const { subscribe, update } = writable<Notif[]>([]);

  function addNotif(msgType: NotifType, title: string, msg: string) {
    const newNotif: Notif = { id: counter++, msgType, title, msg };
    update(notifs => [...notifs, newNotif]);

    setTimeout(() => removeNotif(newNotif.id), 5000);
  }

  function removeNotif(id: number) {
    update(notifs => notifs.filter(notif => notif.id !== id));
  }

  function error(msg: any) {
    addNotif(NotifType.ERROR, "Error", `${msg || "An error occurred"}`);
  }

  function success(msg: any) {
    addNotif(NotifType.SUCCESS, "Success", `${msg || ""}`);
  }

  function info(msg: any) {
    addNotif(NotifType.INFO, "Info", `${msg || ""}`);
  }

  function warning(msg: any) {
    addNotif(NotifType.WARN, "Warning", `${msg || ""}`);
  }

  return {
    subscribe,
    addNotif,
    removeNotif,
    error,
    success,
    info,
    warning
  };
}

export const notifCenter = createNotificationsStore();

