pub mod xyk;

#[macro_export]
macro_rules! ensure {
    ($e:expr) => {
        match $e {
            true => (),
            false => {
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! round_up {
    ($e:expr) => {
        $e.checked_add(FIXED_ROUND_UP)
    };
}

#[macro_export]
macro_rules! to_u256 {
    ($($x:expr),+) => (
        {($(U256::from($x)),+)}
    );
}

#[macro_export]
macro_rules! to_balance {
    ($x:expr) => {
        Balance::try_from($x).ok()
    };
}

#[macro_export]
macro_rules! to_lbp_weight {
    ($x:expr) => {
        LBPWeight::try_from($x).ok()
    };
}
