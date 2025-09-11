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

export function single(handler: (event: Event) => any | Promise<any>) {
    let running = false;
    return async (event: Event) => {
        if (running) return;
        running = true;
        try {
            await handler(event);
        } finally {
            running = false;
        }
    };
}