use crate::modules::armory::Armory;
use crate::modules::live_data_processor::domain_value::Unit as DomainValueUnit;
use crate::modules::live_data_processor::domain_value::{AuraApplication, Creature, HitType, SpellCast};
use crate::modules::live_data_processor::domain_value::{Event, EventType};
use crate::modules::live_data_processor::dto::UnAura;
use crate::modules::live_data_processor::dto::Unit as DtoUnit;
use crate::modules::live_data_processor::tools::server::try_parse_dispel;
use std::collections::HashMap;

#[test]
fn test_aura_application_predicate() {
    // setup dependencies
    let armory = Armory::default();

    let mut summons: HashMap<u64, u64> = HashMap::new();
    summons.insert(0xF130 << 48, 0xF130 << 48);

    // testing predicate in line 35 of dispel.rs: the following clauses exist in the predicate:
    //  a = aura_application_event_ids.len() < dispel.un_aura_amount as usize
    //  b = event.subject == target (dispel.target)
    //  c = aura_application.spell_id == dispel.target_spell_id
    // Correlated Active Clause Coverage (CACC) yields the following tests:
    //  a*b*c, !a*b*c, a*!b*c, a*b*!c

    let result_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit { is_player: false, unit_id: 0xF140000000000000 },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_c.is_ok());
    let result_a_b_c = result_a_b_c.unwrap();
    assert_eq!(result_a_b_c.0, 42);
    assert_eq!(result_a_b_c.1.len(), 1);
    assert_eq!(result_a_b_c.1[0], 42);

    let result_not_a_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit { is_player: false, unit_id: 0xF140000000000000 },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 0,
        },
        // amount of AuraApplications is left part of a:
        &[
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_not_a_b_c.is_ok());
    let result_not_a_b_c = result_not_a_b_c.unwrap();
    assert_eq!(result_not_a_b_c.0, 42);
    assert_eq!(result_not_a_b_c.1.len(), 0);

    let result_a_not_b_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit { is_player: false, unit_id: 0xF140000000000000 },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            // one event which gets accepted
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            // one event with non-matching subject creature id
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: (0xF130 << 48) + 1,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_not_b_c.is_ok());
    let result_a_not_b_c = result_a_not_b_c.unwrap();
    assert_eq!(result_a_not_b_c.0, 42);
    assert_eq!(result_a_not_b_c.1.len(), 1);
    assert_eq!(result_a_not_b_c.1[0], 42);

    let result_a_b_not_c = try_parse_dispel(
        &UnAura {
            un_aura_caster: DtoUnit { is_player: false, unit_id: 0xF140000000000000 },
            // right part of b:
            target: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            aura_caster: DtoUnit {
                is_player: false,
                unit_id: 0xF130 << 48, // creature with ID = 0
            },
            un_aura_spell_id: 42,
            // right part of c:
            target_spell_id: 42,
            // right part of a:
            un_aura_amount: 1,
        },
        // amount of AuraApplications is left part of a:
        &[
            // one event which gets accepted
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 42,
                }),
            },
            // one event with non-matching event's spell id
            Event {
                id: 42,
                timestamp: 1337,
                // left part of b:
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::AuraApplication(AuraApplication {
                    caster: DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    }),
                    stack_amount: 42,
                    // left part of c:
                    spell_id: 43,
                }),
            },
            Event {
                id: 42,
                timestamp: 1337,
                subject: DomainValueUnit::Creature(Creature {
                    creature_id: 0xF130 << 48,
                    entry: 0,
                    owner: Some(0xF130 << 48),
                }),
                event: EventType::SpellCast(SpellCast {
                    victim: Some(DomainValueUnit::Creature(Creature {
                        creature_id: 0xF130 << 48,
                        entry: 0,
                        owner: Some(0xF130 << 48),
                    })),
                    hit_type: HitType::Evade,
                    spell_id: Some(42),
                    damage: vec![],
                    heal: vec![],
                    threat: vec![],
                }),
            },
        ],
        1337,
        1337,
        &armory,
        42,
        &summons,
    );
    assert!(result_a_b_not_c.is_ok());
    let result_a_b_not_c = result_a_b_not_c.unwrap();
    assert_eq!(result_a_b_not_c.0, 42);
    assert_eq!(result_a_b_not_c.1.len(), 1);
    assert_eq!(result_a_b_not_c.1[0], 42);
}

// #[test]
// fn test_aura_application_predicate() {
//     // setup dependencies
//     let armory = Armory::default();

//     let mut summons: HashMap<u64, u64> = HashMap::new();
//     summons.insert(1, 1);
//     // test 50
// }
