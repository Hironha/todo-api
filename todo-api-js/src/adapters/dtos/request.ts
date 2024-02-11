import { type Issue } from "valibot";

import { Exception } from "@domain/utils/exception";
import { type Result } from "@domain/utils/result";

export abstract class Request<T extends {}> {
    constructor(protected src: { [K in keyof T]?: unknown }) {}

    abstract parse(): Result<T, ParseError>;
}

export type ParseErrorDetail = { key: string; message: string };

export class ParseError extends Exception<"ParseError"> {
    public readonly details: ParseErrorDetail[];

    constructor(details: ParseErrorDetail | ParseErrorDetail[]) {
        super("ParseError", "Failed parsing request");
        this.details = Array.isArray(details) ? details : [details];
    }

    static fromValibot(issues: Issue[]) {
        const details: ParseErrorDetail[] = issues.reduce((details, issue) => {
            if (issue.path) {
                const key = issue.path.map((item) => item.key).join(".");
                details.push({ key, message: issue.message });
            }
            return details;
        }, [] as ParseErrorDetail[]);

        return new ParseError(details);
    }
}
