use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

pub trait CreateModal: Sized {
    const CUSTOM_ID: &'static str;

    fn create_modal() -> ModalData;
}

impl<T: CreateModal> CreateModal for Box<T> {
    const CUSTOM_ID: &'static str = T::CUSTOM_ID;

    fn create_modal() -> ModalData {
        T::create_modal()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModalData {
    pub custom_id: String,
    pub title: String,
    pub components: Vec<Component>,
}

impl From<ModalData> for InteractionResponse {
    fn from(value: ModalData) -> Self {
        InteractionResponse {
            kind: InteractionResponseType::Modal,
            data: Some(InteractionResponseData {
                components: Some(value.components),
                custom_id: Some(value.custom_id),
                title: Some(value.title),
                ..Default::default()
            }),
        }
    }
}
