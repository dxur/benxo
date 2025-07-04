export interface PolicyContext<TSubject = any> {
    subject: TSubject;
    metadata?: Record<string, unknown>;
}

export interface PolicyResult {
    allowed: boolean;
    reason?: string;
    hints?: string[];
}

export interface Policy<TSubject = any> {
    readonly usecase: string; // can be anything and any unknown use case is a warning, eg: validate, visibility just a naming convension should be thout
    evaluate(context: PolicyContext<TSubject>): PolicyResult | Promise<PolicyResult>;
}