use crate::error::{ParseError, ParseOptionError, ParseOptionErrorType};
use twilight_model::application::interaction::modal::{
    ModalInteractionData, ModalInteractionDataActionRow,
};
use twilight_model::channel::message::component::ComponentType;

pub trait ModalModel: Sized {
    fn from_interaction(data: ModalInputData) -> Result<Self, ParseError>;
}

impl<T: ModalModel> ModalModel for Box<T> {
    fn from_interaction(data: ModalInputData) -> Result<Self, ParseError> {
        T::from_interaction(data).map(Box::new)
    }
}

impl ModalModel for Vec<ModalInteractionDataActionRow> {
    fn from_interaction(data: ModalInputData) -> Result<Self, ParseError> {
        Ok(data.components)
    }
}

pub trait ModalComponent: Sized {
    fn from_component(
        kind: ComponentType,
        value: Option<String>,
    ) -> Result<Self, ParseOptionErrorType>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalInputData {
    pub components: Vec<ModalInteractionDataActionRow>,
}

impl ModalInputData {
    pub fn parse_component<T>(&self, custom_id: &str) -> Result<Option<T>, ParseError>
    where
        T: ModalComponent,
    {
        let Some(component) = self
            .components
            .iter()
            .flat_map(|row| &row.components)
            .find(|component| component.custom_id == custom_id)
        else {
            return Ok(None);
        };

        match ModalComponent::from_component(component.kind, component.value.clone()) {
            Ok(value) => Ok(Some(value)),
            Err(kind) => Err(ParseError::Option(ParseOptionError {
                field: custom_id.to_string(),
                kind,
            })),
        }
    }
}

impl From<ModalInteractionData> for ModalInputData {
    fn from(data: ModalInteractionData) -> Self {
        Self {
            components: data.components,
        }
    }
}

impl ModalComponent for Option<String> {
    fn from_component(
        _kind: ComponentType,
        value: Option<String>,
    ) -> Result<Self, ParseOptionErrorType> {
        Ok(value)
    }
}
