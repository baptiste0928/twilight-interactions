use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

pub trait CreateModal: Sized {
    fn create_modal(custom_id: String) -> ModalData;
}

impl<T: CreateModal> CreateModal for Box<T> {
    fn create_modal(custom_id: String) -> ModalData {
        T::create_modal(custom_id)
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
