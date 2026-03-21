# Protected blocks

Protected blocks allow you use separate attributes for an inner scope.

They behave differently than regular blocks in the following ways:
1. They don't consume attributes. 
1. Attribute changes inside the block won't persist outside of the block.
1. They do not expand into duplicates of a parent block element.

## Syntax

Prefix a block with `@` to protect it.

```ranty
[rep: 3]
@{ A{B} }
{C}
# -> ABCCC
```

The protected block is still normalized internally, but it stays a single element from the parent's point of view.
