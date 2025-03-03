This is a set of macros for ergonomically working with
[`TCell`]s and [`TLCell`]s from the [`qcell`] crate.
This was particularly inspired by the [`cell_family`]
crate, and one additional goal for the `cell_wrapper`
crate was to be compatible with the [`qcell`] codebase
as a dependency; no forking necessary.

# Simple declaration

Creating a marker-owner-cell system is now as easy as:

```rust
def_cells! {
    [pub mod] foo_grp: TCellUniGrp;
}
```

This creates a new inline module (named `foo_grp`, in this example),
and populates it with a new `TCell` system, along with some
automatically-implemented traits that will help other parts
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
pub mod put_some_here::a_bit_further
pub mod two_go_here::this_longer_way::now_arrived
pub mod two_go_here::and_also_this_way
mod oh_and_here_too::no_here::okay_yes_here
```

# Cell system subcategories

This crate also offers four subcategories of cell systems, which offer
three subcategories of cells, primarily for self-describing
project organization purposes.

## The subcategories of cells are:

1. **Uniform:** A general-purpose subcategory
2. **Private:** A subcategory intended for private struct methods
3. **Public:** A subcategory intended for program-wide use

## The `mod` subcategories are:

1. **UniGrp:** Creates one system of uniform cells
2. **AccGrp:** Creates one system of public cells, and one system of private cells
3. **PubGrp:** Creates one system of public cells
4. **PvtGrp:** Creates one system of private cells

# Families-style declaration:

Before explaining the benefits of these subcategories, this crate
does also provide even *simpler* declarations for anyone who prefers
the simplicity of `cell_family`. The `cell_wrapper` crate only depends
on the `qcell` implementation, however, and does not reimplement the
same logic adjustments found in the `cell_family` implementation.

```rust
// For creating a TCell system:
create_t_group!(FooOwner[FooMarker] => FooCell<T>);

// An alternative syntax, and creating a TLCell system this time:
create_tl_group! {
    marker: BarMarker,
    owner: BarOwner,
    cell: BarCell<T>
}

// And if you'd rather create each part individually:
new_t_marker_type!(BazMarker);
new_t_cell_type!(BazCell[BazMarker]<T>);
new_t_owner_type!(BazOwner[TestTMarker]);

// These allows for visibility specs and attributes:
create_t_group!(#[allow(dead_code)] pub FooOwner[FooMarker] => FooCell<T>);

create_tl_group! {
    pub marker: BarMarker,
    #[allow(dead_code)]
    pub owner: BarOwner,
    pub cell: BarCell<T>
}
```

[`TCell`]: https://docs.rs/qcell/latest/qcell/struct.TCell.html
[`TLCell`]: https://docs.rs/qcell/latest/qcell/struct.TLCell.html
[`qcell`]: https://docs.rs/qcell/latest/qcell/index.html