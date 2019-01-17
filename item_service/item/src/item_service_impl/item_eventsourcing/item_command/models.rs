use crate::item_service_impl::models::p_item::PItem;
use crate::item_service_impl::models::pitem_data::PItemData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PItemCommand {
    CreateItem(PItem),
    UpdateItem { commander: String, item_data: PItemData },
    StartAuction {user_id: String},
    UpdatePrice{price: f32},
    FinishAuction{winner: Option<String>,price: f32},
    GetItem,
}