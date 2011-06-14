// xfail-stage0

mod test_single_attr_outer {

    #[attr = "val"]
    const int x = 10;

    #[attr = "val"]
    fn f() {}

    #[attr = "val"]
    mod mod1 {
    }

    #[attr = "val"]
    native "rust" mod rustrt { }

    #[attr = "val"]
    type t = obj { };


    #[attr = "val"]
    obj o() { }
}

mod test_multi_attr_outer {

    #[attr1 = "val"]
    #[attr2 = "val"]
    const int x = 10;

    #[attr1 = "val"]
    #[attr2 = "val"]
    fn f() {}

    #[attr1 = "val"]
    #[attr2 = "val"]
    mod mod1 {
    }

    #[attr1 = "val"]
    #[attr2 = "val"]
    native "rust" mod rustrt { }

    #[attr1 = "val"]
    #[attr2 = "val"]
    type t = obj { };


    #[attr1 = "val"]
    #[attr2 = "val"]
    obj o() { }
}

mod test_stmt_single_attr_outer {

    fn f() {

        #[attr = "val"]
        const int x = 10;

        #[attr = "val"]
        fn f() {}

        /* FIXME: Issue #493
        #[attr = "val"]
        mod mod1 {
        }

        #[attr = "val"]
        native "rust" mod rustrt {
        }
        */

        #[attr = "val"]
        type t = obj { };

        #[attr = "val"]
        obj o() { }

    }
}

mod test_stmt_multi_attr_outer {

    fn f() {

        #[attr1 = "val"]
        #[attr2 = "val"]
        const int x = 10;

        #[attr1 = "val"]
        #[attr2 = "val"]
        fn f() {}

        /* FIXME: Issue #493
        #[attr1 = "val"]
        #[attr2 = "val"]
        mod mod1 {
        }

        #[attr1 = "val"]
        #[attr2 = "val"]
        native "rust" mod rustrt {
        }
        */

        #[attr1 = "val"]
        #[attr2 = "val"]
        type t = obj { };

        #[attr1 = "val"]
        #[attr2 = "val"]
        obj o() { }

    }
}

fn main() {
}