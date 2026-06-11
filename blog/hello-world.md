# Hello world

When creating a personal website, most people reach for React or Astro. I get it - they're popular,
well-documented, and everyone seems to be using them. But after fighting with both for longer than I'd
like to admit, I decided to try something different: generating plain static HTML with Rust.
Let me explain why.

## JavaScript ecosystem

React is bloated. The canonical example is the "Hello World" app - a single `<h1>` tag requires
a dedicated CLI tool just to bootstrap the project. After clicking through a billion options in
`create-react-app` (or its spiritual successor `vite`) you finally have a working setup.
Congratulations, you've spent 20 minutes creating a build system so that JavaScript can build
JavaScript. You haven't written a single line of your actual site yet.

Then there's JSX. Someone will always say:

> JSX is basically HTML, you can just paste your `.html` file into a React component and it works.

And then that same person will complain that your entire site isn't split into a hundred components,
each in its own file, with a barrel export and a dedicated `__tests__` folder.

## Using Astro.js

Astro looked promising. It's more ergonomic than React for content-heavy sites, and the island
architecture is genuinely clever. But then you hit the asset imports.

It's nice to write `import Logo from '../images/logo.png'` - until you need to do it dynamically.
Because this is special-cased syntax, it doesn't work in loops or with paths that aren't string
literals known at compile time. To import assets dynamically you're forced to use `import.meta.glob`,
which is its own kind of unpleasant:

```javascript
const images = import.meta.glob('../images/*.png');
```

It works, sort of. But the typechecker doesn't know what types the glob will return, so you end up
casting everything to `any` and losing all safety. If you know a clean solution to this, please
enlighten me.

## So Why Rust?

To be continued...
