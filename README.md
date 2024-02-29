# swc-plugin-react-auto-classnames

Automatically adds class names to all React components. Ex:

```ts
const MyComponent = () => <Component />;
```
will be transformed to

```ts
const MyComponent = () => <Component className="file-name-component" />;
```

## Installation

```bash
npm i -D swc-plugin-react-auto-classnames
```

Add plugin to wherever you have an SWC config (e.g. `.swcrc` file, `swc-loader` config, etc).

```js
// JavaScript
{
  jsc: {
    parser: {
      jsx: true,
    },
    experimental: {
      plugins: [
        ['swc-plugin-react-auto-classnames', {}],
      ],
    },
  },
}

// TypeScript
{
  jsc: {
    parser: {
      syntax: 'typescript',
      tsx: true,
    },
    experimental: {
      plugins: [
        ['swc-plugin-react-auto-classnames', {}],
      ],
    }
  },
}
```

## License

MIT
