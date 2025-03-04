This is a set of macros for ergonomically working with
[`TCell`]s and [`TLCell`]s from the [`qcell`] crate.
This is particularly inspired by the [`cell_family`]
crate, and one additional goal for the `cell_wrappers`
crate is to be compatible with the [`qcell`] codebase
as a dependency; no forking necessary.

# Simple declaration

Creating a marker-owner-cell system is easy with `def_cells`,
as we can see here:

```rust
def_cells! {
    [pub mod] foo_grp: TCellUniGrp;
}
```

This creates a new inline module (named `foo_grp`, in this example),
and populates it with a new [`TCell`] system, along with some
automatically-implemented `trait`s that will help other parts
of this crate provide you with even *more* leverage.

You can even create complex clusters of systems in one go:

```rust
def_cells! {
    [pub mod] put_some_here::{a_bit_further: TLCellUniGrp};
    [pub mod] two_go_here::{
        this_longer_way::{now_arrived: TLCellPvtGrp},
        and_also_this_way: TLCellPubGrp
    };
    [mod] oh_and_here_too::{no_here::{okay_yes_here: TLCellAccGrp}};
}
```

The above declaration will create the following inline modules
of cell systems:

```
pub mod put_some_here {
    pub mod a_bit_further { ... }
}
pub mod two_go_here {
    pub mod this_longer_way {
        pub mod now_arrived { ... }
    }
    pub mod and_also_this_way { ... }
}
mod oh_and_here_too {
    pub mod no_here {
        pub mod okay_yes_here { ... }
    }
}
```

The following `type`s were also declared in `put_some_here::a_bit_further`,
for example, since we declared it as a `TLCellUniGrp`:

```rust
pub mod put_some_here {
    pub mod a_bit_further {
        // This is a TLCell Uni Grp (or "TLCellUniGrp"):
        pub struct UniMarker;
        pub type UniCell<T> = qcell::TLCell<UniMarker,T>;
        pub type UniOwner = qcell::TLCellOwner<UniMarker>;

        // Utility traits are implemented too:
        // ...
    }
}
```

# Cell system subcategories

This crate also offers four subcategories of cell systems, which offer
three subcategories of cells, primarily for self-describing
project organization purposes.

## The subcategories of cells are:

1. `uniform`: A general-purpose subcategory
2. `private`: A subcategory intended for private struct methods
3. `public`: A subcategory intended for program-wide use

## The `mod` subcategories are:

1. `UniGrp:` Creates one system of uniform cells
2. `AccGrp:` Creates one system of public cells, and one system of private cells
3. `PubGrp:` Creates one system of public cells
4. `PvtGrp:` Creates one system of private cells

And these get appended to the cell types to form declaration
identifiers, like so:

> `TLCell + UniGrp -> TLCellUniGrp`

# Families-style declaration:

Before explaining the benefits of these subcategories, this crate
does also provide even *simpler* declarations for anyone who prefers
the simplicity of [`cell_family`]. The `cell_wrappers` crate only depends
on the `qcell` implementation, however, and does not reimplement the
same logic adjustments found in the [`cell_family`] implementation.

```rust
// For creating a TCell system:
new_t_group!(FooOwner[FooMarker] => FooCell<T>);

// An alternative syntax, and creating a TLCell system this time:
new_tl_group! {
    marker: BarMarker,
    owner: BarOwner,
    cell: BarCell<T>
}

// And if you'd rather create each part individually:
new_t_marker_type!(BazMarker);
new_t_cell_type!(BazCell[BazMarker]<T>);
new_t_owner_type!(BazOwner[TestTMarker]);

// These allows for visibility specs and attributes:
new_t_group!(#[allow(dead_code)] pub FooOwner[FooMarker] => FooCell<T>);

new_tl_group! {
    pub marker: BarMarker,
    #[allow(dead_code)]
    pub owner: BarOwner,
    pub cell: BarCell<T>
}
```

Read more at:
* `new_t_group` / `new_tl_group`
* `new_t_marker` / `new_tl_marker`
* `new_t_owner` / `new_tl_owner`
* `new_t_cell` / `new_tl_cell`

# Quick owner scopes

This crate provides a flexible macro-based syntax for quickly
and easily setting up cell owner scopes for a wide variety of
use cases.

This macro is called "`c_scp`", which is short for "cell scope".

The following is an example, which will be investigated shortly:

```rust
c_scp! {
    use test_uni_grp::UniOwner => (
        self.test_cell => mut test_cont
    ) {
        *test_cont += 1;
        assert_eq!(*test_cont, 1);
    }
}
```

## Owner reference

We start by declaring the owner type to use,
and we will create an anonymous one: \
> `use test_uni_grp::UniOwner =>`

**However,** we also have these options, too:

1. `let owner_name = test_uni_grp::UniOwner =>` \
Owners default to the name "`__scope_owner`", but we can
also set a custom name, such as `owner_name`.
2. `use _ =>` or `let owner_name = _ =>` \
This automatically determines the necessary owner type, based
on context.
3. `use [self] =>` or `let owner_name = [self] =>` \
This can only be used in the method of a `struct`, which
implements one of three special `trait`s, and will be
explained in more detail later.
4. `use &borrowed_owner =>` or `use &mut borrowed_owner =>` \
This selects an owner available in the surrounding scope,
declared with the identifier of `borrowed_owner`, for example.

## Cell and container references

Next, we have the following: \
> `(self.test_cell => mut test_cont)`

This selects `self.test_cell` as the cell which matches the owner,
and its contained value will be assigned to a new variable,
called `test_cont`. We are declaring `test_cont` as mutable here,
so the owner will be borrowed mutably, too, and the cell will be
accessed with its `rw()` method.

If we only wanted to select the cell, and not access it yet,
then the syntax lets us work with the following alternatives:

1. > `(self.test_cell)` \
which coerces the owner to be borrowed ***immutably***, and...
2. > `(mut self.test_cell)` \
...which coerces the owner to be borrowing ***mutably***.

### Container reference options

There are quite a lot of ways to declare the container variable:

1. > `(self.test_cell => test_cont)` \
Declares an immutable variable.
2. > `(self.test_cell => & test_cont)` \
Adds an extra borrow operator when pulling `self.test_cell`'s value.
3. > `(self.test_cell => * test_cont)` \
Dereferences the value within `self.test_cell` before assigning
it to `test_cont`.
4. > `(self.test_cell => *mut test_cont)` \
Dereferences the value first, but also declares `test_cont`
as mutable.
5. > `(self.test_cell => *out outer_cont)` \
Dereferences the value in `self.test_cell`, and assigns it to a
variable found in the surrounding scope, which is called
`outer_cont`, in this example. This is useful for situations where
you do not want to work with the value in a new scope, and just want
to extract the value from the cell for use in the surrounding scope.
6. > `(self.test_cell => *out mut test_cont)` \
Same as before, but for cases where `outer_cont` is declared
***mutable***.
7. > `(self.test_cell => test_cont : u8)` \
Coerces the extracted value to `u8` before storing it in `test_cont`.
8. > `(self.test_cell => *out test_cont as u8)` \
When sending the value to the *surrounding* scope, `as` must be used,
since a new variable is not being declared for use in the
inner scope.

## Scope body

Finally, we have...

> `{ *test_cont += 1; assert_eq!(*test_cont, 1); }`

...which is just the statement block that gets put into the new
enclosing scope. Once these are completed, then the scope is exited,
and the owner will be dropped, unless it was borrowed from an outer
scope.

# Provided utility `trait`s

This crate provides a lot of `trait`s for internal use, but three
of them are available for you to make use of, specifically:

```rust
// 1.
pub trait GetPvtOwner<T> {
    fn get_private_owner(&self) -> T;
}

// 2.
pub trait GetPubOwner<T> {
    fn get_public_owner(&self) -> T;
}

// 3.
pub trait GetUniOwner<T> {
    fn get_uniform_owner(&self) -> T;
}
```

For [`c_scp`] syntaxes which use the `[self]` owner source, these traits
are called for `self`. A `struct` can implement these for any number
of relevant owners, and three macros are provided for automatic
implementation of these `trait`s:

1. `impl_get_pvt!(struct_name, owner::Path);`
2. `impl_get_pub!(struct_name, owner::Path);`
3. `impl_get_uni!(struct_name, owner::Path);`

You may also want to create custom implementations for these,
as well, in case you want these methods to do any extra tasks
before or after an owner is being provided.

These three `trait`s are selected by the cell being accessed in
the `c_scp` syntax, so it will choose a **uniform**, **public**,
or **private** implementation based on its own declared
subcategory type.

Read more at:
* `GetPvtOwner` / `impl_get_pvt`
* `GetPubOwner` / `impl_get_pub`
* `GetUniOwner` / `impl_get_uni`

[`TCell`]: https://docs.rs/qcell/latest/qcell/struct.TCell.html
[`TLCell`]: https://docs.rs/qcell/latest/qcell/struct.TLCell.html
[`qcell`]: https://docs.rs/qcell/latest/qcell/index.html
[`cell_family`]: https://docs.rs/cell-family/0.1.0/cell_family/index.html