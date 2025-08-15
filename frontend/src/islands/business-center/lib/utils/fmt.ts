export const currencyFormatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "DZD",
});

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

export function camelToTitleCase(camel: string) {
    return camel
        .replace(/([A-Z])/g, ' $1')
        .replace(/\b\w/g, char => char.toUpperCase())
        .trim();
}

export function snakeToTitleCase(snake: string) {
    return snake
        .replace(/_/g, ' ')
        .replace(/\b\w/g, char => char.toUpperCase())
        .trim();
}