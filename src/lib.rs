//! This is a set of macros for ergonomically working with
//! [`TCell`]s and [`TLCell`]s from the [`qcell`] crate.
//! This was particularly inspired by the [`cell_family`]
//! crate, and one additional goal for the `cell_wrapper`
//! crate was to be compatible with the [`qcell`] codebase
//! as a dependency; no forking necessary.
//! 
//! # Simple declaration
//! 
//! Creating a marker-owner-cell system is now as easy as:
//! 
//! ```rust
//! def_cells! {
//!     [pub mod] foo_grp: TCellUniGrp;
//! }
//! ```
//! 
//! This creates a new inline module (named `foo_grp`, in this example),
//! and populates it with a new `TCell` system, along with some
//! automatically-implemented `trait`s that will help other parts
//! of this crate provide you with even *more* leverage.
//! 
//! You can even create complex clusters of systems in one go:
//! 
//! ```rust
//! def_cells! {
//!     [pub mod] put_some_here::{a_bit_further: TLCellUniGrp};
//!     [pub mod] two_go_here::{
//!         this_longer_way::{now_arrived: TLCellPvtGrp},
//!         and_also_this_way: TLCellPubGrp
//!     };
//!     [mod] oh_and_here_too::{no_here::{okay_yes_here: TLCellAccGrp}};
//! }
//! ```
//! 
//! The above declaration will create the following inline modules
//! of cell systems:
//! 
//! ```
//! pub mod put_some_here {
//!     pub mod a_bit_further { ... }
//! }
//! pub mod two_go_here {
//!     pub mod this_longer_way {
//!         pub mod now_arrived { ... }
//!     }
//!     pub mod and_also_this_way { ... }
//! }
//! mod oh_and_here_too {
//!     pub mod no_here {
//!         pub mod okay_yes_here { ... }
//!     }
//! }
//! ```
//! 
//! The following `type`s were also declared in `put_some_here::a_bit_further`,
//! for example, since we declared it as a `TLCellUniGrp`:
//! 
//! ```rust
//! pub mod put_some_here {
//!     pub mod a_bit_further {
//!         // This is a TLCell Uni Grp (or "TLCellUniGrp"):
//!         pub struct UniMarker;
//!         pub type UniCell<T> = qcell::TLCell<UniMarker,T>;
//!         pub type UniOwner = qcell::TLCellOwner<UniMarker>;
//! 
//!         // Utility traits are implemented too:
//!         // ...
//!     }
//! }
//! ```
//! 
//! # Cell system subcategories
//! 
//! This crate also offers four subcategories of cell systems, which offer
//! three subcategories of cells, primarily for self-describing
//! project organization purposes.
//! 
//! ## The subcategories of cells are:
//! 
//! 1. `uniform`: A general-purpose subcategory
//! 2. `private`: A subcategory intended for private struct methods
//! 3. `public`: A subcategory intended for program-wide use
//! 
//! ## The `mod` subcategories are:
//! 
//! 1. `UniGrp:` Creates one system of uniform cells
//! 2. `AccGrp:` Creates one system of public cells, and one system of private cells
//! 3. `PubGrp:` Creates one system of public cells
//! 4. `PvtGrp:` Creates one system of private cells
//! 
//! And these get appended to the cell types to form declaration
//! identifiers, like so:
//! 
//! > `TLCell` + `UniGrp` = `TLCellUniGrp`
//! 
//! # Families-style declaration:
//! 
//! Before explaining the benefits of these subcategories, this crate
//! does also provide even *simpler* declarations for anyone who prefers
//! the simplicity of `cell_family`. The `cell_wrapper` crate only depends
//! on the `qcell` implementation, however, and does not reimplement the
//! same logic adjustments found in the `cell_family` implementation.
//! 
//! ```rust
//! // For creating a TCell system:
//! create_t_group!(FooOwner[FooMarker] => FooCell<T>);
//! 
//! // An alternative syntax, and creating a TLCell system this time:
//! create_tl_group! {
//!     marker: BarMarker,
//!     owner: BarOwner,
//!     cell: BarCell<T>
//! }
//! 
//! // And if you'd rather create each part individually:
//! new_t_marker_type!(BazMarker);
//! new_t_cell_type!(BazCell[BazMarker]<T>);
//! new_t_owner_type!(BazOwner[TestTMarker]);
//! 
//! // These allows for visibility specs and attributes:
//! create_t_group!(#[allow(dead_code)] pub FooOwner[FooMarker] => FooCell<T>);
//! 
//! create_tl_group! {
//!     pub marker: BarMarker,
//!     #[allow(dead_code)]
//!     pub owner: BarOwner,
//!     pub cell: BarCell<T>
//! }
//! ```
//! 
//! # Quick owner scopes
//! 
//! This crate provides a flexible macro-based syntax for quickly
//! and easily setting up cell owner scopes for a wide variety of
//! use cases.
//! 
//! This macro is called "`c_scp`", which is short for "cell scope".
//! 
//! The following is an example, which will be investigated shortly:
//! 
//! ```rust
//! c_scp! {
//!     use test_uni_grp::UniOwner => (
//!         self.test_cell => mut test_cont
//!     ) {
//!         *test_cont += 1;
//!         assert_eq!(*test_cont, 1);
//!     }
//! }
//! ```
//! 
//! ## Owner reference
//! 
//! We start by declaring the owner type to use,
//! and we will create an anonymous one: \
//! > `use test_uni_grp::UniOwner =>`
//! 
//! **However,** we also have these options, too:
//! 
//! 1. `let owner_name = test_uni_grp::UniOwner =>` \
//! Owners default to the name "`__scope_owner`", but we can
//! also set a custom name, such as `owner_name`.
//! 2. `use _ =>` or `let owner_name = _ =>` \
//! This automatically determines the necessary owner type, based
//! on context.
//! 3. `use [self] =>` or `let owner_name = [self] =>` \
//! This can only be used in the method of a `struct`, which
//! implements one of three special `trait`s, and will be
//! explained in more detail later.
//! 4. `use &borrowed_owner =>` or `use &mut borrowed_owner =>` \
//! This selects an owner available in the surrounding scope,
//! declared with the identifier of `borrowed_owner`, for example.
//! 
//! ## Cell and container references
//! 
//! Next, we have the following: \
//! > `(self.test_cell => mut test_cont)`
//! 
//! This selects `self.test_cell` as the cell which matches the owner,
//! and its contained value will be assigned to a new variable,
//! called `test_cont`. We are declaring `test_cont` as mutable here,
//! so the owner will be borrowed mutably, too, and the cell will be
//! accessed with its `rw()` method.
//! 
//! If we only wanted to select the cell, and not access it yet,
//! then the syntax lets us work with the following alternatives:
//! 
//! 1. > `(self.test_cell)` \
//! which coerces the owner to be borrowed ***immutably***, and...
//! 2. > `(mut self.test_cell)` \
//! ...which coerces the owner to be borrowing ***mutably***.
//! 
//! ### Container reference options
//! 
//! There are quite a lot of ways to declare the container variable:
//! 
//! 1. > `(self.test_cell => test_cont)` \
//! Declares an immutable variable.
//! 2. > `(self.test_cell => & test_cont)` \
//! Adds an extra borrow operator when pulling `self.test_cell`'s value.
//! 3. > `(self.test_cell => * test_cont)` \
//! Dereferences the value within `self.test_cell` before assigning
//! it to `test_cont`.
//! 4. > `(self.test_cell => *mut test_cont)` \
//! Dereferences the value first, but also declares `test_cont`
//! as mutable.
//! 5. > `(self.test_cell => *out outer_cont)` \
//! Dereferences the value in `self.test_cell`, and assigns it to a
//! variable found in the surrounding scope, which is called
//! `outer_cont`, in this example. This is useful for situations where
//! you do not want to work with the value in a new scope, and just want
//! to extract the value from the cell for use in the surrounding scope.
//! 6. > `(self.test_cell => *out mut test_cont)` \
//! Same as before, but for cases where `outer_cont` is declared
//! ***mutable***.
//! 7. > `(self.test_cell => test_cont : u8)` \
//! Coerces the extracted value to `u8` before storing it in `test_cont`.
//! 8. > `(self.test_cell => *out test_cont as u8)` \
//! When sending the value to the *surrounding* scope, `as` must be used,
//! since a new variable is not being declared for use in the
//! inner scope.
//! 
//! ## Scope body
//! 
//! Finally, we have...
//! 
//! > `{ *test_cont += 1; assert_eq!(*test_cont, 1); }`
//! 
//! ...which is just the statement block that gets put into the new
//! enclosing scope. Once these are completed, then the scope is exited,
//! and the owner will be dropped, unless it was borrowed from an outer
//! scope.
//! 
//! # Provided utility `trait`s
//! 
//! This crate provides a lot of `trait`s for internal use, but three
//! of them are available for you to make use of, specifically:
//! 
//! ```rust
//! // 1.
//! pub trait GetPvtOwner<T> {
//!     fn get_private_owner(&self) -> T;
//! }
//! 
//! // 2.
//! pub trait GetPubOwner<T> {
//!     fn get_public_owner(&self) -> T;
//! }
//! 
//! // 3.
//! pub trait GetUniOwner<T> {
//!     fn get_uniform_owner(&self) -> T;
//! }
//! ```
//! 
//! For `c_scp` syntaxes which use the `[self]` owner source, these traits
//! are called for `self`. A `struct` can implement these for any number
//! of relevant owners, and three macros are provided for automatic
//! implementation of these `trait`s:
//! 
//! 1. `impl_get_pvt!(struct_name, owner::Path);`
//! 1. `impl_get_pub!(struct_name, owner::Path);`
//! 1. `impl_get_uni!(struct_name, owner::Path);`
//! 
//! You may also want to create custom implementations for these,
//! as well, in case you want these methods to do any extra tasks
//! before or after an owner is being provided.
//! 
//! These three `trait`s are selected by the cell being accessed in
//! the `c_scp` syntax, so it will choose a **uniform**, **public**,
//! or **private** implementation based on its own declared
//! subcategory type.
//! 
//! [`TCell`]: https://docs.rs/qcell/latest/qcell/struct.TCell.html
//! [`TLCell`]: https://docs.rs/qcell/latest/qcell/struct.TLCell.html
//! [`qcell`]: https://docs.rs/qcell/latest/qcell/index.html
//! [`cell_family`]: https://lib.rs/crates/cell-family

#[macro_export]
macro_rules! c_scp {
    ( @handle_op_props ( $scope:ident $ref_type:ident immut $cell_info:tt ) $decl:tt ) => {
        & $decl
    };
    ( @handle_op_props ( $scope:ident $ref_type:ident ismut $cell_info:tt ) $decl:tt ) => {
        & mut $decl
    };
    {
        @handle_owner_source ( @from_path $type_path:path => ) ,
        ( $scope:ident $ref_type:ident $muta:ident $cell_info:tt )
    } => {
        <$type_path> :: new()
    };
    {
        @handle_owner_source ( @from_auto => ) ,
        ( $scope:ident $ref_type:ident $muta:ident ( $cell_src:expr ) )
    } => {
        $cell_src . get_new_matching_owner()
    };
    {
        @handle_owner_source ( @from_self $_self:ident => ) ,
        ( $scope:ident $ref_type:ident $muta:ident ( $cell_src:expr ) )
    } => {
        $cell_src . get_matching_owner_from($_self)
    };
    ( @handle_sources_right $owner_name:ident , ( @from_scp_src_immut $owner_src:expr => ) , $op_props:tt ) => {
        let $owner_name = & $owner_src;
    };
    ( @handle_sources_right $owner_name:ident , ( @from_scp_src_ismut $owner_src:expr => ) , $op_props:tt ) => {
        let $owner_name = &mut $owner_src;
    };
    ( @handle_sources_right $owner_name:ident , $src_info:tt , $op_props:tt ) => {
        let $owner_name =
        $crate::c_scp!( @handle_op_props $op_props (
            $crate::c_scp!( @handle_owner_source $src_info , $op_props )
        ) ) ;
    };
    // General pattern reorganizer - Stage 1
    // Separate out borrow details
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                mut $cell_src:expr
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal borrow ismut ( $cell_src ) )
        );
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal borrow immut ( $cell_src ) )
        );
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                * out mut $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external deref ismut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        ( * $cell_src . rw ( $owner_name ) ) $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                & out mut $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external hardborrow ismut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        ( & $cell_src . rw ( $owner_name ) ) $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                out mut $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external borrow ismut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        ( $cell_src . rw ( $owner_name ) ) $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                & out $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external hardborrow immut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        &( $cell_src . ro ( $owner_name ) )
        $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                * out $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external deref immut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        ( * $cell_src . ro ( $owner_name ) )
        $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                * mut $container_name:ident $( . $container_ext:ident)*
                $( : $container_type0:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal borrow ismut ( $cell_src ) )
        );
        let mut $container_name $( . $container_ext)*
        $( : $container_type0 )? =
        ( * $cell_src . rw ( $owner_name ) );
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                out $container_name:ident $( . $container_ext:ident)*
                $( as $container_type1:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( external borrow immut ( $cell_src ) )
        );
        $container_name $( . $container_ext)* =
        $cell_src . ro ( $owner_name )
        $( as $container_type1 )? ;
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                * $container_name:ident $( . $container_ext:ident)*
                $( : $container_type0:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal deref immut ( $cell_src ) )
        );
        let $container_name $( . $container_ext)*
        $( : $container_type0 )? =
        * $cell_src . ro ( $owner_name );
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                $(&)? mut $container_name:ident $( . $container_ext:ident)*
                $( : $container_type0:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal borrow ismut ( $cell_src ) )
        );
        let $container_name $( . $container_ext)*
        $( : $container_type0 )? =
        $cell_src . rw ( $owner_name );
    };
    {
        @reorganize_body1 (
            $owner_type_header:tt ,
            ( $owner_name:ident ) ,
            (
                $cell_src:expr =>
                $(&)? $container_name:ident $( . $container_ext:ident)*
                $( : $container_type0:ty )?
            )
        )
    } => {
        $crate::c_scp!(
            @handle_sources_right
            $owner_name ,
            $owner_type_header ,
            ( internal borrow immut ( $cell_src ) )
        );
        let $container_name $( . $container_ext)*
        $( : $container_type0 )? =
        $cell_src . ro ( $owner_name );
    };
    // General pattern reorganizer
    {
        let $scope_owner_name:ident = _ =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_auto => ) ,
                    ( $scope_owner_name ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        use _ =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_auto => ) ,
                    ( __scope_owner ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        let $scope_owner_name:ident = & mut $owner_src:expr =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_scp_src_ismut $owner_src => ) ,
                    ( $scope_owner_name ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        use & mut $owner_src:expr =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_scp_src_ismut $owner_src => ) ,
                    ( __scope_owner ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        let $scope_owner_name:ident = & $owner_src:expr =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_scp_src_immut $owner_src => ) ,
                    ( $scope_owner_name ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        use & $owner_src:expr =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_scp_src_immut $owner_src => ) ,
                    ( __scope_owner ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        let $scope_owner_name:ident = [$_self:ident] =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_self $_self => ) ,
                    ( $scope_owner_name ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        use [$_self:ident] =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_self $_self => ) ,
                    ( __scope_owner ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        let $scope_owner_name:ident =
        $owner_type_header:ident $( :: $owner_path_ext:ident )* =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_path $owner_type_header $( :: $owner_path_ext )* => ) ,
                    ( $scope_owner_name ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    };
    {
        use
        $owner_type_header:ident $( :: $owner_path_ext:ident )* =>
        $operation_details:tt
        $( $statements:block )? $(;)?
    } => {
        {
            $crate::c_scp! {
                @reorganize_body1 (
                    ( @from_path $owner_type_header $( :: $owner_path_ext )* => ) ,
                    ( __scope_owner ) ,
                    $operation_details
                )
            }
            ; $( $statements )?
        }
    }
}

// These structs are just here to give the linter
// something to match to, if desired.
mod cell_grp_types {
    #[allow(dead_code)]
    pub struct TCellUniGrp;
    #[allow(dead_code)]
    pub struct TCellAccGrp;
    #[allow(dead_code)]
    pub struct TLCellUniGrp;
    #[allow(dead_code)]
    pub struct TLCellAccGrp;
    #[allow(dead_code)]
    pub struct TCellPubGrp;
    #[allow(dead_code)]
    pub struct TCellPvtGrp;
    #[allow(dead_code)]
    pub struct TLCellPubGrp;
    #[allow(dead_code)]
    pub struct TLCellPvtGrp;
}

// Represents the TCell and TLCell implementations.
// "GT" refers to the possibility of either T or TL.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellImpl {
    T,
    TL
}

// Represents the default access types.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellAccessLevels {
    Uniform,
    Private,
    Public
}

// Represents the roles in the T/TLCell ecosystem.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellRoles {
    Marker,
    Owner,
    Cell
}

// These are here for execution management and organization.
//
// Implementation type
pub trait IsTImpl {
    #[inline]
    fn get_cell_impl() -> CellImpl {
        CellImpl::T
    }
    #[inline]
    fn get_self_cell_impl(&self) -> CellImpl {
        CellImpl::T
    }
}
pub trait IsTLImpl {
    #[inline]
    fn get_cell_impl() -> CellImpl {
        CellImpl::TL
    }
    #[inline]
    fn get_self_cell_impl(&self) -> CellImpl {
        CellImpl::TL
    }
}
// Access levels
pub trait IsGTPvtAccess {
    #[inline]
    fn is_private_access() -> bool {
        true
    }
    #[inline]
    fn is_uniform_access() -> bool {
        false
    }
    #[inline]
    fn is_public_access() -> bool {
        false
    }
    #[inline]
    fn get_access_level() -> CellAccessLevels {
        CellAccessLevels::Private
    }
    #[inline]
    fn is_self_private_access(&self) -> bool {
        true
    }
    #[inline]
    fn is_self_uniform_access(&self) -> bool {
        false
    }
    #[inline]
    fn is_self_public_access(&self) -> bool {
        false
    }
    #[inline]
    fn get_self_access_level(&self) -> CellAccessLevels {
        CellAccessLevels::Private
    }
}
pub trait IsTPvtAccess : IsGTPvtAccess + IsTImpl {}
pub trait IsTLPvtAccess : IsGTPvtAccess + IsTLImpl {}
pub trait IsGTUniAccess {
    #[inline]
    fn is_private_access() -> bool {
        false
    }
    #[inline]
    fn is_uniform_access() -> bool {
        true
    }
    #[inline]
    fn is_public_access() -> bool {
        false
    }
    #[inline]
    fn get_access_level() -> CellAccessLevels {
        CellAccessLevels::Uniform
    }
    #[inline]
    fn is_self_private_access(&self) -> bool {
        false
    }
    #[inline]
    fn is_self_uniform_access(&self) -> bool {
        true
    }
    #[inline]
    fn is_self_public_access(&self) -> bool {
        false
    }
    #[inline]
    fn get_self_access_level(&self) -> CellAccessLevels {
        CellAccessLevels::Uniform
    }
}
pub trait IsTUniAccess : IsGTUniAccess + IsTImpl {}
pub trait IsTLUniAccess : IsGTUniAccess + IsTLImpl {}
pub trait IsGTPubAccess {
    #[inline]
    fn is_private_access() -> bool {
        false
    }
    #[inline]
    fn is_uniform_access() -> bool {
        false
    }
    #[inline]
    fn is_public_access() -> bool {
        true
    }
    #[inline]
    fn get_access_level() -> CellAccessLevels {
        CellAccessLevels::Public
    }
    #[inline]
    fn is_self_private_access(&self) -> bool {
        false
    }
    #[inline]
    fn is_self_uniform_access(&self) -> bool {
        false
    }
    #[inline]
    fn is_self_public_access(&self) -> bool {
        true
    }
    #[inline]
    fn get_self_access_level(&self) -> CellAccessLevels {
        CellAccessLevels::Public
    }
}
pub trait IsTPubAccess : IsGTPubAccess + IsTImpl {}
pub trait IsTLPubAccess : IsGTPubAccess + IsTLImpl {}
// Markers
pub trait IsGTMarker {
    #[inline]
    fn get_cell_role() -> CellRoles {
        CellRoles::Marker
    }
    #[inline]
    fn get_self_cell_role(&self) -> CellRoles {
        CellRoles::Marker
    }
}
pub trait IsGTUniMarker : IsGTMarker + IsGTUniAccess {}
pub trait IsGTPubMarker : IsGTMarker + IsGTPubAccess {}
pub trait IsGTPvtMarker : IsGTMarker + IsGTPvtAccess {}
pub trait IsTMarker : IsGTMarker + IsTImpl {}
pub trait IsTUniMarker : IsTMarker + IsTUniAccess {}
pub trait IsTPubMarker : IsTMarker + IsTPubAccess {}
pub trait IsTPvtMarker : IsTMarker + IsTPvtAccess {}
pub trait IsTLMarker : IsGTMarker + IsTLImpl {}
pub trait IsTLUniMarker : IsTLMarker + IsTLUniAccess {}
pub trait IsTLPubMarker : IsTLMarker + IsTLPubAccess {}
pub trait IsTLPvtMarker : IsTLMarker + IsTLPvtAccess {}
// Owners
pub trait IsGTOwner {
    #[inline]
    fn get_cell_role() -> CellRoles {
        CellRoles::Owner
    }
    #[inline]
    fn get_self_cell_role(&self) -> CellRoles {
        CellRoles::Owner
    }
}
pub trait IsGTUniOwner : IsGTOwner + IsGTUniAccess {}
pub trait IsGTPubOwner : IsGTOwner + IsGTPubAccess {}
pub trait IsGTPvtOwner : IsGTOwner + IsGTPvtAccess {}
pub trait IsTOwner : IsGTOwner + IsTImpl {}
pub trait IsTUniOwner : IsTOwner + IsTUniAccess {}
pub trait IsTPubOwner : IsTOwner + IsTPubAccess {}
pub trait IsTPvtOwner : IsTOwner + IsTPvtAccess {}
pub trait IsTLOwner : IsGTOwner + IsTLImpl {}
pub trait IsTLUniOwner : IsTLOwner + IsTLUniAccess {}
pub trait IsTLPubOwner : IsTLOwner + IsTLPubAccess {}
pub trait IsTLPvtOwner : IsTLOwner + IsTLPvtAccess {}
// Cells
pub trait IsGTCell {
    #[inline]
    fn get_cell_role() -> CellRoles {
        CellRoles::Cell
    }
    #[inline]
    fn get_self_cell_role(&self) -> CellRoles {
        CellRoles::Cell
    }
}
pub trait IsGTUniCell : IsGTCell + IsGTUniAccess {}
pub trait IsGTPubCell : IsGTCell + IsGTPubAccess {}
pub trait IsGTPvtCell : IsGTCell + IsGTPvtAccess {}
pub trait IsTCell : IsGTCell + IsTImpl {}
pub trait IsTUniCell : IsTCell + IsTUniAccess {}
pub trait IsTPubCell : IsTCell + IsTPubAccess {}
pub trait IsTPvtCell : IsTCell + IsTPvtAccess {}
pub trait IsTLCell : IsGTCell + IsTLImpl {}
pub trait IsTLUniCell : IsTLCell + IsTLUniAccess {}
pub trait IsTLPubCell : IsTLCell + IsTLPubAccess {}
pub trait IsTLPvtCell : IsTLCell + IsTLPvtAccess {}

pub trait GetEasyPvtOwner {
    type OwnerType;
    fn get_new_matching_owner(&self) -> Self::OwnerType;
    fn get_matching_owner_from(&self, src : & impl GetPvtOwner<Self::OwnerType>) -> Self::OwnerType;
}

pub trait GetEasyPubOwner {
    type OwnerType;
    fn get_new_matching_owner(&self) -> Self::OwnerType;
    fn get_matching_owner_from(&self, src : & impl GetPubOwner<Self::OwnerType>) -> Self::OwnerType;
}

pub trait GetEasyUniOwner {
    type OwnerType;
    fn get_new_matching_owner(&self) -> Self::OwnerType;
    fn get_matching_owner_from(&self, src : & impl GetUniOwner<Self::OwnerType>) -> Self::OwnerType;
}

// Implementation defaults for qcell contents
impl<Q, T> IsTImpl for qcell::TCell<Q, T> where T : ?Sized {}
impl<Q, T> IsGTCell for qcell::TCell<Q, T> where T : ?Sized {}
impl<Q, T> IsTCell for qcell::TCell<Q, T> where T : ?Sized {}

impl<Q, T> IsTLImpl for qcell::TLCell<Q, T> where T : ?Sized {}
impl<Q, T> IsGTCell for qcell::TLCell<Q, T> where T : ?Sized {}
impl<Q, T> IsTLCell for qcell::TLCell<Q, T> where T : ?Sized {}

impl<Q> IsTImpl for qcell::TCellOwner<Q> {}
impl<Q> IsGTOwner for qcell::TCellOwner<Q> {}
impl<Q> IsTOwner for qcell::TCellOwner<Q> {}

impl<Q> IsTLImpl for qcell::TLCellOwner<Q> {}
impl<Q> IsGTOwner for qcell::TLCellOwner<Q> {}
impl<Q> IsTLOwner for qcell::TLCellOwner<Q> {}

// Some traits for getting the shorthand forms to work.
pub trait GetPvtOwner<T> {
    fn get_private_owner(&self) -> T;
}
pub trait GetPubOwner<T> {
    fn get_public_owner(&self) -> T;
}
pub trait GetUniOwner<T> {
    fn get_uniform_owner(&self) -> T;
}

#[macro_export]
macro_rules! impl_get_pvt {
    ( $struct_name:ident => $owner_path:path ) => {
        impl $crate::GetPvtOwner<$owner_path> for $struct_name {
            #[inline]
            fn get_private_owner(&self) -> $owner_path {
                <$owner_path>::new()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_pub {
    ( $struct_name:ident => $owner_path:path ) => {
        impl $crate::GetPubOwner<$owner_path> for $struct_name {
            #[inline]
            fn get_public_owner(&self) -> $owner_path {
                <$owner_path>::new()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_uni {
    ( $struct_name:ident => $owner_path:path ) => {
        impl $crate::GetUniOwner<$owner_path> for $struct_name {
            #[inline]
            fn get_uniform_owner(&self) -> $owner_path {
                <$owner_path>::new()
            }
        }
    };
}

#[macro_export]
macro_rules! new_t_marker_type {
    ( @finish_build => ( $marker_name:ident ) ) => {
        impl IsGTMarker for $marker_name {}
        impl IsTMarker for $marker_name {}
        impl IsTImpl for $marker_name {}
        impl IsGTUniAccess for $marker_name {}
        impl IsTUniAccess for $marker_name {}
        impl IsTUniMarker for $marker_name {}
    };
    ( $( # [ $attr:meta ] )* $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        pub struct $marker_name;
        $crate::new_t_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_t_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis type $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_t_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis struct $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_t_marker_type!( @finish_build => ( $marker_name ) )
    }
}

#[macro_export]
macro_rules! new_tl_marker_type {
    ( @finish_build => ( $marker_name:ident ) ) => {
        impl IsGTMarker for $marker_name {}
        impl IsTLMarker for $marker_name {}
        impl IsTLImpl for $marker_name {}
        impl IsGTUniAccess for $marker_name {}
        impl IsTLUniAccess for $marker_name {}
        impl IsTLUniMarker for $marker_name {}
    };
    ( $( # [ $attr:meta ] )* $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        struct $marker_name;
        $crate::new_tl_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_tl_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis type $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_tl_marker_type!( @finish_build => ( $marker_name ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis struct $marker_name:ident ) => {
        $( # [ $attr:meta ] )*
        $visibilty struct $marker_name;
        $crate::new_tl_marker_type!( @finish_build => ( $marker_name ) )
    }
}

#[macro_export]
macro_rules! new_t_owner_type {
    ( @finish_build => ( $owner_name:ident [ $marker_name:ident ] ) ) => {
        impl IsGTUniAccess for $owner_name {}
        impl IsTUniAccess for $owner_name {}
        impl IsTUniOwner for $owner_name {}
    };
    ( $( # [ $attr:meta ] )* $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $owner_name = qcell::TCellOwner<$marker_name>;
        $crate::new_t_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $owner_name = qcell::TCellOwner<$marker_name>;
        $crate::new_t_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis type $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $owner_name = qcell::TCellOwner<$marker_name>;
        $crate::new_t_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
}

#[macro_export]
macro_rules! new_tl_owner_type {
    ( @finish_build => ( $owner_name:ident [ $marker_name:ident ] ) ) => {
        impl IsGTUniAccess for $owner_name {}
        impl IsTLUniAccess for $owner_name {}
        impl IsTLUniOwner for $owner_name {}
    };
    ( $( # [ $attr:meta ] )* $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $owner_name = qcell::TLCellOwner<$marker_name>;
        $crate::new_tl_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $owner_name = qcell::TLCellOwner<$marker_name>;
        $crate::new_tl_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis type $owner_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $owner_name = qcell::TLCellOwner<$marker_name>;
        $crate::new_tl_owner_type!( @finish_build => ( $owner_name [ $marker_name ] ) )
    };
}

#[macro_export]
macro_rules! new_t_cell_type {
    ( @finish_build => ( $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) ) => {
        impl<$cell_type> IsGTUniAccess for $cell_name<$cell_type> {}
        impl<$cell_type> IsTUniAccess for $cell_name<$cell_type> {}
        impl<$cell_type> IsTUniCell for $cell_name<$cell_type> {}
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<$cell_type> = qcell::TCell<$marker_name, $cell_type>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident [ $marker_name:ident ] < $cell_type:ident > ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<$cell_type> = qcell::TCell<$marker_name, $cell_type>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<T> = qcell::TCell<$marker_name, T>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < T > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<$cell_type> = qcell::TCell<$marker_name, $cell_type>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident [ $marker_name:ident ] < $cell_type:ident > ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<$cell_type> = qcell::TCell<$marker_name, $cell_type>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<T> = qcell::TCell<$marker_name, T>;
        $crate::new_t_cell_type!( @finish_build => ( $cell_name < T > [ $marker_name ] ) )
    };
}

#[macro_export]
macro_rules! new_tl_cell_type {
    ( @finish_build => ( $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) ) => {
        impl<$cell_type> IsGTUniAccess for $cell_name<$cell_type> {}
        impl<$cell_type> IsTLUniAccess for $cell_name<$cell_type> {}
        impl<$cell_type> IsTLUniCell for $cell_name<$cell_type> {}
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<$cell_type> = qcell::TLCell<$marker_name, $cell_type>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident [ $marker_name:ident ] < $cell_type:ident > ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<$cell_type> = qcell::TLCell<$marker_name, $cell_type>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $cell_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        type $cell_name<T> = qcell::TLCell<$marker_name, T>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < T > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident < $cell_type:ident > [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<$cell_type> = qcell::TLCell<$marker_name, $cell_type>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident [ $marker_name:ident ] < $cell_type:ident > ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<$cell_type> = qcell::TLCell<$marker_name, $cell_type>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < $cell_type > [ $marker_name ] ) )
    };
    ( $( # [ $attr:meta ] )* $visibilty:vis $cell_name:ident [ $marker_name:ident ] ) => {
        $( # [ $attr:meta ] )*
        $visibilty type $cell_name<T> = qcell::TLCell<$marker_name, T>;
        $crate::new_tl_cell_type!( @finish_build => ( $cell_name < T > [ $marker_name ] ) )
    };
}

#[macro_export]
macro_rules! create_t_group {
    ( $( # [ $attr:meta ] )* $visibilty:vis $owner_name:ident [ $marker_name:ident ] => $cell_name:ident $( < $cell_type:ident > )? ) => {
        $( # [ $attr:meta ] )*
        $crate::new_t_marker_type!($visibilty $marker_name);
        $( # [ $attr:meta ] )*
        $crate::new_t_owner_type!($visibilty $owner_name [ $marker_name ]);
        $( # [ $attr:meta ] )*
        $crate::new_t_cell_type!($visibilty $cell_name $( < $cell_type > )? [ $marker_name ]);
    };
    ( @for_individual => ( $visibilty:vis cell : $cell_name:ident $( < $cell_type:ident > )? [ $marker_name:ident ] ) ) => {
        $crate::new_t_cell_type!($visibilty $cell_name [ $marker_name ] $( < $cell_type > )?)
    };
    ( @for_individual => ( $visibilty:vis owner : $owner_name:ident [ $marker_name:ident ] ) ) => {
        $crate::new_t_owner_type!($visibilty $owner_name [ $marker_name ] )
    };
    {
        $( # [ $attr2:meta ] )* $visibilty2:vis marker : $marker_name:ident ,
        $( # [ $attr0:meta ] )* $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $( # [ $attr1:meta ] )* $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? $(,)?
    } => {
        $( # [ $attr2:meta ] )*
        $crate::new_t_marker_type!($visibilty2 $marker_name);
        $( # [ $attr0:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $( # [ $attr1:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
    {
        $( # [ $attr0:meta ] )* $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $( # [ $attr2:meta ] )* $visibilty2:vis marker : $marker_name:ident ,
        $( # [ $attr1:meta ] )* $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? $(,)?
    } => {
        $( # [ $attr2:meta ] )*
        $crate::new_t_marker_type!($visibilty2 $marker_name);
        $( # [ $attr0:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $( # [ $attr1:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
    {
        $( # [ $attr0:meta ] )* $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $( # [ $attr1:meta ] )* $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? ,
        $( # [ $attr2:meta ] )* $visibilty2:vis marker : $marker_name:ident $(,)?
    } => {
        $( # [ $attr2:meta ] )*
        $crate::new_t_marker_type!($visibilty2 $marker_name);
        $( # [ $attr0:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $( # [ $attr1:meta ] )*
        $crate::create_t_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
}

#[macro_export]
macro_rules! create_tl_group {
    ( $visibilty:vis $owner_name:ident [ $marker_name:ident ] => $cell_name:ident $( < $cell_type:ident > )? ) => {
        $crate::new_tl_marker_type!($visibilty $marker_name);
        $crate::new_tl_owner_type!($visibilty $owner_name [ $marker_name ]);
        $crate::new_tl_cell_type!($visibilty $cell_name $( < $cell_type > )? [ $marker_name ]);
    };
    ( @for_individual => ( $visibilty:vis cell : $cell_name:ident $( < $cell_type:ident > )? [ $marker_name:ident ] ) ) => {
        $crate::new_tl_cell_type!($visibilty $cell_name [ $marker_name ] $( < $cell_type > )?)
    };
    ( @for_individual => ( $visibilty:vis owner : $owner_name:ident [ $marker_name:ident ] ) ) => {
        $crate::new_tl_owner_type!($visibilty $owner_name [ $marker_name ] )
    };
    {
        $visibilty2:vis marker : $marker_name:ident ,
        $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? $(,)?
    } => {
        $crate::new_tl_marker_type!($visibilty2 $marker_name);
        $crate::create_tl_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $crate::create_tl_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
    {
        $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $visibilty2:vis marker : $marker_name:ident ,
        $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? $(,)?
    } => {
        $crate::new_tl_marker_type!($visibilty2 $marker_name);
        $crate::create_tl_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $crate::create_tl_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
    {
        $visibilty0:vis $key0:ident : $value0:ident $( < $cell_type0:ident > )? ,
        $visibilty1:vis $key1:ident : $value1:ident $( < $cell_type1:ident > )? ,
        $visibilty2:vis marker : $marker_name:ident $(,)?
    } => {
        $crate::new_tl_marker_type!($visibilty2 $marker_name);
        $crate::create_tl_group!( @for_individual => ( $visibilty0 $key0 : $value0 $( < $cell_type0 > )? [ $marker_name ] ) );
        $crate::create_tl_group!( @for_individual => ( $visibilty1 $key1 : $value1 $( < $cell_type1 > )? [ $marker_name ] ) );
    };
}

// Panic messages
#[inline]
#[allow(dead_code)]
fn pvt_owner_unavailable_msg() -> String {
    String::from(
        "Tried to request a private owner from a mod that cannot provide one."
    )
}

#[inline]
#[allow(dead_code)]
fn uni_owner_unavailable_msg() -> String {
    String::from(
        "Tried to request a uniform owner from a mod that cannot provide one."
    )
}

#[inline]
#[allow(dead_code)]
fn pub_owner_unavailable_msg() -> String {
    String::from(
        "Tried to request a public owner from a mod that cannot provide one."
    )
}

#[inline]
#[allow(dead_code)]
fn pvt_cell_unavailable_msg() -> String {
    String::from(
        "Tried to request a private cell from a mod that cannot provide one."
    )
}

#[inline]
#[allow(dead_code)]
fn uni_cell_unavailable_msg() -> String {
    String::from(
        "Tried to request a uniform cell from a mod that cannot provide one."
    )
}

#[inline]
#[allow(dead_code)]
fn pub_cell_unavailable_msg() -> String {
    String::from(
        "Tried to request a public cell from a mod that cannot provide one."
    )
}

#[macro_export]
macro_rules! def_cells {
    // Uniform group
    {
        @for_uniform_group => &:($( #[$attr:meta] )*):& ->
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $_marker_pub_type:ident ,
            $owner_pvt_type:ident , $_owner_pub_type:ident ,
            $cell_pvt_type:ident , $_cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $_pub_impl_type:ident
        )
    } => {
        use $crate::*;
        $( #[$attr] )*
        pub struct UniMarker ;
        $( #[$attr] )*
        pub type UniCell<T> = qcell::$cell_type<UniMarker,T> ;
        $( #[$attr] )*
        pub type UniOwner = qcell::$owner_type<UniMarker> ;

        impl IsGTUniAccess for UniMarker {}
        impl IsGTMarker for UniMarker {}
        impl $impl_type for UniMarker {}
        impl $pvt_impl_type for UniMarker {}
        impl $marker_impl_type for UniMarker {}
        impl $marker_pvt_type for UniMarker {}

        impl<T> IsGTUniAccess for UniCell<T> {}
        impl<T> $pvt_impl_type for UniCell<T> {}
        impl<T> $cell_pvt_type for UniCell<T> {}

        impl IsGTUniAccess for UniOwner {}
        impl $pvt_impl_type for UniOwner {}
        impl $owner_pvt_type for UniOwner {}

        impl<T> GetEasyUniOwner for UniCell<T> {
            type OwnerType = UniOwner;

            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }

            fn get_matching_owner_from(&self, src : & impl GetUniOwner<Self::OwnerType>) -> Self::OwnerType {
                src.get_uniform_owner()
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> CellImpl {
            UniMarker::get_cell_impl()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_private_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_uniform_access() -> bool {
            true
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_public_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_owner() -> ! {
            panic!("{}", uni_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> UniOwner {
            UniOwner::new()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> ! {
            panic!("{}", pub_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(_item: T) -> ! {
            panic!("{}", uni_cell_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(item: T) -> UniCell<T> {
            UniCell::new(item)
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(_item: T) -> ! {
            panic!("{}", pub_cell_unavailable_msg())
        }
    };

    // Access group
    {
        @for_access_group => &:($( #[$attr:meta] )*):& ->
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $owner_pub_type:ident ,
            $cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $pub_impl_type:ident
        )
    } => {
        use $crate::*;
        $( #[$attr] )*
        pub struct PubMarker;
        $( #[$attr] )*
        pub type PubCell<T> = qcell::$cell_type<self::PubMarker, T>;
        $( #[$attr] )*
        pub type PubOwner = qcell::$owner_type<self::PubMarker>;

        impl IsGTPubAccess for PubMarker {}
        impl IsGTMarker for PubMarker {}
        impl $impl_type for PubMarker {}
        impl $pub_impl_type for PubMarker {}
        impl $marker_impl_type for PubMarker {}
        impl $marker_pub_type for PubMarker {}

        impl<T> IsGTPubAccess for PubCell<T> {}
        impl<T> $pub_impl_type for PubCell<T> {}
        impl<T> $cell_pub_type for PubCell<T> {}

        impl IsGTPubAccess for PubOwner {}
        impl $pub_impl_type for PubOwner {}
        impl $owner_pub_type for PubOwner {}

        impl<T> GetEasyPubOwner for PubCell<T> {
            type OwnerType = PubOwner;

            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }

            fn get_matching_owner_from(&self, src : & impl GetPubOwner<Self::OwnerType>) -> Self::OwnerType {
                src.get_public_owner()
            }
        }
    
        $( #[$attr] )*
        pub struct PvtMarker;
        $( #[$attr] )*
        pub type PvtCell<T> = qcell::$cell_type<self::PvtMarker, T>;
        $( #[$attr] )*
        pub type PvtOwner = qcell::$owner_type<self::PvtMarker>;

        impl IsGTPvtAccess for PvtMarker {}
        impl IsGTMarker for PvtMarker {}
        impl $impl_type for PvtMarker {}
        impl $pvt_impl_type for PvtMarker {}
        impl $marker_impl_type for PvtMarker {}
        impl $marker_pvt_type for PvtMarker {}

        impl<T> IsGTPvtAccess for PvtCell<T> {}
        impl<T> $pvt_impl_type for PvtCell<T> {}
        impl<T> $cell_pvt_type for PvtCell<T> {}

        impl IsGTPvtAccess for PvtOwner {}
        impl $pvt_impl_type for PvtOwner {}
        impl $owner_pvt_type for PvtOwner {}

        impl<T> GetEasyPvtOwner for PvtCell<T> {
            type OwnerType = PvtOwner;

            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }

            fn get_matching_owner_from(&self, src : & impl GetPvtOwner<Self::OwnerType>) -> Self::OwnerType {
                src.get_private_owner()
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> CellImpl {
            PvtMarker::get_cell_impl()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_private_access() -> bool {
            true
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_uniform_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_public_access() -> bool {
            true
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_owner() -> PvtOwner {
            PvtOwner::new()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> ! {
            panic!("{}", uni_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> PubOwner {
            PubOwner::new()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(item: T) -> PvtCell<T> {
            PvtCell::new(item)
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(_item: T) -> ! {
            panic!("{}", uni_cell_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(item: T) -> PubCell<T> {
            PubCell::new(item)
        }
    };

    // Public group
    {
        @for_public_group => &:($( #[$attr:meta] )*):& ->
        (
            $cell_type:ident , $owner_type:ident ,
            $_marker_pvt_type:ident , $marker_pub_type:ident ,
            $_owner_pvt_type:ident , $owner_pub_type:ident ,
            $_cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $_pvt_impl_type:ident , $pub_impl_type:ident
        )
    } => {
        use $crate::*;
        pub struct PubMarker;
        $( #[$attr] )*
        pub type PubCell<T> = qcell::$cell_type<self::PubMarker, T>;
        $( #[$attr] )*
        pub type PubOwner = qcell::$owner_type<self::PubMarker>;

        impl IsGTPubAccess for PubMarker {}
        impl IsGTMarker for PubMarker {}
        impl $impl_type for PubMarker {}
        impl $pub_impl_type for PubMarker {}
        impl $marker_impl_type for PubMarker {}
        impl $marker_pub_type for PubMarker {}

        impl<T> IsGTPubAccess for PubCell<T> {}
        impl<T> $pub_impl_type for PubCell<T> {}
        impl<T> $cell_pub_type for PubCell<T> {}

        impl IsGTPubAccess for PubOwner {}
        impl $pub_impl_type for PubOwner {}
        impl $owner_pub_type for PubOwner {}

        impl<T> GetEasyPubOwner for PubCell<T> {
            type OwnerType = PubOwner;

            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }

            fn get_matching_owner_from(&self, src : & impl GetPubOwner<Self::OwnerType>) -> Self::OwnerType {
                src.get_public_owner()
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> CellImpl {
            PubMarker::get_cell_impl()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_private_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_uniform_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_public_access() -> bool {
            true
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_owner() -> ! {
            panic!("{}", pvt_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> ! {
            panic!("{}", uni_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> PubOwner {
            PubOwner::new()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(_item: T) -> ! {
            panic!("{}", pvt_cell_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(_item: T) -> ! {
            panic!("{}", uni_cell_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(item: T) -> PubCell<T> {
            PubCell::new(item)
        }
    };

    // Private group
    {
        @for_private_group => &:($( #[$attr:meta] )*):& ->
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $_owner_pub_type:ident ,
            $cell_pvt_type:ident , $_cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $_pub_impl_type:ident
        )
    } => {
        use $crate::*;
        $( #[$attr] )*
        pub struct PvtMarker;
        $( #[$attr] )*
        pub type PvtCell<T> = qcell::$cell_type<self::PvtMarker, T>;
        $( #[$attr] )*
        pub type PvtOwner = qcell::$owner_type<self::PvtMarker>;

        impl IsGTPvtAccess for PvtMarker {}
        impl IsGTMarker for PvtMarker {}
        impl $impl_type for PvtMarker {}
        impl $pvt_impl_type for PvtMarker {}
        impl $marker_impl_type for PvtMarker {}
        impl $marker_pvt_type for PvtMarker {}

        impl<T> IsGTPvtAccess for PvtCell<T> {}
        impl<T> $pvt_impl_type for PvtCell<T> {}
        impl<T> $cell_pvt_type for PvtCell<T> {}

        impl IsGTPvtAccess for PvtOwner {}
        impl $pvt_impl_type for PvtOwner {}
        impl $owner_pvt_type for PvtOwner {}

        impl<T> GetEasyPvtOwner for PvtCell<T> {
            type OwnerType = PvtOwner;

            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }

            fn get_matching_owner_from(&self, src : & impl GetPvtOwner<Self::OwnerType>) -> Self::OwnerType {
                src.get_private_owner()
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> CellImpl {
            PvtMarker::get_cell_impl()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_private_access() -> bool {
            true
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_uniform_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn has_public_access() -> bool {
            false
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_owner() -> PvtOwner {
            PvtOwner::new()
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> ! {
            panic!("{}", uni_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> ! {
            panic!("{}", pub_owner_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(item: T) -> PvtCell<T> {
            PvtCell::new(item)
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(_item: T) -> ! {
            panic!("{}", uni_cell_unavailable_msg())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(_item: T) -> ! {
            panic!("{}", pub_cell_unavailable_msg())
        }
    };

    // Individual evaluations -> uniform
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $owner_pub_type:ident ,
            $cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $pub_impl_type:ident
        ) , @for_uniform
    } => {
        $group_visibility mod $cell_mod_name {
            $crate::def_cells! {
                @for_uniform_group => &:$attrs:& ->
                (
                    $cell_type , $owner_type ,
                    $marker_pvt_type , $marker_pub_type ,
                    $owner_pvt_type , $owner_pub_type ,
                    $cell_pvt_type , $cell_pub_type ,
                    $impl_type , $marker_impl_type ,
                    $pvt_impl_type , $pub_impl_type
                )
            }
        }
    };

    // Individual evaluations -> access
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $owner_pub_type:ident ,
            $cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $pub_impl_type:ident
        ) , @for_access
    } => {
        $group_visibility mod $cell_mod_name {
            $crate::def_cells! {
                @for_access_group => &:$attrs:& ->
                (
                    $cell_type , $owner_type ,
                    $marker_pvt_type , $marker_pub_type ,
                    $owner_pvt_type , $owner_pub_type ,
                    $cell_pvt_type , $cell_pub_type ,
                    $impl_type , $marker_impl_type ,
                    $pvt_impl_type , $pub_impl_type
                )
            }
        }
    };

    // Individual evaluations -> public
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $owner_pub_type:ident ,
            $cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $pub_impl_type:ident
        ) , @for_public
    } => {
        $group_visibility mod $cell_mod_name {
            $crate::def_cells! {
                @for_public_group => &:$attrs:& ->
                (
                    $cell_type , $owner_type ,
                    $marker_pvt_type , $marker_pub_type ,
                    $owner_pvt_type , $owner_pub_type ,
                    $cell_pvt_type , $cell_pub_type ,
                    $impl_type , $marker_impl_type ,
                    $pvt_impl_type , $pub_impl_type
                )
            }
        }
    };

    // Individual evaluations -> private
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        (
            $cell_type:ident , $owner_type:ident ,
            $marker_pvt_type:ident , $marker_pub_type:ident ,
            $owner_pvt_type:ident , $owner_pub_type:ident ,
            $cell_pvt_type:ident , $cell_pub_type:ident ,
            $impl_type:ident , $marker_impl_type:ident ,
            $pvt_impl_type:ident , $pub_impl_type:ident
        ) , @for_private
    } => {
        $group_visibility mod $cell_mod_name {
            $crate::def_cells! {
                @for_private_group => &:$attrs:& ->
                (
                    $cell_type , $owner_type ,
                    $marker_pvt_type , $marker_pub_type ,
                    $owner_pvt_type , $owner_pub_type ,
                    $cell_pvt_type , $cell_pub_type ,
                    $impl_type , $marker_impl_type ,
                    $pvt_impl_type , $pub_impl_type
                )
            }
        }
    };

    // Interpret block of definitions
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TCellUniGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TCell , TCellOwner ,
                IsTUniMarker , IsTUniMarker ,
                IsTUniOwner , IsTUniOwner ,
                IsTUniCell , IsTUniCell ,
                IsTImpl , IsTMarker ,
                IsTUniAccess , IsTUniAccess
            ), @for_uniform
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TCellAccGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TCell , TCellOwner ,
                IsTPvtMarker , IsTPubMarker ,
                IsTPvtOwner , IsTPubOwner ,
                IsTPvtCell , IsTPubCell ,
                IsTImpl , IsTMarker ,
                IsTPvtAccess , IsTPubAccess
            ), @for_access
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TLCellUniGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TLCell , TLCellOwner ,
                IsTLUniMarker , IsTLUniMarker ,
                IsTLUniOwner , IsTLUniOwner ,
                IsTLUniCell , IsTLUniCell ,
                IsTLImpl , IsTLMarker ,
                IsTLUniAccess , IsTLUniAccess
            ), @for_uniform
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TLCellAccGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TLCell , TLCellOwner ,
                IsTLPvtMarker , IsTLPubMarker ,
                IsTLPvtOwner , IsTLPubOwner ,
                IsTLPvtCell , IsTLPubCell ,
                IsTLImpl , IsTLMarker ,
                IsTLPvtAccess , IsTLPubAccess
            ), @for_access
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TCellPubGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TCell , TCellOwner ,
                IsTPubMarker , IsTPubMarker ,
                IsTPubOwner , IsTPubOwner ,
                IsTPubCell , IsTPubCell ,
                IsTImpl , IsTMarker ,
                IsTPubAccess , IsTPubAccess
            ), @for_public
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TCellPvtGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TCell , TCellOwner ,
                IsTPvtMarker , IsTPvtMarker ,
                IsTPvtOwner , IsTPvtOwner ,
                IsTPvtCell , IsTPvtCell ,
                IsTImpl , IsTMarker ,
                IsTPvtAccess , IsTPvtAccess
            ), @for_private
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TLCellPubGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TLCell , TLCellOwner ,
                IsTLPubMarker , IsTLPubMarker ,
                IsTLPubOwner , IsTLPubOwner ,
                IsTLPubCell , IsTLPubCell ,
                IsTLImpl , IsTLMarker ,
                IsTLPubAccess , IsTLPubAccess
            ), @for_public
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident : TLCellPvtGrp
    } => {
        $crate::def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , (
                TLCell , TLCellOwner ,
                IsTLPvtMarker , IsTLPvtMarker ,
                IsTLPvtOwner , IsTLPvtOwner ,
                IsTLPvtCell , IsTLPvtCell ,
                IsTLImpl , IsTLMarker ,
                IsTLPvtAccess , IsTLPvtAccess
            ), @for_private
        }
    };

    // Cluster definitions - Check for array
    {
        @check_cluster =>
        &:$attrs:tt:&
        { $(
            $short_straw:ident
            $( : $short_category:ident )?
            $( :: $extension:tt )?
        ),+ }
    } => {
        $(
            $( $crate::def_cells! {
                @line =>
                &:$attrs:&
                [ pub mod ] $short_straw : $short_category
            } )?
            $( pub mod $short_straw {
                $crate::def_cells! {
                    @check_cluster => &:$attrs:& $extension
                }
            } )?
        )+
    };
    // Cluster definitions - Stage 0
    {
        @line =>
        &:$attrs:tt:&
        [ $group_visibility:vis mod ] $cell_mod_name:ident :: $more_cells:tt
    } => {
        $group_visibility mod $cell_mod_name {
            $crate::def_cells! {
                @check_cluster => &:$attrs:& $more_cells
            }
        }
    };

    // Line splitter
    {
        $(
            $( #[$attr:meta] )*
            [ $group_visibility:vis mod ]
            $header:ident
            $( :: $cluster:tt ; )?
            $( : $category:ident ; )? $(;)*
        )+
    } => {
        $(
            $crate::def_cells! { @line =>
                &:( $( #[$attr])* ):&
                [ $group_visibility mod ]
                $header
                $( :: $cluster )?
                $( : $category )?
            }
        )+
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: How to publish a library crate:
    //https://karthikrathinavel8.medium.com/how-to-create-and-publish-a-rust-library-85c4d25132b2

    //TODO: Add some tests to show this off
    //TODO: DOCUMENTATION, BOI

    fn got_value<T>(item: T) {
        assert!(std::option::Option::Some(item).is_some())
    }

    #[test]
    fn t_uni_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellUniGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::has_private_access(), false);
        assert_eq!(test_grp::has_uniform_access(), true);
        assert_eq!(test_grp::has_public_access(), false);

        assert_eq!(test_grp::UniMarker::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::UniMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::UniMarker::get_access_level(), CellAccessLevels::Uniform);

        assert_eq!(test_grp::UniOwner::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::UniOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::UniOwner::get_access_level(), CellAccessLevels::Uniform);

        assert_eq!(test_grp::UniCell::<i32>::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::UniCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::UniCell::<i32>::get_access_level(), CellAccessLevels::Uniform);

        let test_owner = test_grp::UniOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Uniform);

        let test_cell = test_grp::UniCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Uniform);
    }

    #[test]
    #[should_panic]
    fn t_uni_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellUniGrp;
        }
        test_grp::new_private_owner();
    }

    #[test]
    fn t_uni_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellUniGrp;
        }
        got_value(test_grp::new_uniform_owner());
    }

    #[test]
    #[should_panic]
    fn t_uni_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellUniGrp;
        }
        test_grp::new_public_owner();
    }

    #[test]
    fn tl_uni_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellUniGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::has_private_access(), false);
        assert_eq!(test_grp::has_uniform_access(), true);
        assert_eq!(test_grp::has_public_access(), false);

        assert_eq!(test_grp::UniMarker::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::UniMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::UniMarker::get_access_level(), CellAccessLevels::Uniform);

        assert_eq!(test_grp::UniOwner::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::UniOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::UniOwner::get_access_level(), CellAccessLevels::Uniform);

        assert_eq!(test_grp::UniCell::<i32>::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::UniCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::UniCell::<i32>::get_access_level(), CellAccessLevels::Uniform);

        let test_owner = test_grp::UniOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Uniform);

        let test_cell = test_grp::UniCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Uniform);
    }

    #[test]
    #[should_panic]
    fn tl_uni_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellUniGrp;
        }
        test_grp::new_private_owner();
    }

    #[test]
    fn tl_uni_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellUniGrp;
        }
        got_value(test_grp::new_uniform_owner());
    }

    #[test]
    #[should_panic]
    fn tl_uni_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellUniGrp;
        }
        test_grp::new_public_owner();
    }

    #[test]
    fn t_pvt_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPvtGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::has_private_access(), true);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), false);

        assert_eq!(test_grp::PvtMarker::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PvtMarker::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtOwner::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PvtOwner::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtCell::<i32>::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PvtCell::<i32>::get_access_level(), CellAccessLevels::Private);

        let test_owner = test_grp::PvtOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Private);

        let test_cell = test_grp::PvtCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Private);
    }

    #[test]
    fn t_pvt_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPvtGrp;
        }
        got_value(test_grp::new_private_owner());
    }

    #[test]
    #[should_panic]
    fn t_pvt_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPvtGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    #[should_panic]
    fn t_pvt_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPvtGrp;
        }
        test_grp::new_public_owner();
    }

    #[test]
    fn tl_pvt_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPvtGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::has_private_access(), true);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), false);

        assert_eq!(test_grp::PvtMarker::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PvtMarker::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtOwner::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PvtOwner::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtCell::<i32>::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PvtCell::<i32>::get_access_level(), CellAccessLevels::Private);

        let test_owner = test_grp::PvtOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Private);

        let test_cell = test_grp::PvtCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Private);
    }

    #[test]
    fn tl_pvt_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPvtGrp;
        }
        got_value(test_grp::new_private_owner());
    }

    #[test]
    #[should_panic]
    fn tl_pvt_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPvtGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    #[should_panic]
    fn tl_pvt_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPvtGrp;
        }
        test_grp::new_public_owner();
    }

    #[test]
    fn t_pub_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPubGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::has_private_access(), false);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), true);

        assert_eq!(test_grp::PubMarker::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PubMarker::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubOwner::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PubOwner::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubCell::<i32>::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PubCell::<i32>::get_access_level(), CellAccessLevels::Public);

        let test_owner = test_grp::PubOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Public);

        let test_cell = test_grp::PubCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Public);
    }

    #[test]
    #[should_panic]
    fn t_pub_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPubGrp;
        }
        test_grp::new_private_owner();
    }

    #[test]
    #[should_panic]
    fn t_pub_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPubGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    fn t_pub_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellPubGrp;
        }
        got_value(test_grp::new_public_owner());
    }

    #[test]
    fn tl_pub_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPubGrp;
        }

        assert_eq!(test_grp::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::has_private_access(), false);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), true);

        assert_eq!(test_grp::PubMarker::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PubMarker::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubOwner::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PubOwner::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubCell::<i32>::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PubCell::<i32>::get_access_level(), CellAccessLevels::Public);

        let test_owner = test_grp::PubOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Public);

        let test_cell = test_grp::PubCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Public);
    }

    #[test]
    #[should_panic]
    fn tl_pub_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPubGrp;
        }
        test_grp::new_private_owner();
    }

    #[test]
    #[should_panic]
    fn tl_pub_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPubGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    fn tl_pub_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellPubGrp;
        }
        got_value(test_grp::new_public_owner());
    }

    #[test]
    fn t_acc_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellAccGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::has_private_access(), true);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), true);

        assert_eq!(test_grp::PubMarker::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PubMarker::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubOwner::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PubOwner::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubCell::<i32>::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PubCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PubCell::<i32>::get_access_level(), CellAccessLevels::Public);

        let test_owner = test_grp::PubOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Public);

        let test_cell = test_grp::PubCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PvtMarker::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PvtMarker::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtOwner::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PvtOwner::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtCell::<i32>::get_cell_impl(), CellImpl::T);
        assert_eq!(test_grp::PvtCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PvtCell::<i32>::get_access_level(), CellAccessLevels::Private);

        let test_owner = test_grp::PvtOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Private);

        let test_cell = test_grp::PvtCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::T);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Private);
    }

    #[test]
    fn t_acc_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellAccGrp;
        }
        got_value(test_grp::new_private_owner());
    }

    #[test]
    #[should_panic]
    fn t_acc_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellAccGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    fn t_acc_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TCellAccGrp;
        }
        got_value(test_grp::new_public_owner());
    }

    #[test]
    fn tl_acc_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellAccGrp;
        }
        assert_eq!(test_grp::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::has_private_access(), true);
        assert_eq!(test_grp::has_uniform_access(), false);
        assert_eq!(test_grp::has_public_access(), true);

        assert_eq!(test_grp::PubMarker::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PubMarker::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubOwner::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PubOwner::get_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PubCell::<i32>::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PubCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PubCell::<i32>::get_access_level(), CellAccessLevels::Public);

        let test_owner = test_grp::PubOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Public);

        let test_cell = test_grp::PubCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Public);

        assert_eq!(test_grp::PvtMarker::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtMarker::get_cell_role(), CellRoles::Marker);
        assert_eq!(test_grp::PvtMarker::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtOwner::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtOwner::get_cell_role(), CellRoles::Owner);
        assert_eq!(test_grp::PvtOwner::get_access_level(), CellAccessLevels::Private);

        assert_eq!(test_grp::PvtCell::<i32>::get_cell_impl(), CellImpl::TL);
        assert_eq!(test_grp::PvtCell::<i32>::get_cell_role(), CellRoles::Cell);
        assert_eq!(test_grp::PvtCell::<i32>::get_access_level(), CellAccessLevels::Private);

        let test_owner = test_grp::PvtOwner::new();

        assert_eq!(test_owner.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_owner.get_self_cell_role(), CellRoles::Owner);
        assert_eq!(test_owner.get_self_access_level(), CellAccessLevels::Private);

        let test_cell = test_grp::PvtCell::new(1);

        assert_eq!(test_cell.get_self_cell_impl(), CellImpl::TL);
        assert_eq!(test_cell.get_self_cell_role(), CellRoles::Cell);
        assert_eq!(test_cell.get_self_access_level(), CellAccessLevels::Private);
    }

    #[test]
    fn tl_acc_pvt_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellAccGrp;
        }
        got_value(test_grp::new_private_owner());
    }

    #[test]
    #[should_panic]
    fn tl_acc_uni_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellAccGrp;
        }
        test_grp::new_uniform_owner();
    }

    #[test]
    fn tl_acc_pub_owner_declaration_accuracy_test() {
        def_cells! {
            [pub mod] test_grp: TLCellAccGrp;
        }
        got_value(test_grp::new_public_owner());
    }

    #[test]
    fn branching_modules_test() {
        def_cells! {
            [pub mod] test_grp0::{test_grp1: TLCellUniGrp};
            [pub mod] test_grp2::{
                test_grp3::{test_grp4: TLCellPvtGrp},
                test_grp5: TLCellPubGrp
            };
            [pub mod] test_grp6::{test_grp7::{test_grp8: TLCellAccGrp}};
        }
        assert!(test_grp0::test_grp1::has_uniform_access());
        assert!(test_grp2::test_grp3::test_grp4::has_private_access());
        assert!(test_grp2::test_grp5::has_public_access());
        assert_eq!(test_grp6::test_grp7::test_grp8::has_uniform_access(), false);
    }

    #[test]
    fn default_returns_test() {
        {
            create_t_group!(TestOwnerT[MarkerT] => TestCellT<T>);
            let def_cell = TestCellT::new(1);
            let def_owner = TestOwnerT::new();

            assert_eq!(MarkerT::get_cell_impl(), CellImpl::T);
            assert_eq!(TestCellT::<i32>::get_cell_impl(), CellImpl::T);
            assert_eq!(TestOwnerT::get_cell_impl(), CellImpl::T);
            assert_eq!(MarkerT::get_cell_role(), CellRoles::Marker);
            assert_eq!(TestCellT::<i32>::get_cell_role(), CellRoles::Cell);
            assert_eq!(TestOwnerT::get_cell_role(), CellRoles::Owner);
            assert_eq!(MarkerT::get_access_level(), CellAccessLevels::Uniform);
            assert_eq!(TestCellT::<i32>::get_access_level(), CellAccessLevels::Uniform);
            assert_eq!(TestOwnerT::get_access_level(), CellAccessLevels::Uniform);

            assert_eq!(def_cell.get_self_cell_impl(), CellImpl::T);
            assert_eq!(def_owner.get_self_cell_impl(), CellImpl::T);
            assert_eq!(def_cell.get_self_cell_role(), CellRoles::Cell);
            assert_eq!(def_owner.get_self_cell_role(), CellRoles::Owner);
            assert_eq!(def_cell.get_self_access_level(), CellAccessLevels::Uniform);
            assert_eq!(def_owner.get_self_access_level(), CellAccessLevels::Uniform);
        } {
            create_tl_group!(TestOwnerTL[MarkerTL] => TestCellTL<T>);
            let def_cell = TestCellTL::new(1);
            let def_owner = TestOwnerTL::new();

            assert_eq!(MarkerTL::get_cell_impl(), CellImpl::TL);
            assert_eq!(TestCellTL::<i32>::get_cell_impl(), CellImpl::TL);
            assert_eq!(TestOwnerTL::get_cell_impl(), CellImpl::TL);
            assert_eq!(MarkerTL::get_cell_role(), CellRoles::Marker);
            assert_eq!(TestCellTL::<i32>::get_cell_role(), CellRoles::Cell);
            assert_eq!(TestOwnerTL::get_cell_role(), CellRoles::Owner);
            assert_eq!(MarkerTL::get_access_level(), CellAccessLevels::Uniform);
            assert_eq!(TestCellTL::<i32>::get_access_level(), CellAccessLevels::Uniform);
            assert_eq!(TestOwnerTL::get_access_level(), CellAccessLevels::Uniform);

            assert_eq!(def_cell.get_self_cell_impl(), CellImpl::TL);
            assert_eq!(def_owner.get_self_cell_impl(), CellImpl::TL);
            assert_eq!(def_cell.get_self_cell_role(), CellRoles::Cell);
            assert_eq!(def_owner.get_self_cell_role(), CellRoles::Owner);
            assert_eq!(def_cell.get_self_access_level(), CellAccessLevels::Uniform);
            assert_eq!(def_owner.get_self_access_level(), CellAccessLevels::Uniform);
        }
    }

    #[test]
    fn individual_builder_grammar_tests() {
        new_t_marker_type!(pub TestTMarkerA);
        new_t_cell_type!(pub TestTCellA<T>[TestTMarkerA]);
        new_t_owner_type!(pub TestTOwnerA[TestTMarkerA]);

        new_tl_marker_type!(TestTLMarkerA);
        new_tl_cell_type!(TestTLCellA<T>[TestTLMarkerA]);
        new_tl_owner_type!(TestTLOwnerA[TestTLMarkerA]);

        new_t_marker_type!(TestTMarkerB);
        new_t_cell_type!(TestTCellB[TestTMarkerB]<T>);
        new_t_owner_type!(TestTOwnerB[TestTMarkerB]);

        new_tl_marker_type!(TestTLMarkerB);
        new_tl_cell_type!(TestTLCellB[TestTLMarkerB]<T>);
        new_tl_owner_type!(TestTLOwnerB[TestTLMarkerB]);

        new_t_marker_type!(TestTMarkerC);
        new_t_cell_type!(TestTCellC[TestTMarkerC]);
        new_t_owner_type!(TestTOwnerC[TestTMarkerC]);

        new_tl_marker_type!(TestTLMarkerC);
        new_tl_cell_type!(TestTLCellC[TestTLMarkerC]);
        new_tl_owner_type!(TestTLOwnerC[TestTLMarkerC]);

        create_t_group!(TestTOwnerD[TestTMarkerD] => TestTCellD<T>);
        create_t_group!(TestTOwnerE[TestTMarkerE] => TestTCellE);

        create_tl_group!(TestTLOwnerD[TestTLMarkerD] => TestTLCellD<T>);
        create_tl_group!(TestTLOwnerF[TestTLMarkerF] => TestTLCellF);

        create_t_group! {
            marker: TestTMarkerG,
            owner: TestTOwnerG,
            cell: TestTCellG<T>
        }
        create_t_group! {
            marker: TestTMarkerH,
            owner: TestTOwnerH,
            cell: TestTCellH
        }

        create_tl_group! {
            marker: TestTLMarkerG,
            owner: TestTLOwnerG,
            cell: TestTLCellG<T>
        }
        create_tl_group! {
            marker: TestTLMarkerH,
            owner: TestTLOwnerH,
            cell: TestTLCellH
        }

        create_t_group! {
            pub marker: TestTMarkerI,
            pub owner: TestTOwnerI,
            pub cell: TestTCellI<T>
        }
        create_t_group! {
            marker: TestTMarkerJ,
            cell: TestTCellJ,
            owner: TestTOwnerJ
        }

        create_tl_group! {
            owner: TestTLOwnerI,
            marker: TestTLMarkerI,
            cell: TestTLCellI<T>
        }
        create_tl_group! {
            cell: TestTLCellJ,
            marker: TestTLMarkerJ,
            owner: TestTLOwnerJ
        }
    }

    #[test]
    fn default_impl_grammar_tests() {
        def_cells! {
            [mod] test_uni_grp: TLCellUniGrp;
            [mod] test_pvt_grp: TLCellPvtGrp;
            [mod] test_pub_grp: TLCellPubGrp;
        }

        struct MyStruct {
            //
        }

        impl MyStruct {
            fn do_test(&self) {
                let owner = self.get_uniform_owner();
                got_value(&owner);

                let owner = self.get_private_owner();
                got_value(&owner);

                let owner = self.get_public_owner();
                got_value(&owner);
            }
        }

        impl_get_pvt!(MyStruct => test_pvt_grp::PvtOwner);
        impl_get_pub!(MyStruct => test_pub_grp::PubOwner);
        impl_get_uni!(MyStruct => test_uni_grp::UniOwner);

        let test_struct = MyStruct { };

        test_struct.do_test();
    }

    #[test]
    fn c_scp_grammar_tests() {
        def_cells! {
            [mod] test_uni_grp: TLCellUniGrp;
            [mod] test_uni_grp2: TLCellUniGrp;
        }

        struct MyStruct {
            test_cell: test_uni_grp::UniCell<i32>,
            test_cell2: test_uni_grp2::UniCell<i32>
        }

        impl MyStruct {
            fn do_test(&self) {
                // Mutable internal modification
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => &mut test_cont
                    ) {
                        *test_cont += 1;
                        assert_eq!(*test_cont, 6);
                    }
                }
                // Internal immutable
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => & test_cont
                    ) {
                        assert_eq!(*test_cont, 6);
                    }
                }
                // Internal immutable (checking for optional borrow)
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => test_cont
                    ) {
                        assert_eq!(*test_cont, 6);
                    }
                }
                // Mutable internal modification (deref early)
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => *mut test_cont
                    ) {
                        test_cont += 1;
                        assert_eq!(test_cont, 7);
                    }
                }
                // Internal immutable (deref early)
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => * test_cont
                    ) {
                        assert_eq!(test_cont, 6);
                    }
                }
                // Internal immutable (hard borrow)
                c_scp! {
                    use test_uni_grp::UniOwner => (
                        self.test_cell => & test_cont
                    ) {
                        assert_eq!(test_cont, &6);
                    }
                }
                // Mutable internal modification (auto; deref early)
                c_scp! {
                    use _ => (
                        self.test_cell => *mut test_cont
                    ) {
                        test_cont += 1;
                        assert_eq!(test_cont, 7);
                    }
                }
                // Internal immutable (auto; deref early)
                c_scp! {
                    use _ => (
                        self.test_cell => * test_cont
                    ) {
                        assert_eq!(test_cont, 6);
                    }
                }
                // Mutable internal modification (struct-auto; deref early)
                c_scp! {
                    use [self] => (
                        self.test_cell => *mut test_cont
                    ) {
                        test_cont += 1;
                        assert_eq!(test_cont, 7);
                    }
                }
                // Internal immutable (struct-auto; deref early)
                c_scp! {
                    use [self] => (
                        self.test_cell => * test_cont
                    ) {
                        assert_eq!(test_cont, 6);
                    }
                }

                // Start testing external scopes
                let mut outer_test_cont: i32;
                // Mutable external modification (struct-auto; deref early)
                c_scp! {
                    use [self] => (
                        self.test_cell => *out mut outer_test_cont
                    )
                }
                // Modifying in the variable's scope
                outer_test_cont += 1;
                assert_eq!(outer_test_cont, 7);
                // External immutable (struct-auto; deref early)
                c_scp! {
                    use [self] => (
                        self.test_cell => *out outer_test_cont
                    ) {
                        // self.test_cell overwrites the variable
                        // when this c_scp beings.
                        assert_eq!(outer_test_cont, 6);
                    }
                }

                // No auto-store
                c_scp! {
                    let my_owner = _ => (
                        mut self.test_cell
                    ) {
                        let careful = self.test_cell.rw(my_owner);
                        *careful += 1;
                        assert_eq!(*careful, 7);
                    }
                }
                c_scp! {
                    let my_owner = [self] => (
                        self.test_cell
                    ) {
                        let careful = self.test_cell.ro(my_owner);
                        assert_eq!(*careful, 7);
                    }
                }
                c_scp! {
                    let my_owner = [self] => (
                        mut self.test_cell
                    ) {
                        let careful = self.test_cell.rw(my_owner);
                        *careful += 1;
                        assert_eq!(*careful, 8);
                    }
                }
            }

            fn do_test2(&self, mut borrowed_owner: &mut test_uni_grp2::UniOwner) {
                // Directly borrow pre-built owner
                c_scp! {
                    let mutable_owner = &mut borrowed_owner => (
                        self.test_cell2 => mut careful
                    ) {
                        *careful += 1;
                        assert_eq!(*careful, 3);
                    }
                }
            }

            fn do_test3(&self, borrowed_owner: &test_uni_grp2::UniOwner) {
                // Directly borrow pre-built owner
                c_scp! {
                    use &borrowed_owner => (
                        self.test_cell2 => careful
                    ) {
                        assert_eq!(*careful, 3);
                    }
                }
            }

            fn do_test4(&self, mut borrowed_owner: test_uni_grp2::UniOwner) {
                // Directly borrow pre-built owner
                c_scp! {
                    use &borrowed_owner => (
                        self.test_cell2 => careful
                    ) {
                        assert_eq!(*careful, 3);
                    }
                }
                c_scp! {
                    use &mut borrowed_owner => (
                        self.test_cell2 => mut careful
                    ) {
                        *careful += 1;
                        assert_eq!(*careful, 4);
                    }
                }
            }
        }

        impl_get_uni!(MyStruct => test_uni_grp::UniOwner);
        impl_get_uni!(MyStruct => test_uni_grp2::UniOwner);

        let test_struct = MyStruct {
            test_cell: test_uni_grp::UniCell::new(5),
            test_cell2: test_uni_grp2::UniCell::new(2)
        };

        test_struct.do_test();

        {
            let mut borrowable_owner
                = test_uni_grp2::UniOwner::new();
            test_struct.do_test2(&mut borrowable_owner);
            test_struct.do_test3(&borrowable_owner);
        }
        
        {
            test_struct.do_test4(test_uni_grp2::UniOwner::new());
        }
    }
}
