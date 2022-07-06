export type AwaitedType<T> = T extends {
	then(onfulfilled?: (value: infer U) => unknown): unknown;
}
	? U
	: T;
