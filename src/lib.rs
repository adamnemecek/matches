#[macro_export]
macro_rules! matches {
    ($v: expr, $($arm: expr => { $body: expr }),*, $(_ => { $default: expr })? ) => {
        match $v {
            $(v if v == $arm => {
                $body
            }),*
            $(_ => { $default })?
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
