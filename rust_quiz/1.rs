macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}

fn main() {
    print!(
        "{}{}{}",
        m! { return || true },
        m! { (return) || true },
        m! { {return} || true },
    );
    //fun();
}


fn fun() {
    print!(
        "{}{}{}",
        { "return || true"; 1 },
        { "(return) || true"; 1 },
        { "{return}"; 1 }<<{ "|| true"; 1 },
    )
}
