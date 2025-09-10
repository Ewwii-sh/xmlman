# Configuration Rules

These rules ensure that the structure and logic of your configuration remain flexible, safe, and prone to less runtime errors.

Currently there are only 2 rules, but as `XmlMan` matures, many more rules may appear.

## Rule 1 - Use a `<Root>` Element

Always wrap all widgets and configuration elements within a single `<Root>` element. This safely aligns with ewwii's model and also allows having multiple elements in a configuration because in xml, we may only have root 1 element.

```xml
<Root>
    <Window name="1">
        <!-- ... -->
    </Window>

    <Window name="2">
        <!-- ... -->
    </Window>
</Root>
```

## Rule 2 - Avoid Orphaned Poll/Listen Handlers

Poll and Listen handlers must always be defined at the root level and not within other widgets like `<Box>` or `<Window>`. Ewwii ignores orphaned handlers but `XmlMan` rejects it directly for safe configuration.

```xml
<Root>
    <!-- valid -->
    <Poll var="valid"></Poll>
    <Window name="1">
        <Box>
            <!-- orphan -->
            <Poll var="orphan"></Poll>
        </Box>
    </Window>
</Root>
```
