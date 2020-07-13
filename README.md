# tsmd

![Rust](https://github.com/nonnontrivial/tsmd/workflows/Rust/badge.svg)

## Purpose

This is a CLI that generates `.md` documentation for interfaces in `.ts` source files.

### Installation & Guide

```shell
cargo install tsmd
```

Running the following command will create `./input.md` and fill it with a [markdown table](https://www.markdownguide.org/extended-syntax#tables) for each `interface` in `./input.ts`.

```shell
tsmd -s ./input.ts
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

```md
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
```
