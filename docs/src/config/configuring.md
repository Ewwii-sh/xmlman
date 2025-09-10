# Configuring

Making a configuration with `XmlMan` is really easy and straightforward due to the declarative style of xml.

Just define what the UI looks like, and you will be done!

## Prerequisites

The only requirement before using `XmlMan` is a basic idea of xml. Most of the documentation here expects that you know a bit of xml.

If you don't know anything about xml, then I suggest learning about it online.

## Basic Setup

```xml
<!-- The <Root> element is important! -->
<Root>
    <Window name="Foo">
        <Label text="boo"/>
    </Window>
</Root>
```

Let's analyze the above config.

At the top of this example, we can see a `<Root>` element (see [rules#rule-1](./rules#rule-1---use-a-root-element)) in which is a wrapper that **contains your configuration**. It is a mandotary requirement as xml can't handle multiple widgets at the top level.

And inside the root, we can see a `<Window>` element. This element is basically an application window to which you can add widgets to. And the `<Label>` element inside the window is a label widget which shows up inside our window when its opened.
