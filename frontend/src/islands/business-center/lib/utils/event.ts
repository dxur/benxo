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