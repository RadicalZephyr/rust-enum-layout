# Rust Enum Layout Test

I got interested in how Rust lays out nested enums, so I wrote a
little test to show what happens, and I decided to put it on Github
because it was not what I expected.


## Reading the Type Names

I named the types so you can sort of tell what a type is just from the
name. A type with a name that starts with `R` is a "simple" enum that
just has labeled variants with no internal data, and the number after
the `R` is the number of different variants in the type, i.e. it's
"rank" (hence the "R").

A type prefixed with `Comp` is a composite enum, the number after
`Comp` is the number of variants in the top level enum. The suffixes
after that indicate what types are contained in the respective
variants. If there is only one type that same type is contained inside
each variant.


## Results and Commentary

Okay, now the current output along with my opinionated and only
minimally informed commentary.

### Simple enums (no nested types)

```
size of R0:     0 bytes
size of R1:     0 bytes
size of R2:     1 bytes
size of R3:     1 bytes
size of R257:   2 bytes
```

No real surprises here, we need a tag field large enough to be able to
differentiate all the variants. Only sort-of interesting thing is that
here both the rank 0 and rank 1 enums are both correctly recognized as
ZSTs (a "zero sized type").

### Composite new-type enums (single variant with internal data)

```
size of Comp1R0:        0 bytes
size of Comp1R1:        0 bytes
size of Comp1R2:        1 bytes
size of Comp1R3:        1 bytes
size of Comp1R257:      2 bytes
```

Not much interesting going on here either, this basically shows that
newtype enums and newtype structs are both optimized away during
layout.


### Composite enums with two variants

```
size of Comp2R0:        0 bytes
size of Comp2R1:        1 bytes
size of Comp2R2:        2 bytes
size of Comp2R3:        2 bytes
size of Comp2R257:      4 bytes
```

This is the first interesting result to me.

Now we can see that the compiler is making a distinction between the
rank 0 and rank 1 enums, and apparently it decides that we need a tag
type to differentiate between the two variants of the composite. I
guess that makes sense because while the R0 and the R1 enums may both
be ZSTs, the difference between them is that the R1 enum can actually
be created. Since all variants of `Comp2R0` contain a value that can
never be created, that enum can never be created either. This remains
true no matter how much nesting or how many variants containing R0 we
have, so I won't include the composite R0 types in the remaining
results.

The other interesting thing here is that the R2 and R3 containing
composites need 2 bytes for tags! Wondering what the compiler would do
with this kind of nested enum is exactly what instigated this entire
investigation. I kind of assumed that the Rust compiler would be able
to optimize this enum layout and basically flatten it out into one
giant enum with only a single tag, so the fact that more bytes are
used is a bit surprising to me.

The Comp2R257 case is even more interesting to me, and it makes me
want to go take a look at the actual code generation to see how those
four bytes are being used. This case seems like a seriously missed
optimization, because now we have actively broken the naive heuristic
that I had that the size of an enum is roughly `1 +
Enum::variants().map(size_of).max()`. My best guess is that this is
actually an alignment thing rather than that the enum data isn't being
packed correctly. Since there are multiple variants in the composite
enum we need a tag number to differentiate them, since the data in
each variant is just the same 16 bit tag for the R257 enum, this tag
field gets padded to 16 bits so the alignment of the inner R257 value
is properly aligned as well. So while it might seem like we the size
of this enum could actually be 3 bytes, in actual existing
architectures it is always going to be effectively 4 bytes because of
alignment padding, even if Rust moved the top level tag to be _after_
the inner data only 8 bit types could be placed directly after that 3
byte value in memory. In real code it's likely that there is even more
padding so this 1 byte inefficiency isn't an issue.

> Editors Note: Forgive the Rusty pseudocode describing my heuristic,
it just seemed clearer to express the idea in code or math instead of
English, and I couldn't figure out how to write it using mathematical
set builder notation in a way that was remotely readable.


### Composite enums with 3 variants

```
size of Comp3R1:    1 bytes
size of Comp3R2:    2 bytes
size of Comp3R3:    2 bytes
size of Comp3R257:  4 bytes
```

### Composite enums with two different variants

```
size of Comp2R0R1:      0 bytes
size of Comp2R1R2:      1 bytes
size of Option<R2>:     1 bytes
size of Comp2R2R3:      2 bytes
size of Comp2R0R257:    2 bytes
size of Comp2R1R257:    2 bytes
size of Comp2R2R257:    4 bytes
size of Comp2R3R257:    4 bytes
```

A couple new interesting things going on here.

First, we have the fact that the Comp2R0R1 variant enum is basically
still being recognized as a newtype of R1. This is again because the
R0 type can never be created, so the variant containing it can't be
created.

The other interesting thing is the Comp2R1R2 variant. This one is
actually surprising because it looks like the enum tag flattening I
expected is actually happening here! So it's surprising because I saw
what I expected after being surprised that I didn't see this behavior
when I expected to earlier! Who's on first?

Anyway, that's interesting and I totally agree with the rust compiler
that it should be able to differentiate between 3 different possible
variants with only 1 byte of tag info.

Arg!! Actually, this is totally expected behavior because this
optimization is basically just the Rust compiler recognizing that
Comp2R1R2 is structurally identical to `Option<R2>` and doing the same
optimization! The fact that it's the structure of `Option` that's
being recognized (rather than that specific type) is actually pretty
cool in itself though.

The very next value then is surprising again though, because for some
reason when we get to Comp2R2R3 we suddenly need 2 bytes for the tag!
Why does the compiler recognize that CompR1R2 only has three possible
values and it can flatten the tag into one byte, but Comp2R2R3 which
only has 5 possible values needs two bytes? This again seems like a
lost opportunity for a layout optimization.

My best guess is that the difference is because R1 is a ZST and that
it's actually going to represent variant `A(R1)` as 0, and then the
tag for R2 is the only data needed, and presumably that variant tag
now needs to be nonzero? Or maybe R2 is represented as 0 and 1, but
`A(R1::A)` is actually some larger value, maybe 255. It's still
interesting that even though the composite enum has two values, no tag
is needed to differentiate between them.

When we get to Comp2R2R257 though it's even more distressing that
we're missing out on this tag merging optimization because we now have
to pad the composite tag for alignment reasons.

### Composite enums with variants, two different inner types

These variants introduce a new pattern. The top level composite has
three variants, but there are only two sub-types in the suffix! In
this case this means that the first two variants both contain the
first suffix type, and the third variant contains the second suffix
type.

```
size of Comp3R0R257:    2 bytes
size of Comp3R1R257:    2 bytes
size of Comp3R2R257:    4 bytes
size of Comp3R3R257:    4 bytes
```

Again, the R0 variant propagates and marks all the variants containing
it as unreachable so it gets optimized away.

Again, the recognition that R1 is a ZST is also still present and the
two variants containing it are still being collapsed somehow. This
actually is the enum layout optimization I was thinking was
possible. I'm curious as to what value is being used for the `A(R1)`
and what value is being used for `A(R2)`? I guess it's just some of
the unused space in the second byte of the tag for R257?



## Final Insights

While writing up my commentary I finally actually realized the
implications of the optimization I was suggesting. If Rust were to
flatten the tags for all sub variants into one monolithic variant
flag, then it would be impossible to use the same code to match on the
enum data inside of one specific variant because the layout of that
type when inside this particular composite enum would be different
than when that enum is free-standing or indeed, inside another
enum. So we can't actually do this optimization in as many places as I
thought we could.

In fact, thinking about it even more, I guess that Rust _is_ doing
this layout optimization whenever it can, that's why we see that
`Comp2R1R257`, `Comp3R1R257` are both the same size as R257. Because
R1 is a singleton value it cannot possibly be matched on (and in fact
we don't need any data to represent it!), so once we match on the
composite the only interesting value we could possibly have is an
`R257`. But `R257` doesn't use all the values in it's 16 bit tag, so
we can just encode those two variants containing no data in some of
the values that aren't used by `R257`.

However, as soon as we need to be able to match on at least two
variants, we actually need to have a separate tag value for the outer
enum, because we need to be able to match on the values for either
variants data in the same way that it is matched on when it's an
independent type.

But...

Wait a second. If we can just use some of those extra bits in R257's
tag when the other data is a singleton, but we just need to
differentiate the top-level variants, shouldn't we be able to do the
same thing when other variants also need to be matchable? It's just a
bit more tricky to recognize that it's possible.

R257 only needs 1 bit of the second tag byte, and it uses values 0 to
256, (or `0x0000` to `0x0100`), and 257 to 65,535 are unused
(i.e. `0x0101` to `0xFFFF`).

So let's say that we're laying out `CompR2R257`. We definitely need at
least two bytes because that's how big R257 is. But we could say that
`CompR2R257::A` (which contains an R2), is then recognized by starting
with `0x02`, and then the R2 value (which is 1 byte) is packed into
the second byte of the tag using it's normal values and it can still
be matched on separately as an 8 bit value. Assuming that the valid
values for it are `0x00` and `0x01` that variant is then either
`0x0200` or `0x0201`. So now we're using values 0 to 256, 512, and
513, and the values 257 to 511, 514 to 65,535 are unused.

This is admittedly a bit of an artificial case, because not many enums
in the wild have even 257 values, let alone even more than that!

And if we can't constrain what that first byte is we can't necessarily
reuse it for the tag of the composite enum.
