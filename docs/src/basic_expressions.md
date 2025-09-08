# Expressions

**XmlMan** supports writing Rhai expressions inside strings. This is both a powerful feature and a potential weakness.

## Why it's a superpower

This feature allows **XmlMan** to be dynamic rather than purely static, giving you more flexibility.

## Why it's a weakness

It’s a weakness because it allows arbitrary Rhai code. **XmlMan** cannot validate these expressions at parse time, so they may cause errors during runtime.

## How to use expressions

Expressions are written inside `${}`.

Example:

```xml
<?xml version="1.0" encoding="UTF-8"?>

<Root>
    <Window name="Banana">
        <Box>
            <!-- The expression inside ${} is a Rhai expression -->
            <Label text="${2 + 2}"/>
        </Box>
    </Window>
</Root>
```
