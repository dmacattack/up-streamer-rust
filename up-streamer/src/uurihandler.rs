use up_rust::UUri;

const AUTHORITY_ANY_NAME: &str = "*";
const ENTITY_ANY_INSTANCE_ANY_SERVICE: u32 = 0xFFFF_FFFF;
const ENTITY_ANY_VERSION: u32 = 0xFF;
const ENTITY_ANY_RESOURCE_ID: u32 = 0xFFFF;


pub fn uauthority_to_uuri(authority_name: &str) -> UUri {
    UUri {
        authority_name: authority_name.to_string(),
        ue_id: ENTITY_ANY_INSTANCE_ANY_SERVICE,
        ue_version_major: ENTITY_ANY_VERSION,
        resource_id: ENTITY_ANY_RESOURCE_ID,
        ..Default::default()
    }
}

pub fn any_uuri() -> UUri {
    UUri {
        authority_name: AUTHORITY_ANY_NAME.to_string(),
        ue_id: ENTITY_ANY_INSTANCE_ANY_SERVICE,
        ue_version_major: ENTITY_ANY_VERSION,
        resource_id: ENTITY_ANY_RESOURCE_ID,
        ..Default::default()
    }
}