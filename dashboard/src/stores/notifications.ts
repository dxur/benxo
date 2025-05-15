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

    function error(msg: string) {
        addNotif(NotifType.ERROR, "Error", msg);
    }

    function success(msg: string) {
        addNotif(NotifType.SUCCESS, "Success", msg);
    }

    function info(msg: string) {
        addNotif(NotifType.INFO, "Info", msg);
    }

    function warn(msg: string) {
        addNotif(NotifType.WARN, "Warning", msg);
    }

    return {
        subscribe,
        addNotif,
        removeNotif,
        error,
        success,
        info,
        warn
    };
}

export const notifCenter = createNotificationsStore();

