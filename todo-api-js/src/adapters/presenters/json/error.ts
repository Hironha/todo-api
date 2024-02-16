import { type InternalError, type Exception } from "@domain/utils/exception";
import { type ParseError } from "@adapters/dtos/request";
import { type Json, type JsonView } from "./view";

export type JsonErrorBody = JsonView<{
    code: string;
    message: string;
    details?: Json;
}>;

export class JsonError {
    constructor(public readonly status: number, public readonly error: JsonErrorBody) {}

    static create(status: number, error: Exception): JsonError {
        return new JsonError(status, { code: error.kind, message: error.message });
    }

    static fromParse(err: ParseError): JsonError {
        return new JsonError(400, {
            code: err.kind,
            message: err.message,
            details: err.details,
        });
    }

    static fromInternal(err: InternalError): JsonError {
        return new JsonError(500, { code: err.kind, message: err.message });
    }
}
