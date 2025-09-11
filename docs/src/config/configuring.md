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

## Properties

Even if you have the layout of your window ready, it wont really do anything unless we use properties.

To understand properties, lets go back to the previous example again.

From the previous example, lets focus on the `<Label text="boo"/>` element. See the text attribute? Yeah, that is the property of the widget.

`XmlMan` does not bother validating the attributes on its own as it would be very complex. Instead, it just directly transpiles that attribute to the appropriate rhai attribute.

For example, `<Label text="boo"/>` would get converted to `label(#{ text: "boo" })`.

To use properties efficiently, checkout the [widget properties section in ewwii docs](https://ewwii-sh.github.io/ewwii/widgets/props.html).

## Special properties

Now that you have learnt of properties, lets now learn of the special properties that is only valid in `XmlMan`.

These properties exist, because we can do something like this in rhai:

```js, ignore
enter([
  defwindow("example", #{
    // the following thing
    geometry: #{
        x: "0%",
        y: "2px",
        width: "90%",
        height: "30px",
        anchor: "top center"
    },
  }, label(#{ text: "example content" }))
])
```

Here we can see that the geometry property has nested properties within itself which is not possible to implement in xml.

So, to fix this issue, `XmlMan` intercepts special properties assigned to a `<Window>` and translates it to the rhai equivalent.
