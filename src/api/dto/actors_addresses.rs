use std::collections::HashMap;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use itertools::Itertools;
use crate::domain::actor_address::ActorsAddresses;
use utoipa::{ToSchema};

#[derive(Copy, Clone)]
enum AddressType {
    Linkedin = 30,
    Facebook = 25,
    Instagram = 29,
    CirconscriptionAddress = 2,
    Fax = 12,
    Website = 22,
    OfficialAddress = 0,
    Phone = 11,
    Twitter = 24,
    SenatorUrl = 23,
    Mail = 15,
}

impl TryFrom<i32> for AddressType {
    type Error = ();

    fn try_from(x: i32) -> Result<Self, Self::Error> {
        match x {
            x if x == AddressType::Linkedin as i32 => Ok(AddressType::Linkedin),
            x if x == AddressType::Facebook as i32 => Ok(AddressType::Facebook),
            x if x == AddressType::Instagram as i32 => Ok(AddressType::Instagram),
            x if x == AddressType::CirconscriptionAddress as i32 => Ok(AddressType::CirconscriptionAddress),
            x if x == AddressType::Fax as i32 => Ok(AddressType::Fax),
            x if x == AddressType::Website as i32 => Ok(AddressType::Website),
            x if x == AddressType::OfficialAddress as i32 => Ok(AddressType::OfficialAddress),
            x if x == AddressType::Phone as i32 => Ok(AddressType::Phone),
            x if x == AddressType::Twitter as i32 => Ok(AddressType::Twitter),
            x if x == AddressType::SenatorUrl as i32 => Ok(AddressType::SenatorUrl),
            x if x == AddressType::Mail as i32 => Ok(AddressType::Mail),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressDTO {
    pub street_number: String,
    pub street_name: String,
    pub zip_code: String,
    pub city: String,
    pub affiliate_phone_numbers: Vec<String>,
    pub affiliate_mail_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ActorsAddressesDTO {
    pub websites: Vec<String>,
    pub mail_addresses: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub addresses: Vec<AddressDTO>,
}

impl ActorsAddressesDTO {
    pub fn from_entities(addresses: &Vec<ActorsAddresses>) -> ActorsAddressesDTO {
        assert_eq!(addresses.into_iter()
                       .group_by(|a| a.actor_uid.clone())
                       .into_iter()
                       .collect::<HashMap<_, _>>().len(), 1);

        let affiliates = addresses.iter()
            .group_by(|a| a.affiliate_address.clone())
            .into_iter()
            .map(|(k, group)| (k, group.cloned().collect::<Vec<_>>()))
            .into_iter()
            .collect::<HashMap<_, _>>();

        let types = addresses.iter()
            .group_by(|a| a.address_type)
            .into_iter()
            .map(|(k, group)| (k, group.cloned().collect::<Vec<_>>()))
            .into_iter()
            .collect::<HashMap<_, _>>();


        ActorsAddressesDTO {
            websites: [AddressType::Website, AddressType::Facebook, AddressType::Instagram, AddressType::Twitter]
                .into_iter()
                .flat_map(|x| types.get(&(x as i32)).cloned().unwrap_or(Vec::new()))
                .filter_map(|x| x.address.clone())
                .collect::<Vec<_>>(),
            mail_addresses: types.get(&(AddressType::Mail as i32))
                .unwrap_or(&Vec::new())
                .into_iter()
                .filter(|x| x.affiliate_address.is_none())
                .filter_map(|x| x.address.clone())
                .collect::<Vec<_>>(),
            phone_numbers: types.get(&(AddressType::Phone as i32))
                .unwrap_or(&Vec::new())
                .into_iter()
                .filter(|x| x.affiliate_address.is_none())
                .filter_map(|x| x.phone.clone())
                .collect::<Vec<_>>(),
            addresses: [AddressType::CirconscriptionAddress, AddressType::OfficialAddress]
                .into_iter()
                .flat_map(|x| types.get(&(x as i32)).cloned().unwrap_or(Vec::new()))
                .map(|x| AddressDTO {
                    street_number: x.street_number.clone().unwrap_or_default(),
                    street_name: x.street_name.clone().unwrap_or_default(),
                    zip_code: x.zip_code.clone().unwrap_or_default(),
                    city: x.city.clone().unwrap_or_default(),
                    affiliate_phone_numbers: affiliates.get(&Some(x.uid.clone()))
                        .unwrap_or(&Vec::new())
                        .into_iter()
                        .filter_map(|x| x.phone.clone())
                        .collect::<Vec<_>>(),
                    affiliate_mail_addresses: affiliates.get(&Some(x.uid.clone()))
                        .unwrap_or(&Vec::new())
                        .into_iter()
                        .filter_map(|x|x.address.clone())
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>(),
        }
    }
}