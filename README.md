# swc-plugin-react-auto-classnames

Automatically adds class names to all React components. Ex:

```ts
const MyComponent = () => <Component />;
```
will be transformed to

```ts
const MyComponent = () => <Component className="file-name-component" />;
```

## Building

The .wasm binary is included in the root directory, which includes the functionality.
To rebuild this binary, you can use `cargo build-wasi --release`. This generates a file
under target/ - in order to commit this to a release you can copy to root with
`cp target/wasm32-wasi/release/swc_plugin_react_auto_classnames.wasm .`. Ensure if
releasing to bump the version strings in both the package.json (for the npm package)
and the Cargo.toml file.

## License

MIT
