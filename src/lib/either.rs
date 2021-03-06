
import option;
import option::some;
import option::none;

tag t[T, U] { left(T); right(U); }

type operator[T, U] = fn(&T) -> U ;

fn either[T, U,
          V](&operator[T, V] f_left, &operator[U, V] f_right, &t[T, U] value)
   -> V {
    alt (value) {
        case (left(?l)) { f_left(l) }
        case (right(?r)) { f_right(r) }
    }
}

fn lefts[T, U](&vec[t[T, U]] eithers) -> vec[T] {
    let vec[T] result = [];
    for (t[T, U] elt in eithers) {
        alt (elt) {
            case (left(?l)) { result += [l] }
            case (_) {/* fallthrough */ }
        }
    }
    ret result;
}

fn rights[T, U](&vec[t[T, U]] eithers) -> vec[U] {
    let vec[U] result = [];
    for (t[T, U] elt in eithers) {
        alt (elt) {
            case (right(?r)) { result += [r] }
            case (_) {/* fallthrough */ }
        }
    }
    ret result;
}

fn partition[T, U](&vec[t[T, U]] eithers) -> tup(vec[T], vec[U]) {
    let vec[T] lefts = [];
    let vec[U] rights = [];
    for (t[T, U] elt in eithers) {
        alt (elt) {
            case (left(?l)) { lefts += [l] }
            case (right(?r)) { rights += [r] }
        }
    }
    ret tup(lefts, rights);
}
//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C $RBUILD 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
//
