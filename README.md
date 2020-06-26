# tsmd

![Rust](https://github.com/nonnontrivial/tsmd/workflows/Rust/badge.svg)

> generate markdown tables from TypeScript interfaces

## Status

Hobby project in early development for auto-generating docs for TypeScript files.

### Todos

- account for generics
- parse non `interface`s
- user config

### Example

> **Note**: This is not published to Cargo yet. `git clone git@github.com:nonnontrivial/tsmd.git` first.

Running the following command will create `./input.md` and fill it with [markdown tables](https://www.markdownguide.org/extended-syntax#tables) according to the `interface`s in `./input.ts`.

```shell
cargo r -- -s ./input.ts

```

When `./input.ts` looks like this,

```typescript
export interface Output<A extends LowLevelAsset<unknown>> {
  id: string;
  timestamp: string;
  data: Omit<A, "">;
}

export interface LowLevelAsset<D> {
  filename?: string;
  filenames?: string[];
  data: D;
}

```

`./input.md` will look like the following.

## Output

| Field | Type |
| --- | --- |
| timestamp | `string` |
| id | `string` |
| data | `Omit<A, "">` |
## LowLevelAsset

| Field | Type |
| --- | --- |
| filenames? | `string[]` |
| filename? | `string` |
| data | `D` |
