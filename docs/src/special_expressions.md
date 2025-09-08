# Special Expression: `@no_quote`

`XmlMan` provides a special expression called `@no_quote` that allows you to assign **non-string values** (like booleans, integers, or other raw data) to element attributes. Normally, attribute values are wrapped in quotes and interpreted as strings. Using `@no_quote` tells the transpiler **not** to quote the value.

### How It Works

In `XmlMan`, XML attributes are normally **always quoted**, which means values are treated as strings. Sometimes, you want to assign **raw values** like booleans or numbers (without quotes). The `@no_quote` expression makes this possible.

#### XML Example

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Root>
    <Window name="Banana">
        <Box>
            <!-- @no_quote(false) ensures this is a boolean, not a string -->
            <Label text="foo" visible="@no_quote(false)"/>
        </Box>
    </Window>
</Root>
```

Here, `visible="@no_quote(false)"` assigns a **boolean `false`** instead of the string `` `false` ``.

#### Transpiled Rhai Code

When the above XML is transpiled, it produces the following Rhai code:

```js
fn Banana_child() {
    box(#{}, [
      label(#{
        "text": `foo`, // string
        "visible": false // boolean
      })
    ])
}

enter([
    defwindow("Banana", #{  }, Banana_child())
])
```

> **Important**
>
> If you write `@no_quote(val) some_other_val`, the engine will **still** wrap the entire value in quotes. Only the direct use of `@no_quote(val)` ensures the value remains unquoted.

> **Caution**
>
> When using `@no_quote`, `XmlMan` cannot guarantee that the resulting code is valid. Make sure the values you pass are appropriate for the context.
