use crate::params::Parameters;

#[derive(Debug, PartialEq)]
struct Format {
    params: Parameters,
    text: String,
    code: String,
}

impl TryFrom<&str> for Format {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let params = Parameters::try_from(value)?;
        let text = params.replace_lossy(value);

        Ok(Format {
            params,
            text,
            code: "".to_string(),
        })
    }
}
