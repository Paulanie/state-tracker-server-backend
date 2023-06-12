// @generated automatically by Diesel CLI.

diesel::table! {
    actors (uid) {
        #[max_length = 32]
        uid -> Varchar,
        #[max_length = 128]
        title -> Varchar,
        #[max_length = 256]
        surname -> Varchar,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        alpha -> Varchar,
        #[max_length = 8]
        trigram -> Nullable<Varchar>,
        birthdate -> Nullable<Date>,
        #[max_length = 512]
        birthplace -> Nullable<Varchar>,
        death_date -> Nullable<Date>,
        #[max_length = 512]
        uri_hatvp -> Nullable<Varchar>,
        profession_id -> Nullable<Int4>,
    }
}

diesel::table! {
    actors_addresses (uid) {
        #[max_length = 32]
        uid -> Varchar,
        #[max_length = 32]
        actor_uid -> Varchar,
        address_type -> Int4,
        #[max_length = 128]
        address_type_name -> Varchar,
        weight -> Nullable<Int4>,
        #[max_length = 256]
        affiliate_address -> Nullable<Varchar>,
        #[max_length = 16]
        street_number -> Varchar,
        #[max_length = 128]
        street_name -> Varchar,
        #[max_length = 8]
        zip_code -> Varchar,
        #[max_length = 128]
        city -> Varchar,
        #[max_length = 512]
        address -> Nullable<Varchar>,
        #[max_length = 32]
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    amendments (uid) {
        #[max_length = 256]
        uid -> Varchar,
        #[max_length = 256]
        examination_ref -> Varchar,
        #[max_length = 512]
        tri_amendment -> Varchar,
        #[max_length = 256]
        legislative_text_ref -> Varchar,
        delivery_date -> Date,
        publication_date -> Date,
        sort_date -> Nullable<Date>,
        #[max_length = 64]
        state -> Varchar,
        #[max_length = 64]
        sub_state -> Varchar,
        #[max_length = 256]
        representation -> Varchar,
        article99 -> Bool,
        content_summary -> Nullable<Text>,
        content_title -> Nullable<Text>,
        #[max_length = 256]
        author_type -> Varchar,
        #[max_length = 32]
        author_uid -> Varchar,
        #[max_length = 32]
        author_political_group_uid -> Varchar,
    }
}

diesel::table! {
    mandates (uid) {
        #[max_length = 32]
        uid -> Varchar,
        #[max_length = 32]
        actor_uid -> Varchar,
        #[max_length = 4]
        term_of_office -> Nullable<Varchar>,
        #[max_length = 16]
        organ_type -> Varchar,
        start_date -> Date,
        publish_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        #[max_length = 4]
        precedence -> Nullable<Varchar>,
        #[max_length = 4]
        principal_appointment -> Nullable<Varchar>,
        #[max_length = 32]
        quality -> Varchar,
        #[max_length = 32]
        organ_uid -> Varchar,
    }
}

diesel::table! {
    organs (uid) {
        #[max_length = 128]
        uid -> Varchar,
        #[sql_name = "type", max_length = 128]
        type_ -> Varchar,
        #[max_length = 512]
        label -> Varchar,
        #[max_length = 512]
        edition_label -> Varchar,
        #[max_length = 128]
        short_label -> Varchar,
        #[max_length = 64]
        abbreviation_label -> Varchar,
        vi_mo_de_start_date -> Nullable<Date>,
        vi_mo_de_end_date -> Nullable<Date>,
        vi_mo_de_approval_date -> Nullable<Date>,
        #[max_length = 128]
        parent_organ_uid -> Nullable<Varchar>,
        #[max_length = 128]
        chamber -> Nullable<Varchar>,
        #[max_length = 128]
        regime -> Varchar,
        legislature -> Int4,
        number -> Int4,
        #[max_length = 64]
        region_type -> Nullable<Varchar>,
        #[max_length = 64]
        region_label -> Nullable<Varchar>,
        #[max_length = 4]
        department_code -> Nullable<Varchar>,
        #[max_length = 64]
        department_label -> Nullable<Varchar>,
    }
}

diesel::table! {
    parliamentary_mandate_collaborators (id) {
        id -> Int4,
        #[max_length = 32]
        mandate_uid -> Varchar,
        #[max_length = 32]
        title -> Varchar,
        #[max_length = 256]
        surname -> Varchar,
        #[max_length = 256]
        name -> Varchar,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
    }
}

diesel::table! {
    parliamentary_mandates (uid) {
        #[max_length = 32]
        uid -> Varchar,
        #[max_length = 32]
        actor_uid -> Varchar,
        #[max_length = 4]
        term_of_office -> Nullable<Varchar>,
        #[max_length = 16]
        organ_type -> Varchar,
        start_date -> Nullable<Date>,
        publish_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        #[max_length = 4]
        precedence -> Nullable<Varchar>,
        #[max_length = 4]
        principal_appointment -> Nullable<Varchar>,
        #[max_length = 32]
        quality -> Varchar,
        #[max_length = 32]
        organ_uid -> Varchar,
        #[max_length = 128]
        chamber -> Nullable<Varchar>,
        #[max_length = 32]
        election_region -> Varchar,
        #[max_length = 32]
        election_region_type -> Varchar,
        #[max_length = 32]
        election_department -> Varchar,
        #[max_length = 2]
        election_department_number -> Varchar,
        election_district_number -> Int4,
        #[max_length = 64]
        election_mandate_cause -> Varchar,
        #[max_length = 32]
        election_district -> Varchar,
        mandate_start -> Date,
        #[max_length = 128]
        end_reason -> Nullable<Varchar>,
        first_election -> Int4,
        assembly_place -> Int4,
        #[max_length = 32]
        replacing_mandate_uid -> Nullable<Varchar>,
    }
}

diesel::table! {
    professions (id) {
        id -> Int4,
        #[max_length = 512]
        name -> Nullable<Varchar>,
        #[max_length = 512]
        family -> Varchar,
        #[max_length = 512]
        category -> Varchar,
    }
}

diesel::joinable!(actors -> professions (profession_id));
diesel::joinable!(actors_addresses -> actors (actor_uid));
diesel::joinable!(amendments -> actors (author_uid));
diesel::joinable!(mandates -> actors (actor_uid));
diesel::joinable!(parliamentary_mandate_collaborators -> parliamentary_mandates (mandate_uid));
diesel::joinable!(parliamentary_mandates -> actors (actor_uid));

diesel::allow_tables_to_appear_in_same_query!(
    actors,
    actors_addresses,
    amendments,
    mandates,
    organs,
    parliamentary_mandate_collaborators,
    parliamentary_mandates,
    professions,
);
