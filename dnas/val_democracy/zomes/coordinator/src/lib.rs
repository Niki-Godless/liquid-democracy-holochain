use hdk::prelude::*;
use val_democracy_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalInput {
    pub title: String,
    pub description: String,
    pub domain: Domain,
}

#[hdk_extern]
pub fn create_proposal(input: ProposalInput) -> ExternResult<Record> {
    let proposal = Proposal {
        title: input.title,
        description: input.description,
        domain: input.domain,
        created_at: sys_time()?,
    };
    let action_hash = create_entry(&EntryTypes::Proposal(proposal.clone()))?;
    let record = get(action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Proposal".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_proposal(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
}

#[hdk_extern]
pub fn create_delegation(delegation: Delegation) -> ExternResult<Record> {
    let action_hash = create_entry(&EntryTypes::Delegation(delegation.clone()))?;
    let my_agent_pub_key = agent_info()?.agent_initial_pubkey;
    create_link(
        my_agent_pub_key.clone(),
        action_hash.clone(),
        LinkTypes::AgentToDelegations,
        (),
    )?;
    let record = get(action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Delegation".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_my_delegations(_: ()) -> ExternResult<Vec<Record>> {
    let my_agent_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(
        LinkQuery::new(my_agent_pub_key, LinkTypeFilter::single_dep(0.into())),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash, GetOptions::default())? {
                records.push(record);
            }
        }
    }
    Ok(records)
}
