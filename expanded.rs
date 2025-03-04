/// This module is the source code that was used for
/// building what is found in [`./example_macro_results/index.html`].
#[allow(dead_code)]
mod example_macro_results_source_code {
    use super::*;
    pub mod example_uni_grp {
        use crate::*;
        pub struct UniMarker;
        pub type UniCell<T> = qcell::TLCell<UniMarker, T>;
        pub type UniOwner = qcell::TLCellOwner<UniMarker>;
        impl crate::IsGTUniAccess for UniMarker {}
        impl crate::IsGTMarker for UniMarker {}
        impl crate::IsTLImpl for UniMarker {}
        impl crate::IsTLUniAccess for UniMarker {}
        impl crate::IsTLMarker for UniMarker {}
        impl crate::IsTLUniMarker for UniMarker {}
        impl<T> crate::IsGTUniAccess for UniCell<T> {}
        impl<T> crate::IsTLUniAccess for UniCell<T> {}
        impl<T> crate::IsTLUniCell for UniCell<T> {}
        impl crate::IsGTUniAccess for UniOwner {}
        impl crate::IsTLUniAccess for UniOwner {}
        impl crate::IsTLUniOwner for UniOwner {}
        impl<T> crate::GetEasyUniOwner for UniCell<T> {
            type OwnerType = UniOwner;
            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }
            fn get_matching_owner_from(
                &self,
                src: &impl crate::GetUniOwner<Self::OwnerType>,
            ) -> Self::OwnerType {
                src.get_uniform_owner()
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> crate::CellImpl {
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
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> UniOwner {
            UniOwner::new()
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pub_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_cell_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(item: T) -> UniCell<T> {
            UniCell::new(item)
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pub_cell_unavailable_msg());
            }
        }
    }
    pub mod example_acc_grp {
        use crate::*;
        pub struct PubMarker;
        pub type PubCell<T> = qcell::TLCell<self::PubMarker, T>;
        pub type PubOwner = qcell::TLCellOwner<self::PubMarker>;
        impl crate::IsGTPubAccess for PubMarker {}
        impl crate::IsGTMarker for PubMarker {}
        impl crate::IsTLImpl for PubMarker {}
        impl crate::IsTLPubAccess for PubMarker {}
        impl crate::IsTLMarker for PubMarker {}
        impl crate::IsTLPubMarker for PubMarker {}
        impl<T> crate::IsGTPubAccess for PubCell<T> {}
        impl<T> crate::IsTLPubAccess for PubCell<T> {}
        impl<T> crate::IsTLPubCell for PubCell<T> {}
        impl crate::IsGTPubAccess for PubOwner {}
        impl crate::IsTLPubAccess for PubOwner {}
        impl crate::IsTLPubOwner for PubOwner {}
        impl<T> crate::GetEasyPubOwner for PubCell<T> {
            type OwnerType = PubOwner;
            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }
            fn get_matching_owner_from(
                &self,
                src: &impl crate::GetPubOwner<Self::OwnerType>,
            ) -> Self::OwnerType {
                src.get_public_owner()
            }
        }
        pub struct PvtMarker;
        pub type PvtCell<T> = qcell::TLCell<self::PvtMarker, T>;
        pub type PvtOwner = qcell::TLCellOwner<self::PvtMarker>;
        impl crate::IsGTPvtAccess for PvtMarker {}
        impl crate::IsGTMarker for PvtMarker {}
        impl crate::IsTLImpl for PvtMarker {}
        impl crate::IsTLPvtAccess for PvtMarker {}
        impl crate::IsTLMarker for PvtMarker {}
        impl crate::IsTLPvtMarker for PvtMarker {}
        impl<T> crate::IsGTPvtAccess for PvtCell<T> {}
        impl<T> crate::IsTLPvtAccess for PvtCell<T> {}
        impl<T> crate::IsTLPvtCell for PvtCell<T> {}
        impl crate::IsGTPvtAccess for PvtOwner {}
        impl crate::IsTLPvtAccess for PvtOwner {}
        impl crate::IsTLPvtOwner for PvtOwner {}
        impl<T> crate::GetEasyPvtOwner for PvtCell<T> {
            type OwnerType = PvtOwner;
            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }
            fn get_matching_owner_from(
                &self,
                src: &impl crate::GetPvtOwner<Self::OwnerType>,
            ) -> Self::OwnerType {
                src.get_private_owner()
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> crate::CellImpl {
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
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_owner_unavailable_msg());
            }
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
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_cell_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(item: T) -> PubCell<T> {
            PubCell::new(item)
        }
    }
    pub mod example_pub_grp {
        use crate::*;
        pub struct PubMarker;
        pub type PubCell<T> = qcell::TLCell<self::PubMarker, T>;
        pub type PubOwner = qcell::TLCellOwner<self::PubMarker>;
        impl crate::IsGTPubAccess for PubMarker {}
        impl crate::IsGTMarker for PubMarker {}
        impl crate::IsTLImpl for PubMarker {}
        impl crate::IsTLPubAccess for PubMarker {}
        impl crate::IsTLMarker for PubMarker {}
        impl crate::IsTLPubMarker for PubMarker {}
        impl<T> crate::IsGTPubAccess for PubCell<T> {}
        impl<T> crate::IsTLPubAccess for PubCell<T> {}
        impl<T> crate::IsTLPubCell for PubCell<T> {}
        impl crate::IsGTPubAccess for PubOwner {}
        impl crate::IsTLPubAccess for PubOwner {}
        impl crate::IsTLPubOwner for PubOwner {}
        impl<T> crate::GetEasyPubOwner for PubCell<T> {
            type OwnerType = PubOwner;
            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }
            fn get_matching_owner_from(
                &self,
                src: &impl crate::GetPubOwner<Self::OwnerType>,
            ) -> Self::OwnerType {
                src.get_public_owner()
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> crate::CellImpl {
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
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pvt_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_owner() -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> PubOwner {
            PubOwner::new()
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pvt_cell_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_cell_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(item: T) -> PubCell<T> {
            PubCell::new(item)
        }
    }
    pub mod example_pvt_grp {
        use crate::*;
        pub struct PvtMarker;
        pub type PvtCell<T> = qcell::TLCell<self::PvtMarker, T>;
        pub type PvtOwner = qcell::TLCellOwner<self::PvtMarker>;
        impl crate::IsGTPvtAccess for PvtMarker {}
        impl crate::IsGTMarker for PvtMarker {}
        impl crate::IsTLImpl for PvtMarker {}
        impl crate::IsTLPvtAccess for PvtMarker {}
        impl crate::IsTLMarker for PvtMarker {}
        impl crate::IsTLPvtMarker for PvtMarker {}
        impl<T> crate::IsGTPvtAccess for PvtCell<T> {}
        impl<T> crate::IsTLPvtAccess for PvtCell<T> {}
        impl<T> crate::IsTLPvtCell for PvtCell<T> {}
        impl crate::IsGTPvtAccess for PvtOwner {}
        impl crate::IsTLPvtAccess for PvtOwner {}
        impl crate::IsTLPvtOwner for PvtOwner {}
        impl<T> crate::GetEasyPvtOwner for PvtCell<T> {
            type OwnerType = PvtOwner;
            fn get_new_matching_owner(&self) -> Self::OwnerType {
                Self::OwnerType::new()
            }
            fn get_matching_owner_from(
                &self,
                src: &impl crate::GetPvtOwner<Self::OwnerType>,
            ) -> Self::OwnerType {
                src.get_private_owner()
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn get_cell_impl() -> crate::CellImpl {
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
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_owner() -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pub_owner_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_private_cell<T>(item: T) -> PvtCell<T> {
            PvtCell::new(item)
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_uniform_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::uni_cell_unavailable_msg());
            }
        }
        #[inline]
        #[allow(dead_code)]
        pub fn new_public_cell<T>(_item: T) -> ! {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(&crate::pub_cell_unavailable_msg());
            }
        }
    }
    struct OtherMarker;
    impl crate::IsGTMarker for OtherMarker {}
    impl crate::IsTMarker for OtherMarker {}
    impl crate::IsTImpl for OtherMarker {}
    impl crate::IsGTUniAccess for OtherMarker {}
    impl crate::IsTUniAccess for OtherMarker {}
    impl crate::IsTUniMarker for OtherMarker {}
    type OtherOwner = qcell::TCellOwner<OtherMarker>;
    impl crate::IsGTUniAccess for OtherOwner {}
    impl crate::IsTUniAccess for OtherOwner {}
    impl crate::IsTUniOwner for OtherOwner {}
    type OtherCell<T> = qcell::TCell<OtherMarker, T>;
    impl<T> crate::IsGTUniAccess for OtherCell<T> {}
    impl<T> crate::IsTUniAccess for OtherCell<T> {}
    impl<T> crate::IsTUniCell for OtherCell<T> {}
    struct ExampleStruct {
        example_uni_cell: example_uni_grp::UniCell<i32>,
        example_acc_pub_cell: example_acc_grp::PubCell<i32>,
        example_acc_pvt_cell: example_acc_grp::PvtCell<i32>,
        example_pub_cell: example_pub_grp::PubCell<i32>,
        example_pvt_cell: example_pvt_grp::PvtCell<i32>,
        example_other_cell: OtherCell<i32>,
    }
    impl ExampleStruct {
        fn new() -> Self {
            Self {
                example_uni_cell: example_uni_grp::UniCell::new(0),
                example_acc_pub_cell: example_acc_grp::PubCell::new(0),
                example_acc_pvt_cell: example_acc_grp::PvtCell::new(0),
                example_pub_cell: example_pub_grp::PubCell::new(0),
                example_pvt_cell: example_pvt_grp::PvtCell::new(0),
                example_other_cell: OtherCell::new(0),
            }
        }
        fn demonstrate_context_selection(&self) {
            let mut outer_container: u8 = 7;
            match (&outer_container, &7) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            {
                let __scope_owner = &mut (self
                    .example_uni_cell
                    .get_matching_owner_from(self));
                let example_container = self.example_uni_cell.rw(__scope_owner);
                {
                    *example_container += 1;
                    match (&*example_container, &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                }
            }
            {
                let __scope_owner = &(self
                    .example_pvt_cell
                    .get_matching_owner_from(self));
                outer_container = (*self.example_pvt_cell.ro(__scope_owner)) as u8;
                {
                    match (&outer_container, &0) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                }
            }
        }
    }
    impl crate::GetUniOwner<example_uni_grp::UniOwner> for ExampleStruct {
        #[inline]
        fn get_uniform_owner(&self) -> example_uni_grp::UniOwner {
            <example_uni_grp::UniOwner>::new()
        }
    }
    impl crate::GetPubOwner<example_acc_grp::PubOwner> for ExampleStruct {
        #[inline]
        fn get_public_owner(&self) -> example_acc_grp::PubOwner {
            <example_acc_grp::PubOwner>::new()
        }
    }
    impl crate::GetPvtOwner<example_acc_grp::PvtOwner> for ExampleStruct {
        #[inline]
        fn get_private_owner(&self) -> example_acc_grp::PvtOwner {
            <example_acc_grp::PvtOwner>::new()
        }
    }
    impl crate::GetPubOwner<example_pub_grp::PubOwner> for ExampleStruct {
        #[inline]
        fn get_public_owner(&self) -> example_pub_grp::PubOwner {
            <example_pub_grp::PubOwner>::new()
        }
    }
    impl crate::GetPvtOwner<example_pvt_grp::PvtOwner> for ExampleStruct {
        #[inline]
        fn get_private_owner(&self) -> example_pvt_grp::PvtOwner {
            <example_pvt_grp::PvtOwner>::new()
        }
    }
    fn run_demonstration() {
        let example_instance = ExampleStruct::new();
        example_instance.demonstrate_context_selection();
    }
}
