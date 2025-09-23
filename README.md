# XmlMan

**XmlMan** is an elegant XML-to-Rhai transpiler for Ewwii, designed to make UI definitions and data structures concise, expressive, and scriptable. It combines the readability of XML with the flexibility of Rhai scripting, giving you precise error reporting without the struggle of Ewwii's Rahi API's.

## Install

XmlMan is available in the [eii-manifests](https://github.com/Ewwii-sh/eii-manifests). So you can use eiipm to to install it:

```bash
$ eiipm i xmlman
```

## Example

### XML Input

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Root>
    <Window name="Banana">
        <Button label="Click me"/>
    </Window>
</Root>
```

### Transpiled Rhai Output

```rs
fn Banana_child() {
    button(#{ "label": `Click me` })
}

enter([
    defwindow("Banana", #{  }, Banana_child())
])
```
