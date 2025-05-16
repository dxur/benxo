export function getOid(id: any): string {
    return id.$oid;
}

export function isNone(value: any): boolean {
    return Object.keys(value).every(key => (value[key] === undefined || value[key] === null));
}
