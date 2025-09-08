export function formatCurrency(amount: number, currency: string): string {
    const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency,
    });
    return formatter.format(amount);
}

export function formatDateTime(dateStr: string): string {
    const date = new Date(dateStr);
    const now = new Date();

    const isToday =
        date.getDate() === now.getDate() &&
        date.getMonth() === now.getMonth() &&
        date.getFullYear() === now.getFullYear();

    const yesterday = new Date(now);
    yesterday.setDate(now.getDate() - 1);

    const isYesterday =
        date.getDate() === yesterday.getDate() &&
        date.getMonth() === yesterday.getMonth() &&
        date.getFullYear() === yesterday.getFullYear();

    const time = date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

    if (isToday) return `Today at ${time}`;
    if (isYesterday) return `Yesterday at ${time}`;

    // Else full date & time in locale format, e.g. "Aug 12, 2025, 14:20"
    return date.toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    });
}

export function camelToTitleCase(camel?: string | null): string {
    if (camel === undefined || camel === null) return "N/A";
    return camel
        .replace(/([A-Z])/g, " $1")
        .replace(/\b\w/g, (char) => char.toUpperCase())
        .trim();
}

export function snakeToTitleCase(snake?: string | null): string {
    if (snake === undefined || snake === null) return "N/A";
    return snake
        .replace(/_/g, " ")
        .replace(/\b\w/g, (char) => char.toUpperCase())
        .trim();
}

export function orIf<C>(c: C): (val: C) => C | undefined;
export function orIf<C, O>(c: C, o: O): (val: C) => C | O;
export function orIf<C, O>(c: C, o?: O) {
    return (val: C) => (val === c ? o : val);
}

export function nullIf<C>(c: C): ReturnType<typeof orIf<C, null>> {
    return orIf(c, null);
}

export function isValidHref(value: string | undefined | null): boolean {
    if (!value) return true; // let Yup handle required()
    try {
        // absolute URL (http, https, mailto, etc.)
        new URL(value);
        return true;
    } catch {
        // relative path or hash link
        return value.startsWith("/") || value.startsWith("#");
    }
}