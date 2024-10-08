use crate::{
    context::PrivateContext, event::event_interface::EventInterface,
    encrypted_logs::payload::compute_encrypted_event_log, oracle::logs_traits::LensForEncryptedEvent,
    oracle::unsafe_rand::unsafe_rand
};
use dep::protocol_types::{address::AztecAddress, point::Point, hash::sha256_to_field};

unconstrained fn compute_unconstrained<Event, NB, MB, OB>(
    contract_address: AztecAddress,
    randomness: Field,
    ovsk_app: Field,
    ovpk: Point,
    ivpk: Point,
    event: Event
) -> ([u8; OB], Field) where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    compute(contract_address, randomness, ovsk_app, ovpk, ivpk, event)
}

fn compute<Event, NB, MB, OB>(
    contract_address: AztecAddress,
    randomness: Field,
    ovsk_app: Field,
    ovpk: Point,
    ivpk: Point,
    event: Event
) -> ([u8; OB], Field) where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    let encrypted_log: [u8; OB] = compute_encrypted_event_log(contract_address, randomness, ovsk_app, ovpk, ivpk, event);
    let log_hash = sha256_to_field(encrypted_log);
    (encrypted_log, log_hash)
}

fn emit_with_keys<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    randomness: Field,
    event: Event,
    ovpk: Point,
    ivpk: Point,
    inner_compute: fn(AztecAddress, Field, Field, Point, Point, Event) -> ([u8; OB], Field)
) where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    let contract_address: AztecAddress = context.this_address();
    let ovsk_app: Field  = context.request_ovsk_app(ovpk.hash());
    let (encrypted_log, log_hash) = inner_compute(contract_address, randomness, ovsk_app, ovpk, ivpk, event);
    context.emit_raw_event_log_with_masked_address(randomness, encrypted_log, log_hash);
}

pub fn encode_and_encrypt_event<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    ov: AztecAddress,
    iv: AztecAddress
) -> fn[(&mut PrivateContext, AztecAddress, AztecAddress)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let header = context.get_header();
        let ovpk = header.get_ovpk_m(context, ov);
        let ivpk = header.get_ivpk_m(context, iv);
        let randomness = unsafe_rand();
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute);
    }
}

pub fn encode_and_encrypt_event_unconstrained<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    ov: AztecAddress,
    iv: AztecAddress
) -> fn[(&mut PrivateContext, AztecAddress, AztecAddress)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let header = context.get_header();
        let ovpk = header.get_ovpk_m(context, ov);
        let ivpk = header.get_ivpk_m(context, iv);
        let randomness = unsafe_rand();
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute_unconstrained);
    }
}

pub fn encode_and_encrypt_event_with_randomness<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    randomness: Field,
    ov: AztecAddress,
    iv: AztecAddress
) -> fn[(&mut PrivateContext, AztecAddress, AztecAddress, Field)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let header = context.get_header();
        let ovpk = header.get_ovpk_m(context, ov);
        let ivpk = header.get_ivpk_m(context, iv);
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute);
    }
}

pub fn encode_and_encrypt_event_with_randomness_unconstrained<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    randomness: Field,
    ov: AztecAddress,
    iv: AztecAddress
) -> fn[(&mut PrivateContext, AztecAddress, AztecAddress, Field)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let header = context.get_header();
        let ovpk = header.get_ovpk_m(context, ov);
        let ivpk = header.get_ivpk_m(context, iv);
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute_unconstrained);
    }
}

pub fn encode_and_encrypt_event_with_keys<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    ovpk: Point,
    ivpk: Point
) -> fn[(&mut PrivateContext, Point, Point)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let randomness = unsafe_rand();
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute);
    }
}

pub fn encode_and_encrypt_event_with_keys_unconstrained<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    ovpk: Point,
    ivpk: Point
) -> fn[(&mut PrivateContext, Point, Point)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        let randomness = unsafe_rand();
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute_unconstrained);
    }
}

pub fn encode_and_encrypt_event_with_keys_with_randomness<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    randomness: Field,
    ovpk: Point,
    ivpk: Point
) -> fn[(&mut PrivateContext, Field, Point, Point)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute);
    }
}

pub fn encode_and_encrypt_event_with_keys_with_randomness_unconstrained<Event, NB, MB, OB>(
    context: &mut PrivateContext,
    randomness: Field,
    ovpk: Point,
    ivpk: Point
) -> fn[(&mut PrivateContext, Field, Point, Point)](Event) -> () where Event: EventInterface<NB, MB>, [u8; NB]: LensForEncryptedEvent<NB, OB> {
    | e: Event | {
        emit_with_keys(context, randomness, e, ovpk, ivpk, compute_unconstrained);
    }
}
