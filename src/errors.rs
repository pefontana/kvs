use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Undefined  KvsError")]
    Error,
}
