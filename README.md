# BFMacro
A simple macro language. Designed to make writing brainf\*ck programs, a little less f\*cky.

This project still tries to keep the original spirit, by just formally defining the common patterns you would normally use. This is a successor to my old macro system for https://github.com/BenJilks/BF.

## Getting Started
First, start by defining the memory layout of you program. Create a `frame` that gives a name to each memory cell.

```
frame Main {
    a b c
}
```

Now we can crating a `using` block to enter this frame. While in a frame, you can use the normal bf operations. Except for `>` and `<`, as this is considered 'manual movement' and is not allowed in fixed frame mode (See more about this in 'Moving Blocks'). In order to move the pointer, you must use the name of the cell you wish to move the pointer to.

```
using Main {
    a +
    c ++
    b +++
}

# Compiles to: "+>>++<+++"
```

### Macros
A macro allows you define a reusable bit of code. You provide a set of parameters, and the body will act like a using block on a frame consisting of those parameters.

```
macro move(src, dest) {
    src[
        dest+
        src-
    ]
}
```

To invoke a macro, call it providing the arguments. The arguments you provide will keep there same position in the frame, when the macro is evaluated.

```
frame Main { a b c }

using Main {
    copy(b, c)
}

# Compiles to: ">[<+>-]"
```

### Macro Block Arguments
In addition to named cells, you can also provide block as arguments to macros. When referenced by the macro, it will evaluate the block in the original frame context it was created in.

```
macro while(a, do) {
    a[
        do
        a
    ]
}

frame Main { a b c }

using Main {
    while(b, {
        a+
        b-
    })
}

# Compiles to: ">[<+>+]"
```

### Sub-frames
You can annotate a cell in a frame, to have a sub-frame. This will make the size of that cell the size of that sub-frame, and allow you to access its cells using the `.` operator. You are able to have nested sub-frames, and pass them to macros as arguments.

```
frame Vector2 {
    x y
}

frame Main {
    a
    vec: Vector2
    b
}

using Main {
    vec.x +
    vec.y ++
    b +++
}

# Compiles to: ">+>++>+++"
```

### Moving Blocks
See https://bytesizedben.com/advanced-brainf_k for more information about the 'moving head' pattern.

In order to do manual 'memory movement', you must do so from within a `moving` block. It's very important to note that, you must exit the `moving` block at the same pointer position as you entered. This is because the pointer position must be known at compile time for named cell access to work. Consequently, you cannon't access named variable from within a `moving` block.

```
using Main {
    a
    moving {
        >>+>+
        <<<
    }
    b
}
```

You can start a `using` block from inside a `moving` one. This allows you to do named operations on the local frame, being moved.

```
using Main {
    # Setup
    # ...

    moving {
        using LocalFrame {
            a +
            b -
        }

        # Move frame
        # ...
    }
}
```