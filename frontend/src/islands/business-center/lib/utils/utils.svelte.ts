export function useState<T>(val: T): T {
    let proxy = $state(val);
    return proxy;
}