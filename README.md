# XmlMan

**XmlMan** is an elegant XML-to-Rhai transpiler for Ewwii, designed to make UI definitions and data structures concise, expressive, and scriptable. It combines the readability of XML with the flexibility of Rhai scripting, giving you precise error reporting without the struggle of Ewwii's Rahi API's.

## Example

### XML Input

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Window name="Banana">
    <Button label="Click me"/>
</Window>
```

### Transpiled Rhai Output

```rs
enter([
    defwindow("Banana", #{}, button(#{ label: "Click me" }))
]);
```
