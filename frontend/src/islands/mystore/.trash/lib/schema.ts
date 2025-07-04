import type { Component } from "svelte";
import type { Policy } from "./policy";

export interface SchemaContext {
    data: Record<string, any>;
    update: (path: string[], value: any) => void;
    get: (path: string[]) => any;
    subscribe: (listener: () => void) => () => void;
    watch: (path: string[], fn: (value: any) => void) => () => void;
    emit: (type: string, payload?: any) => void;
    on: (type: string, handler: (payload: any) => void) => () => void;
    collect: (type: string) => void;
}

export interface FormSchema {
    title: string;
    description?: string;
    policies?: Policy[];
    sections: SectionSchema[];
    hooks?: SchemaHooks;
}

export interface SectionSchema {
    placement?: "nav" | "main" | "aside";
    component?: Component<{ content?: SectionContent, context: SchemaContext }>;
    content?: SectionContent;
}

export interface SectionContent {
    title?: string;
    description?: string;
    hints?: string;
    policies?: Policy[];
    fields?: FieldSchema[];
}

export interface FieldSchema {
    content?: FieldContent;
    component?: Component<{ content?: FieldContent, context: SchemaContext }>;
}

export interface FieldContent {
    key: string;
    title: string;
    label: string;
    type: string;
    required?: boolean;
    pattern?: string;
    options?: string[];
    policies?: Policy[];
}

export interface SchemaHooks {
    onFieldChange?: (path: string[], value: any, context: SchemaContext) => void;
    onSubmit?: (data: Record<string, any>, context: SchemaContext) => void;
    onAction?: (type: string, payload?: any, context?: SchemaContext) => void;
    onMessage?: (type: string, payload?: any, from?: string) => void;
}

