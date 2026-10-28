use wire::Message;

fn assert_message<T: Message>() {}

/// Compile-only compatibility for the original public paths. No runtime or
/// hardware state is constructed; the values are empty placeholders.
#[test]
fn original_public_paths_still_compile() {
    assert_message::<hub_api::Said>();
    assert_message::<hub_api::frame::Said>();
    assert_message::<hub_api::Deed>();
    assert_message::<hub_api::frame::Deed>();
    assert_message::<hub_api::Window>();
    assert_message::<hub_api::frame::Window>();
    assert_message::<hub_api::frame::bond::Bond>();
    assert_message::<hub_api::frame::list::ListReq>();
    assert_message::<hub_api::frame::claim::Claim>();
    assert_message::<hub_api::frame::dma::DmaWire>();
    assert_message::<hub_api::frame::dma::DmaSaid>();
    assert_message::<hub_api::coordination::Wire>();

    let _: Option<hub_api::frame::Wire> = None;
    let _: Option<hub_api::coordination::Prepare> = None;
    let _: Option<hub_api::coordination::Release> = None;
    let _: Option<hub_api::coordination::call::Call> = None;
    let _: Option<hub_api::DeviceId> = None;
    let _: Option<hub_api::Grant> = None;
    let _: Option<hub_api::Fail> = None;
    let _: Option<hub_api::frame::vocab::Fail> = None;

    let _ = &hub_api::CHANNEL;
    let _ = &hub_api::CHANNEL_MARK;
    let _ = &hub_api::E_HUB;
    let _ = &hub_api::INVENTORY_MARK;
    let _ = &hub_api::READY;
    let _ = &hub_api::READY_MARK;
    let _ = &hub_api::binding_mark;
    let _ = &hub_api::coord_mark;
    let _ = &hub_api::window_mark;
    let _ = &hub_api::LAUNCH_MARK;
    let _ = &hub_api::code_to_fail;
    let _ = &hub_api::fail_to_code;
    let _ = &hub_api::ALIVE_MARK;
    let _ = &hub_api::BACK_MARK;
    let _ = &hub_api::BAD;
    let _ = &hub_api::BOND;
    let _ = &hub_api::BOOT;
    let _ = &hub_api::CLAIM;
    let _ = &hub_api::DEAD;
    let _ = &hub_api::DENIED;
    let _ = &hub_api::DTB;
    let _ = &hub_api::INVENTORY_MAX;
    let _ = &hub_api::LIST;
    let _ = &hub_api::LIST_MAX;
    let _ = &hub_api::OK;
    let _ = &hub_api::SERVICE_PATH;
    let _ = &hub_api::SUPERVISOR_EXTERNAL;
    let _ = &hub_api::TAKEN;
    let _ = &hub_api::UNKNOWN;
    let _ = &hub_api::inventory_slot;
    let _ = &hub_api::grant_of;
    let _ = &hub_api::INTERFACE_ID;
    let _ = &hub_api::PUBLICATIONS;
    let _ = &hub_api::REGISTRY;
    let _ = &hub_api::NAME;
    let _ = &hub_api::frame::OK;
    let _ = &hub_api::frame::dma::DMA_BACK;
    let _ = &hub_api::frame::dma::DMA_CLAIM;
    let _ = &hub_api::frame::dma::DMA_CLOSE;
    let _ = &hub_api::frame::dma::DMA_ENROLL;
    let _ = &hub_api::frame::dma::DMA_INSPECT;
    let _ = &hub_api::frame::dma::DMA_SETTLE;
    let _ = &hub_api::frame::dma::PARTY_CONTROL;
    let _ = &hub_api::frame::dma::PARTY_HUB;
    let _ = &hub_api::frame::dma::PARTY_IOMMU;
    let _ = &hub_api::frame::dma::PARTY_PCI;
    let _ = &hub_api::frame::dma::PARTY_RECOVERY;

    let _: Option<hub_api::frame::dma::Call> = None;
    let _: Option<hub_api::DmaCall> = None;
    let _: Option<hub_api::frame::inventory::Inventory> = None;
    let _: Option<hub_api::frame::inventory::InventoryItem> = None;
    let _: Option<hub_api::Inventory> = None;
    let _: Option<hub_api::InventoryItem> = None;
    let _: Option<hub_api::BindingId> = None;
    let _: Option<hub_api::BindingState> = None;
    let _: Option<hub_api::Close> = None;
    let _: Option<hub_api::CloseBinding> = None;
    let _: Option<hub_api::DmaClaim> = None;
    let _: Option<hub_api::DmaSaid> = None;
    let _: Option<hub_api::DmaWire> = None;
    let _: Option<hub_api::Enroll> = None;
    let _: Option<hub_api::Failure> = None;
    let _: Option<hub_api::Inspect> = None;
    let _: Option<hub_api::Settle> = None;

    let _: Option<hub_api::frame::dma::BindingId> = None;
    let _: Option<hub_api::frame::dma::BindingState> = None;
    let _: Option<hub_api::frame::dma::Close> = None;
    let _: Option<hub_api::frame::dma::CloseBinding> = None;
    let _: Option<hub_api::frame::dma::DmaClaim> = None;
    let _: Option<hub_api::frame::dma::Enroll> = None;
    let _: Option<hub_api::frame::dma::Failure> = None;
    let _: Option<hub_api::frame::dma::Inspect> = None;
    let _: Option<hub_api::frame::dma::Settle> = None;

    assert_eq!(hub_api::frame::WINDOW_LEN, 142);
    assert_eq!(hub_api::frame::Window::LEN, hub_api::frame::WINDOW_LEN);
    assert_eq!(hub_api::frame::inventory::INVENTORY_MAX, 64);
    assert_eq!(hub_api::frame::BOND, 1);
    assert_eq!(hub_api::frame::dma::DMA_ENROLL, 1);
    assert!(hub_api::frame::inventory::inventory_slot(0).is_some());
}
