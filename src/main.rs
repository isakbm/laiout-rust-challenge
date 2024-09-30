use reqwest::Client;
use std::io::Write;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // NOTE: this line is just to get you started ...
    let resp = client
        .get("https://dev.laiout.app/api/applicant/getChallenge")
        .send()
        .await;

    // follow instructions in challenge and submit a post request to
    // applicant/checkChallengeSolution
    //
    //
    // {
    //    "instructions": <...>,
    // }
    //
    // When successfull you will be given a `secret` from the `applicant/checkChallengeSolution`
    // endpoint. Enter this secret in the job application.
}

struct Rot13Writer<T>
where
    T: Write,
{
    inner: T,
}

impl<T> Rot13Writer<T>
where
    T: Write,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Write for Rot13Writer<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
