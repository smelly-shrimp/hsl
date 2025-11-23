# HSL
_Html Static Linker_

It's not ready to work with :o

## Features
In those examples `res.html` is result of linking.

### `<include>`
Include html content from other files via `src` attribute.  
`index.html`:
```html
<include src="nav.html" />
<main>
    <h1>Buy Some Tea</h1>
</main>
```
`nav.html`:
```html
<nav>
    <h1>Tea Store</h1>
    <input type="search" placeholder="Search for tea">
</nav>
```

`res.html`:
```html
<nav>
    <h1>Tea Store</h1>
    <input type="search" placeholder="Search for tea">
</nav>
<main>
    <h1>Buy Some Tea</h1>
</main>
```

### `<children>`
(void) tag - include html content passed as children to `<include>`.  
`index.html`:
```html
<include src="tea.html">
    <h1>Earl Gray</h1>
</include>
```

`tea.html`:
```html
<div class="tea">
    <children>
</div>
```

`res.html`
```html
<div class="tea">
    <h1>Earl Gray</h1>
</div>
```

### `{attr-name}`
Following curly brace syntax, allows you to include _string_ content of passed
attributes to component.  

`index.html`
```html
<include src="tea_btn.html" name="Earl Gray" />
<include src="tea_btn.html" name="Black" />
<include src="tea_btn.html" name="Green" />
```

`tea_btn.html`
```html
<button>Buy {name}</button>
```

`res.html`
```html
<button>Buy Earl Gray</button>
<button>Buy Black</button>
<button>Buy Green</button>
```

## To do
I plan to add:
- `<match>` tag for matching/conditions;
- `<for>` tag for loops.
- escaping for `{}` syntax
- `watch` support
