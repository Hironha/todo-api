import { type Result } from "@domain/utils/result";
import { type Id } from "@domain/value-objects/id";
import { type DeleteByIdError } from "@application/repositories/todo";

export type RemoveTodoInput = { id: Id };

export type RemoveTodoOutput = Result<void, RemoveTodoError>;

export type RemoveTodoError = DeleteByIdError;
