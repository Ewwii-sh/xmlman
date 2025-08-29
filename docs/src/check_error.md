# Check Errors in XmlMan

XmlMan performs several validation checks before compiling XML to Rhai.
If a check fails, you will receive a **check error** that helps identify the issue.

## What a Check Error Looks Like

A check error typically contains:

1. **Error level**: e.g., `[ERROR]`
2. **Check error ID**: e.g., `[CE01]`
3. **Message**: describes what went wrong

### Example

```log
[ERROR] [CE01] Enter not found in internal tree. A <Root> should exist in the xml markup.
```

Here’s what this means:

- `[ERROR]`: Severity level
- `[CE01]`: Check error ID, which you can reference in the documentation
- Message: Explains the missing `<Root>` element

## All check errors

Are you curious to learn about all check errors in **XmlMan**, or just want to look up a check error?

Checkout the [check error list](https://github.com/ewwii-sh/xmlman/blob/main/logs/CHECK_ERRORS.md).
