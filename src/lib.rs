mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const NFT_TRACKED_CONTRACT: [u8; 20] = hex!("4b79800e11fa527b01685056970d62878240ea46");
const ATTESTER_TRACKED_CONTRACT: [u8; 20] = hex!("d798a4ade873e2d447b43af34e11882efed911b1");

fn map_nft_events(blk: &eth::Block, events: &mut contract::Events) {
    events.nft_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::NftApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            owner: event.owner,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::NftApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_batch_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::BatchMetadataUpdate::match_and_decode(log) {
                        return Some(contract::NftBatchMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_from_token_id: event.u_from_token_id.to_string(),
                            u_to_token_id: event.u_to_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::MetadataUpdate::match_and_decode(log) {
                        return Some(contract::NftMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_token_id: event.u_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_role_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::RoleAdminChanged::match_and_decode(log) {
                        return Some(contract::NftRoleAdminChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin_role: Vec::from(event.new_admin_role),
                            previous_admin_role: Vec::from(event.previous_admin_role),
                            role: Vec::from(event.role),
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_role_granteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::RoleGranted::match_and_decode(log) {
                        return Some(contract::NftRoleGranted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                            role: Vec::from(event.role),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_role_revokeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::RoleRevoked::match_and_decode(log) {
                        return Some(contract::NftRoleRevoked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                            role: Vec::from(event.role),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.nft_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == NFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::nft_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::NftTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_attester_events(blk: &eth::Block, events: &mut contract::Events) {
    events.attester_attesteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::Attested::match_and_decode(log) {
                        return Some(contract::AttesterAttested {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            attester: event.attester,
                            owner: event.owner,
                            request_id: Vec::from(event.request_id),
                            url: event.url,
                        });
                    }

                    None
                })
        })
        .collect());
    events.attester_ownership_transfer_requesteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::OwnershipTransferRequested::match_and_decode(log) {
                        return Some(contract::AttesterOwnershipTransferRequested {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.attester_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::AttesterOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.attester_request_fulfilleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::RequestFulfilled::match_and_decode(log) {
                        return Some(contract::AttesterRequestFulfilled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.attester_request_sents.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::RequestSent::match_and_decode(log) {
                        return Some(contract::AttesterRequestSent {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.attester_responses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATTESTER_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::attester_contract::events::Response::match_and_decode(log) {
                        return Some(contract::AttesterResponse {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            err: event.err,
                            request_id: Vec::from(event.request_id),
                            response: event.response,
                        });
                    }

                    None
                })
        })
        .collect());
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_nft_events(&blk, &mut events);
    map_attester_events(&blk, &mut events);
    Ok(events)
}

