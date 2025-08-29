# List of check errors

## [CE01]: Missing Root

This means that the `<Root>` element is missing in your xml configuration.

### Invalid xml config example:

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Window name="Potato">
    <Box>
        <Label text="Hi"/>
    </Box>
</Window>
```

This xml configuration is invalid because there is no `<Root>` element.

The `<Root>` element is mandatory because it defines the top-level container that the transpiler reads. Without it, the system cannot locate any content to process, so your code will fail to transpile.

### Valid xml config example:

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Root>
    <Window name="Potato">
        <Box>
            <Label text="Hi"/>
        </Box>
    </Window>
</Root>
```

This configuration is valid because the `<Root>` element is defined.
