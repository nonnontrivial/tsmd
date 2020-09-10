# tsmd

![Rust](https://github.com/nonnontrivial/tsmd/workflows/Rust/badge.svg)

## Purpose

This is a CLI that generates `.md` documentation for `interface`s in `.ts` source files.

### Limitations

This project does not yet support the following desirable features

- resolve non-interface declarations into meaningful docs
- resolve generic parameters and contraints into meaninful docs
- allow vector of source `.ts` files
- aligned rows and columns in output files

### Installation & Guide

```shell
cargo install tsmd
```

The following command takes a relative path to some `.ts` file under the `-s` flag. It creates a `.md` file and fills it with a [table](https://www.markdownguide.org/extended-syntax#tables) for each `interface` in the source `.ts` file.

```shell
tsmd -s ./input.ts
```

When `./input.ts` looks like this:

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

`./input.md` will look like this: 

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
