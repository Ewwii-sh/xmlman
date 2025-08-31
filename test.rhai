fn Banana_child() {
  box(#{  }, [label(#{ "text": `${2 + 2}` })])
}

enter([
  defwindow("Banana", #{ "test": `a` }, Banana_child())
])