import { type InternalError, type Exception } from "@domain/utils/exception";

import { type ParseError } from "@adapters/dtos/request";

export type JsonErrorBody = {
    code: string;
    message: string;
    details?: unknown;
};

export class JsonError {
    constructor(public readonly status: number, public readonly error: JsonErrorBody) {}

    static create(status: number, error: Exception): JsonError {
        return new JsonError(status, { code: error.name, message: error.message });
    }

    static fromParse(err: ParseError): JsonError {
        return new JsonError(400, {
            code: err.name,
            message: err.message,
            details: err.details,
        });
    }

    static fromInternal(err: InternalError): JsonError {
        return new JsonError(500, { code: err.name, message: err.message });
    }
}
