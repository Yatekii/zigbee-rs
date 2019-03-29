pub mod state;
pub mod machine_state;
pub mod machine;
pub mod init;
pub mod serde;
pub mod nwk;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
