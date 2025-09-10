# Configuring

Making a configuration with `XmlMan` is really easy and straightforward due to the declarative style of xml.

Just define what the UI looks like, and you will be done!

## Prerequisites

Before using `XmlMan`, you should have a **basic understanding of XML**, its structure, tags, and attributes. This documentation assumes you're already familiar with these concepts and focuses on how `XmlMan` extends or interacts with them.

If you're completely new to XML, it's worth exploring some tutorials or online resources to get a foundational grasp. Understanding XML will help you navigate `XmlMan` more effectively and avoid confusion.

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
