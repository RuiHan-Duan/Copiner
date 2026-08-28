use crate::policy::{DataSensitivity, ProviderBoundary, RoutingPolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderDescriptor {
    pub id: String,
    pub boundary: ProviderBoundary,
    pub enabled: bool,
}

impl ProviderDescriptor {
    pub fn new(id: impl Into<String>, boundary: ProviderBoundary) -> Self {
        Self {
            id: id.into(),
            boundary,
            enabled: true,
        }
    }
}

pub trait Provider {
    fn descriptor(&self) -> &ProviderDescriptor;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderRegistryError {
    EmptyId,
    DuplicateId(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderRouteError {
    NoAllowedProvider(DataSensitivity),
}

#[derive(Debug, Default)]
pub struct ProviderRouter {
    providers: Vec<ProviderDescriptor>,
    policy: RoutingPolicy,
}

impl ProviderRouter {
    pub fn register(
        &mut self,
        descriptor: ProviderDescriptor,
    ) -> Result<(), ProviderRegistryError> {
        if descriptor.id.trim().is_empty() {
            return Err(ProviderRegistryError::EmptyId);
        }
        if self.providers.iter().any(|item| item.id == descriptor.id) {
            return Err(ProviderRegistryError::DuplicateId(descriptor.id));
        }
        self.providers.push(descriptor);
        Ok(())
    }

    pub fn route(
        &self,
        sensitivity: DataSensitivity,
    ) -> Result<&ProviderDescriptor, ProviderRouteError> {
        self.providers
            .iter()
            .find(|provider| provider.enabled && self.policy.allows(sensitivity, provider.boundary))
            .ok_or(ProviderRouteError::NoAllowedProvider(sensitivity))
    }

    pub fn providers(&self) -> &[ProviderDescriptor] {
        &self.providers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_data_skips_external_provider() {
        let mut router = ProviderRouter::default();
        router
            .register(ProviderDescriptor::new(
                "external",
                ProviderBoundary::External,
            ))
            .expect("register external provider");
        router
            .register(ProviderDescriptor::new("local", ProviderBoundary::Local))
            .expect("register local provider");

        let selected = router
            .route(DataSensitivity::Sensitive)
            .expect("local provider should be allowed");

        assert_eq!(selected.id, "local");
    }

    #[test]
    fn sensitive_data_is_rejected_without_private_provider() {
        let mut router = ProviderRouter::default();
        router
            .register(ProviderDescriptor::new(
                "external",
                ProviderBoundary::External,
            ))
            .expect("register external provider");

        assert_eq!(
            router.route(DataSensitivity::Unknown),
            Err(ProviderRouteError::NoAllowedProvider(
                DataSensitivity::Unknown
            ))
        );
    }

    #[test]
    fn duplicate_provider_id_is_rejected() {
        let mut router = ProviderRouter::default();
        let provider = ProviderDescriptor::new("local", ProviderBoundary::Local);
        router
            .register(provider.clone())
            .expect("first registration");

        assert_eq!(
            router.register(provider),
            Err(ProviderRegistryError::DuplicateId("local".to_owned()))
        );
    }
}
