export function preventDefault(handler?: (event: Event) => void) {
    return (event: Event) => {
        event.preventDefault();
        handler?.(event);
    };
}

export function debounce<T extends (...args: any[]) => void>(
    fn: T,
    delay = 300
): (...args: Parameters<T>) => void {
    let timer: ReturnType<typeof setTimeout>;
    return (...args: Parameters<T>) => {
        clearTimeout(timer);
        timer = setTimeout(() => fn(...args), delay);
    };
}

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export function single(handler: (event: Event) => any | Promise<any>) {
    let running = false;

    return async (event: Event) => {
        if (running) return;

        running = true;

        const el = event.currentTarget as HTMLButtonElement;
        if (el) el.disabled = true;

        try {
            await handler(event);
            await sleep(500);
        } finally {
            running = false;
            if (el) el.disabled = false;
        }
    };
}