#[macro_export]
macro_rules! c_scp {
    // Mutable internal borrow
    {
        use $type_path:path => (
            $cell_src:expr
            => $(&)? mut $container_name:ident
            $( : $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $( { $($statement:stmt);+ ; } )? $(;)?
    } => {
        {
            let __scope_owner = &mut <$type_path> :: new();
            let $container_name $( : $(& $($lif0)? )? $container_type )?
                = $cell_src . rw( __scope_owner ) ;
            $( $($statement)+ )?
        }
    };
    // Immutable internal borrow
    {
        use $type_path:path => (
            $cell_src:expr
            => $(&)? $container_name:ident
            $( : $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $( { $($statement:stmt);+ ; } )? $(;)?
    } => {
        {
            let __scope_owner = & <$type_path> :: new();
            let $container_name $( : $(& $($lif0)? )? $container_type )?
                = $cell_src . ro( __scope_owner ) ;
            $( $($statement)+ )?
        }
    };
    // Immutable internal deref
    {
        use $type_path:path => (
            $cell_src:expr
            => * $container_name:ident
            $( : $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $( { $($statement:stmt);+ ; } )? $(;)?
    } => {
        {
            let __scope_owner = & <$type_path> :: new();
            let $container_name $( : $(& $($lif0)? )? $container_type )?
                = *($cell_src . ro( __scope_owner )) ;
            $( $($statement)+ )?
        }
    };
    // Immutable external borrow
    {
        use $type_path:path => (
            $cell_src:expr
            => out $container_name:ident
            $( as $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $({ $($statement:stmt);+ ; })? $(;)?
    } => {
        {
            let __scope_owner = & <$type_path> :: new();
            $container_name = $cell_src . ro( __scope_owner )
                $( as $(& $($lif0:lifetime)? )? $container_type:ty )? ;
            $( $($statement)+ )?
        }
    };
    // Immutable external deref
    {
        use $type_path:path => (
            $cell_src:expr
            => * out $container_name:ident
            $( as $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $({ $($statement:stmt);+ ; })? $(;)?
    } => {
        {
            let __scope_owner = & <$type_path> :: new();
            $container_name = *($cell_src . ro( __scope_owner )
                $( as $(& $($lif0:lifetime)? )? $container_type:ty )?) ;
            $( $($statement)+ )?
        }
    };
    // Immutable external hard borrow
    {
        use $type_path:path => (
            $cell_src:expr
            => & out $container_name:ident
            $( as $(& $($lif0:lifetime)? )? $container_type:ty )? )
            $({ $($statement:stmt);+ ; })? $(;)?
    } => {
        {
            let __scope_owner = & <$type_path> :: new();
            $container_name = &($cell_src . ro( __scope_owner )
                $( as $(& $($lif0:lifetime)? )? $container_type:ty )?) ;
            $( $($statement)+ )?
        }
    };
}

// These are just here to give the linter something to chew on.
pub struct TLCellUniGrp;
pub struct TCellUniGrp;
pub struct TLCellAccGrp;
pub struct TCellAccGrp;
pub struct TLCellPubGrp;
pub struct TCellPubGrp;
pub struct TLCellPvtGrp;
pub struct TCellPvtGrp;

#[macro_export]
macro_rules! def_cells {
    // Uniform group
    {
        @for_uniform_group => &:($( #[$attr:meta] )*):& ->
        ( $cell_type:ident , $owner_type:ident )
    } => {
        $( #[$attr] )*
        pub struct GrpMarker ;
        $( #[$attr] )*
        pub type GrpCell<T> = qcell::$cell_type<GrpMarker,T> ;
        $( #[$attr] )*
        pub type GrpOwner = qcell::$owner_type<GrpMarker> ;
    };

    // Access group
    {
        @for_access_group => &:($( #[$attr:meta] )*):& ->
        ( $cell_type:ident , $owner_type:ident )
    } => {
        $( #[$attr] )*
        pub struct PubMarker;
        $( #[$attr] )*
        pub type PubCell<T> = qcell::$cell_type<self::PubMarker, T>;
        $( #[$attr] )*
        pub type PubOwner = qcell::$owner_type<self::PubMarker>;
    
        $( #[$attr] )*
        pub struct PvtMarker;
        $( #[$attr] )*
        pub type PvtCell<T> = qcell::$cell_type<self::PvtMarker, T>;
        $( #[$attr] )*
        pub type PvtOwner = qcell::$owner_type<self::PvtMarker>;
    };

    // Public group
    {
        @for_public_group => &:($( #[$attr:meta] )*):& ->
        ( $cell_type:ident , $owner_type:ident )
    } => {
        pub struct PubMarker;
        $( #[$attr] )*
        pub type PubCell<T> = qcell::$cell_type<self::PubMarker, T>;
        $( #[$attr] )*
        pub type PubOwner = qcell::$owner_type<self::PubMarker>;
    };

    // Private group
    {
        @for_private_group => &:($( #[$attr:meta] )*):& ->
        ( $cell_type:ident , $owner_type:ident )
    } => {
        $( #[$attr] )*
        pub struct PvtMarker;
        $( #[$attr] )*
        pub type PvtCell<T> = qcell::$cell_type<self::PvtMarker, T>;
        $( #[$attr] )*
        pub type PvtOwner = qcell::$owner_type<self::PvtMarker>;
    };

    // Individual evaluations -> uniform
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        ( $cell_type:ident , $owner_type:ident ) , @for_uniform
    } => {
        $group_visibility mod $cell_mod_name {
            def_cells! {
                @for_uniform_group => &:$attrs:& ->
                ( $cell_type , $owner_type )
            }
        }
    };

    // Individual evaluations -> access
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        ( $cell_type:ident , $owner_type:ident ) , @for_access
    } => {
        $group_visibility mod $cell_mod_name {
            def_cells! {
                @for_access_group => &:$attrs:& ->
                ( $cell_type , $owner_type )
            }
        }
    };

    // Individual evaluations -> public
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        ( $cell_type:ident , $owner_type:ident ) , @for_public
    } => {
        $group_visibility mod $cell_mod_name {
            def_cells! {
                @for_public_group => &:$attrs:& ->
                ( $cell_type , $owner_type )
            }
        }
    };

    // Individual evaluations -> private
    {
        @for_individual => &:$attrs:tt:& ( $group_visibility:vis mod ) ->
        $cell_mod_name:ident ,
        ( $cell_type:ident , $owner_type:ident ) , @for_private
    } => {
        $group_visibility mod $cell_mod_name {
            def_cells! {
                @for_private_group => &:$attrs:& ->
                ( $cell_type , $owner_type )
            }
        }
    };

    // Interpret block of definitions
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TCellUniGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TCell , TCellOwner ), @for_uniform
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TCellAccGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TCell , TCellOwner ), @for_access
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TLCellUniGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TLCell , TLCellOwner ), @for_uniform
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TLCellAccGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TLCell , TLCellOwner ), @for_access
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TCellPubGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TCell , TCellOwner ), @for_public
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TCellPvtGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TCell , TCellOwner ), @for_private
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TLCellPubGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TLCell , TLCellOwner ), @for_public
        }
    };
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident : TLCellPvtGrp
    } => {
        def_cells! {
            @for_individual => &:$attrs:& ( $group_visibility mod ) ->
            $cell_mod_name , ( TLCell , TLCellOwner ), @for_private
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
            $( def_cells! {
                @line =>
                &:$attrs:&
                pub mod $short_straw : $short_category
            } )?
            $( pub mod $short_straw {
                def_cells! {
                    @check_cluster => &:$attrs:& $extension
                }
            } )?
        )+
    };
    // Cluster definitions - Stage 0
    {
        @line =>
        &:$attrs:tt:&
        $group_visibility:vis mod $cell_mod_name:ident :: $more_cells:tt
    } => {
        $group_visibility mod $cell_mod_name {
            def_cells! {
                @check_cluster => &:$attrs:& $more_cells
            }
        }
    };

    // Line splitter
    {
        $(
            $( #[$attr:meta] )*
            $group_visibility:vis mod
            $header:ident
            $( : $category:ident )?
            $( :: $cluster:tt )? $(;)+
        )+
    } => {
        $(
            def_cells! { @line =>
                &:( $( #[$attr])* ):&
                $group_visibility mod
                $header
                $( : $category )?
                $( :: $cluster )?
            }
        )+
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: How to publish a library crate:
    //https://karthikrathinavel8.medium.com/how-to-create-and-publish-a-rust-library-85c4d25132b2

    #[test]
    fn it_works() {
        //
    }
}
