fn Banana_child() {
    box(#{}, [
      label(#{
        "text": `foo`,
        "visible": false
      })
    ])
  }

enter([
  defwindow("Banana", #{  }, Banana_child())
])