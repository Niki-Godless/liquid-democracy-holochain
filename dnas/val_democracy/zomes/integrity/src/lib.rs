use hdi::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Domain {
    Air,
    Water,
    Food,
    Infrastructure,
    Health,
    Education,
    Cooperation,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub domain: Domain,
    pub created_at: Timestamp,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Delegation {
    pub delegate_to: AgentPubKey,
    pub domain: Domain,
    pub created_at: Timestamp,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Invitation {
    pub from: AgentPubKey,
    pub to_email: String,
    pub created_at: Timestamp,
}

#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Proposal(Proposal),
    Delegation(Delegation),
    Invitation(Invitation),
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentToDelegations,
}

