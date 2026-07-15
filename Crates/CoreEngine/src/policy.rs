#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSensitivity {
    Public,
    Internal,
    Sensitive,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderBoundary {
    External,
    Enterprise,
    Local,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RoutingPolicy;

impl RoutingPolicy {
    pub fn allows(&self, sensitivity: DataSensitivity, boundary: ProviderBoundary) -> bool {
        match sensitivity {
            DataSensitivity::Public => true,
            DataSensitivity::Internal | DataSensitivity::Sensitive | DataSensitivity::Unknown => {
                boundary != ProviderBoundary::External
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_data_is_treated_as_sensitive() {
        let policy = RoutingPolicy;

        assert!(!policy.allows(DataSensitivity::Unknown, ProviderBoundary::External));
        assert!(policy.allows(DataSensitivity::Unknown, ProviderBoundary::Enterprise));
        assert!(policy.allows(DataSensitivity::Unknown, ProviderBoundary::Local));
    }

    #[test]
    fn public_data_can_use_any_boundary() {
        let policy = RoutingPolicy;

        assert!(policy.allows(DataSensitivity::Public, ProviderBoundary::External));
    }
}
