export function isNone(value: any): boolean {
    return Object.keys(value).every(key => (value[key] === undefined || value[key] === null));
}

export function deepEqual(a: any, b: any) {
    if (a === b) return true;

    if (a == null || b == null || typeof a !== 'object' || typeof b !== 'object') {
        return false;
    }

    const keysA = Object.keys(a);
    const keysB = Object.keys(b);

    if (keysA.length !== keysB.length) return false;

    for (let key of keysA) {
        if (!keysB.includes(key) || !deepEqual(a[key], b[key])) {
            return false;
        }
    }

    return true;
}

export function formatDate(time: bigint) {
    const date = new Date(time);

    const options = {
        month: 'long',
        day: 'numeric',
        year: 'numeric',
        hour: 'numeric',
        minute: 'numeric',
    };

    return date.toLocaleString('en-US', options);
}
